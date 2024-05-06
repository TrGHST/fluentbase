use crate::helpers::debug_log;
use crate::{account::Account, helpers::rwasm_exec_hash};
use alloc::{format, vec};
use fluentbase_sdk::{ContextReader, LowLevelSDK};
use fluentbase_sdk::{LowLevelAPI, WasmCreateMethodInput};
use fluentbase_types::{Address, ExitCode, B256, U256};
use revm_primitives::RWASM_MAX_CODE_SIZE;

pub fn _wasm_create<CR: ContextReader>(input: WasmCreateMethodInput) -> Result<Address, ExitCode> {
    debug_log("_wasm_create start");

    // TODO: "gas calculations"
    // TODO: "call depth stack check >= 1024"

    // check write protection
    if CR::contract_is_static() {
        debug_log(&format!(
            "_wasm_create return: Err: exit_code: {}",
            ExitCode::WriteProtection
        ));
        return Err(ExitCode::WriteProtection);
    }

    // code length can't exceed max constructor limit
    if input.bytecode.len() > RWASM_MAX_CODE_SIZE {
        debug_log(&format!(
            "_wasm_create return: Err: exit_code: {}",
            ExitCode::ContractSizeLimit
        ));
        return Err(ExitCode::ContractSizeLimit);
    }

    let mut source_code_hash: B256 = B256::ZERO;
    LowLevelSDK::crypto_keccak256(
        input.bytecode.as_ptr(),
        input.bytecode.len() as u32,
        source_code_hash.as_mut_ptr(),
    );

    // read value input and contract address
    let caller_address = CR::contract_caller();
    // load deployer and contract accounts
    let mut deployer_account = Account::new_from_jzkt(caller_address);

    // create an account
    let mut contract_account = Account::create_account(
        &mut deployer_account,
        input.value,
        input.salt.map(|salt| (salt, source_code_hash)),
    )?;

    // translate WASM to rWASM
    let exit_code = LowLevelSDK::wasm_to_rwasm(
        input.bytecode.as_ptr(),
        input.bytecode.len() as u32,
        core::ptr::null_mut(),
        0,
    );
    if exit_code != ExitCode::Ok.into_i32() {
        debug_log(&format!(
            "_wasm_create return: panic: exit_code: {}",
            exit_code
        ));
        panic!("wasm create failed, exit code: {}", exit_code);
    }
    let rwasm_bytecode_len = LowLevelSDK::sys_output_size();
    let mut rwasm_bytecode = vec![0u8; rwasm_bytecode_len as usize];
    LowLevelSDK::sys_read_output(rwasm_bytecode.as_mut_ptr(), 0, rwasm_bytecode_len);

    // write deployer to the trie
    deployer_account.write_to_jzkt();

    // write contract to the trie
    contract_account.update_bytecode(&input.bytecode, None, &rwasm_bytecode.into(), None);
    let exit_code = rwasm_exec_hash(
        &contract_account.rwasm_code_hash.as_slice(),
        &[],
        input.gas_limit as u32,
        true,
    );
    // if call is not success set deployed address to zero
    if exit_code != ExitCode::Ok.into_i32() {
        debug_log("_wasm_create return: Err: ExitCode::TransactError");
        return Err(ExitCode::from(exit_code));
    }

    debug_log(&format!(
        "_wasm_create return: Ok: contract_account.address {}",
        contract_account.address
    ));

    Ok(contract_account.address)
}
