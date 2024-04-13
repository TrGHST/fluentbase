#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate core;

pub use crate::{
    buffer::{BufferDecoder, BufferEncoder, WritableBuffer},
    empty::EmptyVec,
    encoder::{Encoder, FieldEncoder},
};

mod buffer;
mod empty;
mod encoder;
mod evm;
mod hash;
mod macros;
mod primitive;
// #[cfg(test)]
// mod tests;
#[cfg(test)]
mod buffer_tests;
mod macros_tests;
mod vec;
// mod test;
