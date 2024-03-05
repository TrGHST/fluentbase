use fluentbase_codec::Encoder;
use fluentbase_runtime::{
    instruction::runtime_register_sovereign_handlers,
    types::{Bytes, RuntimeError, STATE_MAIN},
    ExecutionResult,
    Runtime,
    RuntimeContext,
};
use fluentbase_sdk::evm::ContractInput;
use rwasm_codegen::{
    rwasm::{Config, Engine, Linker, Module, Store},
    Compiler,
    CompilerConfig,
};

pub(crate) fn wasm2rwasm(wasm_binary: &[u8], inject_fuel_consumption: bool) -> Vec<u8> {
    let import_linker = Runtime::<()>::new_sovereign_linker();
    Compiler::new_with_linker(
        &wasm_binary.to_vec(),
        CompilerConfig::default().fuel_consume(inject_fuel_consumption),
        Some(&import_linker),
    )
    .unwrap()
    .finalize()
    .unwrap()
}

pub(crate) fn run_rwasm_with_evm_input(
    wasm_binary: Vec<u8>,
    input_data: &[u8],
) -> ExecutionResult<()> {
    let input_data = {
        let mut contract_input = ContractInput::default();
        contract_input.contract_input = Bytes::copy_from_slice(input_data);
        contract_input.encode_to_vec(0)
    };
    let rwasm_binary = wasm2rwasm(wasm_binary.as_slice(), false);
    let mut ctx = RuntimeContext::new(rwasm_binary);
    ctx.with_state(STATE_MAIN)
        .with_fuel_limit(100_000)
        .with_input(input_data)
        .with_catch_trap(true);
    let import_linker = Runtime::<()>::new_sovereign_linker();
    let mut runtime = Runtime::<()>::new(ctx, &import_linker).unwrap();
    runtime.data_mut().clean_output();
    runtime.call().unwrap()
}

pub(crate) fn run_rwasm_with_raw_input(
    wasm_binary: Vec<u8>,
    input_data: &[u8],
    verify_wasm: bool,
) -> ExecutionResult<()> {
    // make sure at least wasm binary works well
    let wasm_exit_code = if verify_wasm {
        let config = Config::default();
        let engine = Engine::new(&config);
        let module = Module::new(&engine, wasm_binary.as_slice()).unwrap();
        let mut ctx = RuntimeContext::<()>::new(vec![]);
        ctx.with_state(STATE_MAIN)
            .with_fuel_limit(10_000_000)
            .with_input(input_data.to_vec())
            .with_catch_trap(true);
        let mut store = Store::new(&engine, ctx);
        let mut linker = Linker::new(&engine);
        runtime_register_sovereign_handlers(&mut linker, &mut store);
        let instance = linker
            .instantiate(&mut store, &module)
            .unwrap()
            .start(&mut store)
            .unwrap();
        let main_func = instance.get_func(&store, "main").unwrap();
        match main_func.call(&mut store, &[], &mut []) {
            Err(err) => {
                let exit_code =
                    Runtime::<RuntimeContext<()>>::catch_trap(&RuntimeError::Rwasm(err));
                if exit_code != 0 {
                    panic!("err happened during wasm execution: {:?}", exit_code);
                }
                // let mut lines = String::new();
                // for log in store.tracer().logs.iter() {
                //     let stack = log
                //         .stack
                //         .iter()
                //         .map(|v| v.to_bits() as i64)
                //         .collect::<Vec<_>>();
                //     lines += format!("{}\t{:?}\t{:?}\n", log.program_counter, log.opcode, stack)
                //         .as_str();
                // }
                // let _ = fs::create_dir("./tmp");
                // fs::write("./tmp/cairo.txt", lines).unwrap();
            }
            Ok(_) => {}
        }
        let wasm_exit_code = store.data().exit_code();
        Some(wasm_exit_code)
    } else {
        None
    };
    // compile and run wasm binary
    let rwasm_binary = wasm2rwasm(wasm_binary.as_slice(), false);
    let mut ctx = RuntimeContext::new(rwasm_binary);
    ctx.with_state(STATE_MAIN)
        .with_fuel_limit(10_000_000)
        .with_input(input_data.to_vec())
        .with_catch_trap(true);
    let import_linker = Runtime::<()>::new_sovereign_linker();
    let mut runtime = Runtime::<()>::new(ctx, &import_linker).unwrap();
    runtime.data_mut().clean_output();
    let execution_result = runtime.call().unwrap();
    if let Some(wasm_exit_code) = wasm_exit_code {
        assert_eq!(execution_result.data().exit_code(), wasm_exit_code);
    }
    execution_result
}
