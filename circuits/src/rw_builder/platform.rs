use crate::{
    exec_step::{ExecStep, GadgetError},
    rw_builder::{
        copy_row::{CopyRow, CopyTableTag},
        opcode::build_stack_read_rw_ops,
        rw_row::RwRow,
    },
};

pub fn build_sys_halt_rw_ops(step: &mut ExecStep) -> Result<(), GadgetError> {
    build_stack_read_rw_ops(step, 0)?;
    Ok(())
}

pub fn build_sys_read_rw_ops(step: &mut ExecStep) -> Result<(), GadgetError> {
    build_stack_read_rw_ops(step, 0)?;
    build_stack_read_rw_ops(step, 1)?;
    build_stack_read_rw_ops(step, 2)?;
    // read 3 input params from the stack
    let length = step.curr_nth_stack_value(0)?;
    let offset = step.curr_nth_stack_value(1)?;
    let target = step.curr_nth_stack_value(2)?;
    debug_assert_eq!(
        step.next_trace.clone().unwrap().memory_changes[0].offset,
        target.as_u32()
    );
    debug_assert_eq!(
        step.next_trace.clone().unwrap().memory_changes[0].len,
        length.as_u32()
    );
    let data = step.next_trace.clone().unwrap().memory_changes[0]
        .data
        .clone();
    let copy_rw_counter = step.next_rw_counter();
    // write result to the memory
    data.iter().enumerate().for_each(|(i, value)| {
        step.rw_rows.push(RwRow::Memory {
            rw_counter: step.next_rw_counter(),
            is_write: true,
            call_id: step.call_id,
            memory_address: target.as_u64() + i as u64,
            value: *value,
            signed: false,
        });
    });
    // create copy row
    step.copy_rows.push(CopyRow {
        tag: CopyTableTag::ReadInput,
        from_address: offset.as_u32(),
        to_address: target.as_u32(),
        length: length.as_u32(),
        rw_counter: copy_rw_counter,
        data: data.iter().map(|v| *v as u32).collect(),
    });
    Ok(())
}

pub fn build_sys_write_rw_ops(step: &mut ExecStep) -> Result<(), GadgetError> {
    build_stack_read_rw_ops(step, 0)?;
    build_stack_read_rw_ops(step, 1)?;
    // read 3 input params from the stack
    let length = step.curr_nth_stack_value(0)?;
    let target = step.curr_nth_stack_value(1)?;
    let mut data = vec![0; length.as_u32() as usize];
    step.curr_read_memory(target.as_u64(), data.as_mut_ptr(), length.as_u32())?;
    let copy_rw_counter = step.next_rw_counter();
    // read result to the memory
    data.iter().enumerate().for_each(|(i, value)| {
        step.rw_rows.push(RwRow::Memory {
            rw_counter: step.next_rw_counter(),
            is_write: false,
            call_id: step.call_id,
            memory_address: target.as_u64() + i as u64,
            value: *value,
            signed: false,
        });
    });
    // create copy row
    step.copy_rows.push(CopyRow {
        tag: CopyTableTag::WriteOutput,
        from_address: target.as_u32(),
        to_address: step.output_len,
        length: length.as_u32(),
        rw_counter: copy_rw_counter,
        data: vec![0; length.as_usize()],
    });
    step.output_len += length.as_u32();
    Ok(())
}
