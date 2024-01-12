use crate::translator::{
    host::Host,
    instructions::utilities::replace_current_opcode_with_call_to_subroutine,
    translator::Translator,
};
use log::debug;

pub fn magic_prefix<H: Host>(_translator: &mut Translator<'_>, _host: &mut H) {
    const OP: &str = "MAGIC_PREFIX";
    debug!("op:{}", OP);
    // https://eips.ethereum.org/EIPS/eip-3541
    // DO NOTHING
}

pub fn keccak256<H: Host>(translator: &mut Translator<'_>, host: &mut H) {
    const OP: &str = "KECCAK256";
    debug!("op:{}", OP);
    replace_current_opcode_with_call_to_subroutine(translator, host);
}

pub fn address<H: Host>(translator: &mut Translator<'_>, host: &mut H) {
    const OP: &str = "ADDRESS";
    debug!("op:{}", OP);
    replace_current_opcode_with_call_to_subroutine(translator, host);
}

pub fn caller<H: Host>(translator: &mut Translator<'_>, host: &mut H) {
    const OP: &str = "CALLER";
    debug!("op:{}", OP);
    replace_current_opcode_with_call_to_subroutine(translator, host);
}

pub fn codesize<H: Host>(translator: &mut Translator<'_>, host: &mut H) {
    const OP: &str = "CODESIZE";
    debug!("op:{}", OP);
    replace_current_opcode_with_call_to_subroutine(translator, host);
}

pub fn codecopy<H: Host>(_translator: &mut Translator<'_>, _host: &mut H) {
    const OP: &str = "CODECOPY";
    panic!("op:{} not implemented", OP);
}

pub fn calldataload<H: Host>(translator: &mut Translator<'_>, host: &mut H) {
    const OP: &str = "CALLDATALOAD";
    debug!("op:{}", OP);
    replace_current_opcode_with_call_to_subroutine(translator, host);
}

pub fn calldatasize<H: Host>(translator: &mut Translator<'_>, host: &mut H) {
    const OP: &str = "CALLDATASIZE";
    debug!("op:{}", OP);
    replace_current_opcode_with_call_to_subroutine(translator, host);
}

pub fn calldatacopy<H: Host>(translator: &mut Translator<'_>, host: &mut H) {
    const OP: &str = "CALLDATACOPY";
    debug!("op:{}", OP);
    replace_current_opcode_with_call_to_subroutine(translator, host);
}

pub fn callvalue<H: Host>(translator: &mut Translator<'_>, host: &mut H) {
    const OP: &str = "CALLVALUE";
    debug!("op:{}", OP);
    replace_current_opcode_with_call_to_subroutine(translator, host);
}

pub fn returndatasize<H: Host>(_translator: &mut Translator<'_>, _host: &mut H) {
    const OP: &str = "RETURNDATASIZE";
    panic!("op:{} not implemented", OP);
}

pub fn returndatacopy<H: Host>(_translator: &mut Translator<'_>, _host: &mut H) {
    const OP: &str = "RETURNDATACOPY";
    panic!("op:{} not implemented", OP);
}

pub fn gas<H: Host>(translator: &mut Translator<'_>, host: &mut H) {
    const OP: &str = "GAS";
    debug!("op:{}", OP);
    replace_current_opcode_with_call_to_subroutine(translator, host);
}
