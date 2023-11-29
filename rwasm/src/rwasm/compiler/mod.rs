use crate::{
    arena::ArenaIndex,
    engine::{
        bytecode::{BranchOffset, Instruction, TableIdx},
        code_map::InstructionPtr,
        DropKeep,
    },
    module::{ConstExpr, DataSegment, DataSegmentKind, ElementSegmentKind, ImportName, Imported},
    rwasm::{
        binary_format::{BinaryFormat, BinaryFormatError, BinaryFormatWriter},
        compiler::drop_keep::{translate_drop_keep, DropKeepWithReturnParam},
        instruction_set::InstructionSet,
        ImportLinker,
    },
    Config,
    Engine,
    Module,
};
use alloc::{collections::BTreeMap, vec::Vec};
use core::ops::Deref;
use fluentbase_rwasm_core::common::{Pages, UntypedValue, ValueType};

mod drop_keep;

#[derive(Debug)]
pub enum CompilerError {
    ModuleError(crate::Error),
    MissingEntrypoint,
    MissingFunction,
    NotSupported(&'static str),
    OutOfBuffer,
    BinaryFormat(BinaryFormatError),
    NotSupportedImport,
    UnknownImport(ImportName),
    MemoryUsageTooBig,
    DropKeepOutOfBounds,
}

impl Into<i32> for CompilerError {
    fn into(self) -> i32 {
        match self {
            CompilerError::ModuleError(_) => -1,
            CompilerError::MissingEntrypoint => -2,
            CompilerError::MissingFunction => -3,
            CompilerError::NotSupported(_) => -4,
            CompilerError::OutOfBuffer => -5,
            CompilerError::BinaryFormat(_) => -6,
            CompilerError::NotSupportedImport => -7,
            CompilerError::UnknownImport(_) => -8,
            CompilerError::MemoryUsageTooBig => -9,
            CompilerError::DropKeepOutOfBounds => -10,
        }
    }
}

pub trait Translator {
    fn translate(&self, result: &mut InstructionSet) -> Result<(), CompilerError>;
}

pub struct Compiler<'linker> {
    engine: Engine,
    module: Module,
    // translation state
    pub(crate) code_section: InstructionSet,
    // mapping from function index to its position inside code section
    function_beginning: BTreeMap<u32, u32>,
    import_linker: Option<&'linker ImportLinker>,
    // for automatic translation
    is_translated: bool,
    injection_segments: Vec<Injection>,
    br_table_status: Option<BrTableStatus>,
    translate_func_as_inline: bool,
}

const REF_FUNC_FUNCTION_OFFSET: u32 = 0xff000000;

#[derive(Debug)]
pub struct Injection {
    pub begin: i32,
    pub end: i32,
    pub origin_len: i32,
}

#[derive(Debug)]
struct BrTableStatus {
    injection_instructions: Vec<Instruction>,
    instr_countdown: u32,
}

#[derive(Debug)]
pub enum FuncOrExport {
    Export(&'static str),
    Func(u32),
    StateRouter(Vec<FuncOrExport>, Instruction),
}

impl Default for FuncOrExport {
    fn default() -> Self {
        Self::Export("main")
    }
}

impl<'linker> Compiler<'linker> {
    pub fn new(wasm_binary: &[u8], consume_fuel: bool) -> Result<Self, CompilerError> {
        Self::new_with_linker(wasm_binary, None, consume_fuel)
    }

    pub fn new_with_linker(
        wasm_binary: &[u8],
        import_linker: Option<&'linker ImportLinker>,
        consume_fuel: bool,
    ) -> Result<Self, CompilerError> {
        let mut config = Config::default();
        config.consume_fuel(consume_fuel);
        let engine = Engine::new(&config);
        let module =
            Module::new(&engine, wasm_binary).map_err(|e| CompilerError::ModuleError(e))?;
        Ok(Compiler {
            engine,
            module,
            code_section: InstructionSet::new(),
            function_beginning: BTreeMap::new(),
            import_linker,
            is_translated: false,
            injection_segments: vec![],
            br_table_status: None,
            translate_func_as_inline: false,
        })
    }

    pub fn translate_func_as_inline(&mut self, v: bool) {
        self.translate_func_as_inline = v;
    }

    pub fn translate(
        &mut self,
        main_index: Option<FuncOrExport>,
        translate_sections: bool,
    ) -> Result<(), CompilerError> {
        if self.is_translated {
            unreachable!("already translated");
        }
        // first we must translate all sections, this is an entrypoint
        if translate_sections {
            self.translate_sections(main_index.unwrap_or_default())?;
        }
        // translate rest functions
        let total_fns = self.module.funcs.len();
        for i in 0..total_fns {
            self.translate_function(i as u32)?;
        }
        // there is no need to inject because code is already validated
        self.code_section.finalize(false);
        self.is_translated = true;
        Ok(())
    }

    fn translate_router(&self, main_index: FuncOrExport) -> Result<InstructionSet, CompilerError> {
        let mut router_opcodes = InstructionSet::new();
        let resolve_export_index = |name| -> Result<u32, CompilerError> {
            let main_index = self
                .module
                .exports
                .get(name)
                .ok_or(CompilerError::MissingEntrypoint)?
                .into_func_idx()
                .ok_or(CompilerError::MissingEntrypoint)?;
            Ok(main_index)
        };
        // find main entrypoint (it must starts with `main` keyword)
        let num_imports = self.module.imports.len_funcs as u32;
        match main_index {
            FuncOrExport::Export(name) => {
                let main_index = resolve_export_index(name)?;
                router_opcodes.op_call_internal(main_index - num_imports);
            }
            FuncOrExport::StateRouter(states, check_instr) => {
                for (state_value, state) in states.iter().enumerate() {
                    let func_index = match state {
                        FuncOrExport::Export(name) => resolve_export_index(name)?,
                        FuncOrExport::Func(index) => *index,
                        _ => unreachable!("not supported router state ({:?})", state),
                    };
                    // push current and second states on the stack
                    router_opcodes.push(check_instr);
                    router_opcodes.op_i32_const(state_value);
                    // if states are not equal then skip this call
                    router_opcodes.op_i32_eq();
                    router_opcodes.op_br_if_nez(2);
                    if func_index < num_imports {
                        router_opcodes.op_call(func_index);
                    } else {
                        router_opcodes.op_call_internal(func_index - num_imports);
                    }
                }
            }
            FuncOrExport::Func(index) => {
                router_opcodes.op_call_internal(index - num_imports);
            }
        }
        Ok(router_opcodes)
    }

    fn translate_sections(&mut self, main_index: FuncOrExport) -> Result<(), CompilerError> {
        // lets reserve 0 index and offset for sections init
        assert_eq!(self.code_section.len(), 0, "code section must be empty");
        self.function_beginning.insert(0, 0);
        // translate global section (replaces with set/get global opcodes)
        let total_globals = self.module.globals.len();
        for i in 0..total_globals {
            self.translate_global(i as u32)?;
        }
        // translate table section (replace with grow/set table opcodes)
        let total_tables = self.module.tables.len();
        for i in 0..total_tables {
            self.translate_table(i as u32)?;
        }
        // translate memory section (replace with grow/load memory opcodes)
        self.translate_memory()?;
        self.translate_data()?;
        // translate router into separate instruction set
        let router_opcodes = self.translate_router(main_index)?;
        // inject main function call with return
        let return_offset = self.code_section.len() + router_opcodes.len() + 1;
        self.code_section.op_i32_const(return_offset);
        self.code_section.extend(router_opcodes);
        self.code_section.op_return();
        self.code_section.op_unreachable();
        // remember that this is injected and shifts br/br_if offset
        self.injection_segments.push(Injection {
            begin: 0,
            end: self.code_section.len() as i32,
            origin_len: 0,
        });
        Ok(())
    }

    fn read_memory_segment(
        memory: &DataSegment,
    ) -> Result<(UntypedValue, &[u8], bool), CompilerError> {
        match memory.kind() {
            DataSegmentKind::Active(seg) => {
                let data_offset = seg
                    .offset()
                    .eval_const()
                    .ok_or(CompilerError::NotSupported("can't eval offset"))?;
                if seg.memory_index().into_u32() != 0 {
                    return Err(CompilerError::NotSupported("not zero index"));
                }
                Ok((data_offset, memory.bytes(), true))
            }
            DataSegmentKind::Passive => Ok((0.into(), memory.bytes(), false)),
        }
    }

    fn translate_memory(&mut self) -> Result<(), CompilerError> {
        for memory in self.module.memories.iter() {
            self.code_section
                .add_memory_pages(memory.initial_pages().into_inner());
        }
        Ok(())
    }

    fn translate_data(&mut self) -> Result<(), CompilerError> {
        for memory in self.module.data_segments.iter() {
            let (offset, bytes, is_active) = Self::read_memory_segment(memory)?;
            if is_active {
                self.code_section.add_memory(offset.to_bits() as u32, bytes);
            } else {
                self.code_section.add_data(bytes);
            }
        }
        Ok(())
    }

    fn translate_global(&mut self, global_index: u32) -> Result<(), CompilerError> {
        let len_imported = self.module.imports.len_globals;
        let globals = &self.module.globals[len_imported..];
        assert!(global_index < globals.len() as u32);
        let global_inits = &self.module.globals_init;
        assert!(global_index < global_inits.len() as u32);
        let global_expr = &global_inits[global_index as usize];
        if let Some(value) = global_expr.eval_const() {
            self.code_section.op_i64_const(value);
        } else if let Some(value) = global_expr.funcref() {
            self.code_section.op_ref_func(value.into_u32());
        }
        self.code_section.op_global_set(global_index);
        Ok(())
    }

    fn translate_const_expr(&self, const_expr: &ConstExpr) -> Result<UntypedValue, CompilerError> {
        let init_value = const_expr.eval_const().ok_or(CompilerError::NotSupported(
            "only static global variables supported",
        ))?;
        Ok(init_value)
    }

    fn translate_table(&mut self, table_index: u32) -> Result<(), CompilerError> {
        assert!(table_index < self.module.tables.len() as u32);
        let table = &self.module.tables[table_index as usize];
        if table.element() != ValueType::FuncRef {
            return Err(CompilerError::NotSupported(
                "only funcref type is supported for tables",
            ));
        }
        // don't use ref_func here due to the entrypoint section
        self.code_section.op_i32_const(0);
        self.code_section.op_i64_const(table.minimum() as usize);
        self.code_section.op_table_grow(table_index);
        self.code_section.op_drop();
        for (i, e) in self.module.element_segments.iter().enumerate() {
            if e.ty != ValueType::FuncRef {
                return Err(CompilerError::NotSupported(
                    "only funcref type is supported for element segments",
                ));
            }
            match &e.kind {
                ElementSegmentKind::Declared => {
                    return Err(CompilerError::NotSupported(
                        "declared mode for element segments is not supported",
                    ))
                }
                ElementSegmentKind::Passive => {
                    for (_, item) in e.items.items().iter().enumerate() {
                        if let Some(value) = item.funcref() {
                            self.code_section.op_i32_const(value.into_u32());
                            self.code_section.op_elem_store(i as u32);
                        }
                    }
                }
                ElementSegmentKind::Active(aes) => {
                    if aes.table_index().into_u32() != table_index {
                        continue;
                    }
                    let dest_offset = self.translate_const_expr(aes.offset())?;
                    for (index, item) in e.items.items().iter().enumerate() {
                        self.code_section
                            .op_i32_const(dest_offset.as_u32() + index as u32);
                        if let Some(value) = item.eval_const() {
                            self.code_section.op_i64_const(value);
                        } else if let Some(value) = item.funcref() {
                            self.code_section.op_ref_func(value.into_u32());
                        }
                        self.code_section.op_table_set(table_index);
                    }
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn swap_stack_parameters(&mut self, param_num: u32) {
        for i in (0..param_num).rev() {
            let depth = i + 1;
            self.code_section.op_local_get(depth + 1);
            self.code_section.op_local_get(2);
            self.code_section.op_local_set(depth + 2);
            self.code_section.op_local_set(1);
        }
    }

    fn translate_function(&mut self, fn_index: u32) -> Result<(), CompilerError> {
        let import_len = self.module.imports.len_funcs;
        // don't translate import functions because we can't translate them
        if fn_index < import_len as u32 {
            return Ok(());
        }
        let fn_index = fn_index - import_len as u32;

        let func_type = self.module.funcs[fn_index as usize + import_len];
        let func_type = self.engine.resolve_func_type(&func_type, Clone::clone);
        let num_inputs = func_type.params();
        let beginning_offset = self.code_section.len();

        if !self.translate_func_as_inline {
            self.swap_stack_parameters(num_inputs.len() as u32);
        }

        let func_body = self
            .module
            .compiled_funcs
            .get(fn_index as usize)
            .ok_or(CompilerError::MissingFunction)?;

        // reserve stack for locals
        let len_locals = self.engine.num_locals(*func_body);
        (0..len_locals).for_each(|_| {
            self.code_section.op_i32_const(0);
        });
        // translate instructions
        let (mut instr_ptr, instr_end) = self.engine.instr_ptr(*func_body);
        while instr_ptr != instr_end {
            self.translate_opcode(&mut instr_ptr, 0)?;
        }
        if !self.translate_func_as_inline {
            self.code_section.op_unreachable();
        }
        // remember function offset in the mapping (+1 because 0 is reserved for sections init)
        self.function_beginning
            .insert(fn_index + 1, beginning_offset);
        Ok(())
    }

    fn extract_drop_keep(instr_ptr: &mut InstructionPtr) -> DropKeep {
        instr_ptr.add(1);
        let next_instr = instr_ptr.get();
        match next_instr {
            Instruction::Return(drop_keep) => *drop_keep,
            _ => unreachable!("incorrect instr after break adjust ({:?})", *next_instr),
        }
    }

    fn extract_table(instr_ptr: &mut InstructionPtr) -> TableIdx {
        instr_ptr.add(1);
        let next_instr = instr_ptr.get();
        match next_instr {
            Instruction::TableGet(table_idx) => *table_idx,
            _ => unreachable!("incorrect instr after break adjust ({:?})", *next_instr),
        }
    }

    fn translate_opcode(
        &mut self,
        instr_ptr: &mut InstructionPtr,
        return_ptr_offset: usize,
    ) -> Result<(), CompilerError> {
        use Instruction as WI;
        let injection_begin = self.code_section.len();
        let mut opcode_count = 1;
        let instr = *instr_ptr.get();
        match instr {
            WI::BrAdjust(branch_offset) => {
                opcode_count += 1;
                if let Some(mut br_table_status) = self.br_table_status.take() {
                    let drop_keep = Self::extract_drop_keep(instr_ptr);
                    let injection_begin = br_table_status.instr_countdown as i32
                        + br_table_status.injection_instructions.len() as i32;
                    let mut drop_keep_ixs = translate_drop_keep(drop_keep)?;
                    self.code_section.op_br(BranchOffset::from(injection_begin));
                    self.code_section.op_return();
                    br_table_status.instr_countdown -= 2;

                    br_table_status
                        .injection_instructions
                        .append(&mut drop_keep_ixs);
                    br_table_status
                        .injection_instructions
                        .push(WI::Br(BranchOffset::from(
                            branch_offset.to_i32() - br_table_status.instr_countdown as i32,
                        )));
                    br_table_status
                        .injection_instructions
                        .push(WI::Return(DropKeep::none()));

                    if br_table_status.instr_countdown == 0 {
                        let injection_len = br_table_status.injection_instructions.len();
                        for i in 0..injection_len {
                            if let Some(offset) =
                                br_table_status.injection_instructions[i].get_jump_offset()
                            {
                                br_table_status.injection_instructions[i].update_branch_offset(
                                    BranchOffset::from(
                                        offset.to_i32() + injection_len as i32 - i as i32 - 2,
                                    ),
                                );
                            }
                        }
                        self.code_section
                            .instr
                            .append(&mut br_table_status.injection_instructions);
                        self.br_table_status = None;
                    } else {
                        self.br_table_status = Some(br_table_status);
                    }
                } else {
                    Self::extract_drop_keep(instr_ptr).translate(&mut self.code_section)?;
                    self.code_section.op_br(branch_offset);
                    self.code_section.op_return();
                }
            }
            // WI::BrIfNez(branch_offset) => {
            //     let jump_dest = (offset as i32 + branch_offset.to_i32()) as u32;
            // }
            WI::BrAdjustIfNez(branch_offset) => {
                opcode_count += 1;
                let br_if_offset = self.code_section.len();
                self.code_section.op_br_if_eqz(0);
                Self::extract_drop_keep(instr_ptr).translate(&mut self.code_section)?;
                let drop_keep_len = self.code_section.len() - br_if_offset + 1;
                self.code_section
                    .get_mut(br_if_offset as usize)
                    .unwrap()
                    .update_branch_offset(BranchOffset::from(1 + drop_keep_len as i32));
                self.code_section.op_br(branch_offset);
                self.code_section.op_return();
            }
            WI::ReturnCallInternal(func_idx) => {
                opcode_count += 1;
                Self::extract_drop_keep(instr_ptr).translate(&mut self.code_section)?;
                let fn_index = func_idx.into_usize() as u32;
                self.code_section.op_return_call_internal(fn_index);
                self.code_section.op_return();
            }
            WI::ReturnCall(_func) => {
                // Self::extract_drop_keep(instr_ptr).translate(&mut self.code_section)?;
                // self.code_section.op_call(func);
                // self.code_section.op_return();
                unreachable!("wait, should it call translate host call?");
            }
            WI::ReturnCallIndirect(_) => {
                // Self::extract_drop_keep(instr_ptr).translate(&mut self.code_section)?;
                // let table_idx = Self::extract_table(instr_ptr);
                // self.code_section.op_return_call_indirect(table_idx);
                // self.code_section.op_return();
                unreachable!("check this")
            }
            WI::Return(drop_keep) => {
                DropKeepWithReturnParam(drop_keep).translate(&mut self.code_section)?;
                if !self.translate_func_as_inline {
                    self.code_section.op_br_indirect(0);
                }
            }
            WI::ReturnIfNez(drop_keep) => {
                let br_if_offset = self.code_section.len();
                self.code_section.op_br_if_eqz(0);
                drop_keep.translate(&mut self.code_section)?;
                let drop_keep_len = self.code_section.len() - br_if_offset;
                self.code_section
                    .get_mut(br_if_offset as usize)
                    .unwrap()
                    .update_branch_offset(BranchOffset::from(1 + drop_keep_len as i32));
                self.code_section.op_return_if_nez();
            }
            WI::CallInternal(func_idx) => {
                let target = self.code_section.len() + 2;
                // we use this constant to remember ref func offset w/o moving function indices
                // self.function_beginning
                //     .insert(REF_FUNC_FUNCTION_OFFSET + target, target);
                // self.code_section
                //     .op_ref_func(REF_FUNC_FUNCTION_OFFSET + target - 1);
                self.code_section.op_i32_const(target);
                let fn_index = func_idx.into_usize() as u32;
                self.code_section.op_call_internal(fn_index);
                // self.code_section.op_drop();
            }
            WI::CallIndirect(_) => {
                let table_idx = Self::extract_table(instr_ptr);
                opcode_count += 1;
                let return_target = self.code_section.len() + 4;
                // replace elem index with func offset
                self.code_section.op_table_get(table_idx);
                self.code_section.op_i32_const(return_target);
                // copy func offset on the top of the stack
                self.code_section.op_local_get(2);
                // jump to the table's func offset
                self.code_section.op_br_indirect(return_ptr_offset as i32);
                // drop remaining func offset from the stack (we copied it before)
                self.code_section.op_drop();
            }
            WI::Call(func_idx) => {
                self.translate_host_call(func_idx.to_u32())?;
            }
            WI::ConstRef(const_ref) => {
                let resolved_const = self.engine.resolve_const(const_ref).unwrap();
                self.code_section.op_i64_const(resolved_const);
            }
            WI::BrTable(target) => {
                self.br_table_status = Some(BrTableStatus {
                    injection_instructions: vec![],
                    instr_countdown: target.to_usize() as u32 * 2,
                });
                self.code_section.push(*instr_ptr.get());
            }
            WI::MemoryGrow => {
                assert!(!self.module.memories.is_empty(), "memory must be provided");
                let max_pages = self.module.memories[0]
                    .maximum_pages()
                    .unwrap_or(Pages::max())
                    .into_inner();
                self.code_section.op_local_get(1);
                self.code_section.op_memory_size();
                self.code_section.op_i32_add();
                self.code_section.op_i32_const(max_pages);
                self.code_section.op_i32_ge_s();
                self.code_section.op_br_if_nez(3);
                self.code_section.op_drop();
                self.code_section.op_i32_const(u32::MAX);
                self.code_section.op_br(2);
                self.code_section.op_memory_grow();
            }
            // WI::LocalGet(local_depth) => {
            //     self.code_section
            //         .op_local_get(local_depth.to_usize() as u32 + 1);
            // }
            // WI::LocalSet(local_depth) => {
            //     self.code_section
            //         .op_local_set(local_depth.to_usize() as u32 + 1);
            // }
            // WI::LocalTee(local_depth) => {
            //     self.code_section
            //         .op_local_tee(local_depth.to_usize() as u32 + 1);
            // }
            _ => {
                self.code_section.push(*instr_ptr.get());
            }
        };
        let injection_end = self.code_section.len();
        if injection_end - injection_begin > opcode_count as u32 {
            self.injection_segments.push(Injection {
                begin: injection_begin as i32,
                end: injection_end as i32,
                origin_len: opcode_count,
            });
        }

        instr_ptr.add(1);
        Ok(())
    }

    fn resolve_host_call(&mut self, fn_index: u32) -> Result<(u32, u32), CompilerError> {
        let imports = self.module.imports.items.deref();
        if fn_index >= imports.len() as u32 {
            return Err(CompilerError::NotSupportedImport);
        }
        let imported = &imports[fn_index as usize];
        let import_name = match imported {
            Imported::Func(import_name) => import_name,
            _ => return Err(CompilerError::NotSupportedImport),
        };
        let import_index_and_fuel_amount = self
            .import_linker
            .ok_or(CompilerError::UnknownImport(import_name.clone()))?
            .index_mapping()
            .get(import_name)
            .ok_or(CompilerError::UnknownImport(import_name.clone()))?;
        Ok(*import_index_and_fuel_amount)
    }

    fn translate_host_call(&mut self, fn_index: u32) -> Result<(), CompilerError> {
        let (import_index, fuel_amount) = self.resolve_host_call(fn_index)?;
        self.code_section.op_consume_fuel(fuel_amount);
        self.code_section.op_call(import_index);
        Ok(())
    }

    pub fn finalize(
        &mut self,
        main_index: Option<FuncOrExport>,
        translate_sections: bool,
    ) -> Result<Vec<u8>, CompilerError> {
        if !self.is_translated {
            self.translate(main_index, translate_sections)?;
        }
        let bytecode = &mut self.code_section;

        let mut i = 0;
        while i < bytecode.len() as usize {
            match bytecode.instr[i] {
                Instruction::CallInternal(func) => {
                    let func_idx = func.to_u32() + 1;
                    bytecode.instr[i] = Instruction::Br(BranchOffset::from(
                        self.function_beginning[&func_idx] as i32 - i as i32,
                    ));
                }
                Instruction::Br(offset)
                | Instruction::BrIfNez(offset)
                | Instruction::BrAdjust(offset)
                | Instruction::BrAdjustIfNez(offset)
                | Instruction::BrIfEqz(offset) => {
                    let mut offset = offset.to_i32();
                    let start = i as i32;
                    let mut target = start + offset;
                    if offset > 0 {
                        for injection in &self.injection_segments {
                            if injection.begin < target && start < injection.begin {
                                offset += injection.end - injection.begin - injection.origin_len;
                                target += injection.end - injection.begin - injection.origin_len;
                            }
                        }
                    } else {
                        for injection in self.injection_segments.iter().rev() {
                            if injection.end < start && target < injection.end {
                                offset -= injection.end - injection.begin - injection.origin_len;
                                target -= injection.end - injection.begin - injection.origin_len;
                            }
                        }
                    };
                    bytecode.instr[i].update_branch_offset(BranchOffset::from(offset));
                }
                Instruction::BrTable(target) => {
                    i += target.to_usize() * 2;
                }
                _ => {}
            };
            i += 1;
        }

        let mut states: Vec<(u32, u32, Vec<u8>)> = Vec::new();
        let mut buffer_offset = 0u32;
        for code in bytecode.instr.iter() {
            let mut buffer: [u8; 100] = [0; 100];
            let mut binary_writer = BinaryFormatWriter::new(&mut buffer[..]);
            code.write_binary(&mut binary_writer)
                .map_err(|e| CompilerError::BinaryFormat(e))?;
            let buffer = binary_writer.to_vec();
            let buffer_size = buffer.len() as u32;
            states.push((buffer_offset, buffer_size, buffer));
            buffer_offset += buffer_size;
        }

        for (i, code) in bytecode.instr.iter().enumerate() {
            let mut code = code.clone();
            let mut affected = false;
            match code {
                Instruction::CallInternal(func) | Instruction::ReturnCallInternal(func) => {
                    let func_idx = func.to_u32() + 1;
                    let func_offset = self
                        .function_beginning
                        .get(&func_idx)
                        .ok_or(CompilerError::MissingFunction)?;
                    let state = &states[*func_offset as usize];
                    code.update_call_index(state.0);
                    affected = true;
                }
                Instruction::RefFunc(func_idx) => {
                    let func_idx = func_idx.to_u32() + 1;
                    let imports = self.module.imports.items.deref();
                    // if ref func refers to host call
                    if func_idx < imports.len() as u32 {
                        panic!("this is not supported right now, no ref func for host calls")
                        // let import_index = self.resolve_host_call(func_idx.to_u32())?;
                        // code.update_call_index(import_index);
                        // affected = true;
                    } else {
                        let func_offset = self
                            .function_beginning
                            .get(&func_idx)
                            .ok_or(CompilerError::MissingFunction)?;
                        let state = &states[*func_offset as usize];
                        code.update_call_index(state.0);
                        affected = true;
                    }
                }
                _ => {}
            };
            if let Some(jump_offset) = code.get_jump_offset() {
                let jump_label = (jump_offset.to_i32() + i as i32) as usize;
                let target_state = states.get(jump_label).ok_or(CompilerError::OutOfBuffer)?;
                code.update_branch_offset(BranchOffset::from(target_state.0 as i32));
                affected = true;
            }
            if affected {
                let current_state = states.get_mut(i).ok_or(CompilerError::OutOfBuffer)?;
                current_state.2.clear();
                code.write_binary_to_vec(&mut current_state.2)
                    .map_err(|e| CompilerError::BinaryFormat(e))?;
            }
        }

        let res = states.iter().fold(Vec::new(), |mut res, state| {
            res.extend(&state.2);
            res
        });
        Ok(res)
    }
}
