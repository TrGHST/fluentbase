use crate::{
    account::Account,
    account_types::MAX_CODE_SIZE,
    evm::{calc_create_address, read_address_from_input, SpecDefault},
    fluent_host::FluentHost,
};
use alloc::{alloc::alloc, boxed::Box};
use core::{alloc::Layout, ptr};
use fluentbase_sdk::{
    evm::{ContractInput, ExecutionContext, IContractInput, U256},
    LowLevelAPI,
    LowLevelSDK,
};
use fluentbase_types::ExitCode;
use revm_interpreter::{
    analysis::to_analysed,
    opcode::make_instruction_table,
    primitives::{Address, Bytecode, Bytes},
    BytecodeLocked,
    Contract,
    Interpreter,
    SharedMemory,
};

#[no_mangle]
pub fn _evm_create(
    value32_offset: *const u8,
    code_offset: *const u8,
    code_length: u32,
    out_address20_offset: *mut u8,
    gas_limit: u32,
) -> ExitCode {
    // TODO: "gas calculations"
    // TODO: "call depth stack check >= 1024"
    // check write protection
    if ExecutionContext::contract_is_static() {
        return ExitCode::WriteProtection;
    }
    // read value input and contract address
    let value32_slice = unsafe { &*ptr::slice_from_raw_parts(value32_offset, 32) };
    let value = U256::from_be_slice(value32_slice);
    let tx_caller_address =
        read_address_from_input(<ContractInput as IContractInput>::TxCaller::FIELD_OFFSET);
    // load deployer and contract accounts
    let mut deployer_account = Account::new_from_jzkt(&tx_caller_address);
    let deployed_contract_address = calc_create_address(&tx_caller_address, deployer_account.nonce);
    let mut contract_account = Account::new_from_jzkt(&deployed_contract_address);
    // if nonce or code is not empty then its collision
    if contract_account.is_not_empty() {
        return ExitCode::CreateCollision;
    }
    deployer_account.inc_nonce();
    contract_account.nonce = 1;
    // transfer value to the just created account
    if !deployer_account.transfer_value(&mut contract_account, &value) {
        return ExitCode::InsufficientBalance;
    }
    let deployer_bytecode_slice =
        unsafe { &*ptr::slice_from_raw_parts(code_offset, code_length as usize) };
    let deployer_bytecode_bytes = Bytes::from_static(deployer_bytecode_slice);
    let deployer_bytecode = to_analysed(Bytecode::new_raw(deployer_bytecode_bytes));
    let deployer_bytecode_locked = BytecodeLocked::try_from(deployer_bytecode).unwrap();

    let contract = Contract {
        hash: deployer_bytecode_locked.hash_slow(),
        bytecode: deployer_bytecode_locked,
        address: Address::new(deployed_contract_address.into_array()),
        caller: Address::new(tx_caller_address.into_array()),
        ..Default::default()
    };
    let mut interpreter = Interpreter::new(Box::new(contract), gas_limit as u64, false);
    let instruction_table = make_instruction_table::<FluentHost, SpecDefault>();
    let mut host = FluentHost::default();
    let shared_memory = SharedMemory::new();
    let interpreter_result = interpreter.run(shared_memory, &instruction_table, &mut host);
    let interpreter_result = if let Some(v) = interpreter_result.into_result_return() {
        v
    } else {
        return ExitCode::EVMCreateError;
    };
    if interpreter_result.is_error()
        || interpreter_result.is_revert()
        || !interpreter_result.is_ok()
    {
        return ExitCode::EVMCreateError;
    }
    assert!(interpreter_result.is_ok());
    let deployed_bytecode =
        fluentbase_types::Bytes::copy_from_slice(interpreter_result.output.iter().as_slice());

    deployer_account.write_to_jzkt();
    contract_account.update_source_bytecode(&deployed_bytecode);

    // TODO convert $deployed_bytecode into rwasm code ($deployed_rwasm_bytecode)
    // TODO save $deployed_rwasm_bytecode into account (with its poseidon hash)

    // read output bytecode
    let bytecode_length = LowLevelSDK::sys_output_size();
    if bytecode_length > MAX_CODE_SIZE {
        return ExitCode::ContractSizeLimit;
    }
    let bytecode = unsafe {
        alloc(Layout::from_size_align_unchecked(
            bytecode_length as usize,
            8,
        ))
    };
    LowLevelSDK::sys_read_output(bytecode, 0, bytecode_length);

    unsafe { ptr::copy(deployed_contract_address.as_ptr(), out_address20_offset, 20) }

    ExitCode::Ok
}
