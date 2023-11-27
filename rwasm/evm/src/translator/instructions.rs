//! EVM opcode implementations.

#[macro_use]

pub mod arithmetic;
pub mod bitwise;
pub mod control;
mod host;
pub mod host_env;
pub mod i256;
pub mod memory;
pub mod opcode;
pub mod stack;
pub mod system;
mod utilities;

pub use opcode::{Instruction, OpCode, OPCODE_JUMPMAP};
