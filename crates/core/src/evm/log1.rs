use fluentbase_sdk::{ContextReader, LowLevelAPI, LowLevelSDK};

pub fn _evm_log1<CR: ContextReader>(
    data_offset: *const u8,
    data_size: u32,
    topic32_1_offset: *const u8,
) {
    const TOPICS_COUNT: usize = 1;
    let address = CR::contract_address();
    LowLevelSDK::jzkt_emit_log(
        address.as_ptr(),
        topic32_1_offset as *const [u8; 32],
        32,
        data_offset,
        data_size,
    );
}
