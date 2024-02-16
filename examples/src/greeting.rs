use crate::deploy_internal;
use fluentbase_sdk::{LowLevelAPI, LowLevelSDK};

pub fn deploy() {
    deploy_internal(include_bytes!("../bin/greeting.wasm"))
}

pub fn main() {
    LowLevelSDK::sys_write("Hello, World".as_bytes());
}
