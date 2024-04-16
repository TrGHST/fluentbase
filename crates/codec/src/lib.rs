#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate core;

pub use crate::buffer::{ReadableBuffer, ReadableBufferImpl, WritableBuffer, WritableBufferImpl};
pub use crate::encoder::{
    Encoder, FieldMeta, Serializable, SimpleEncoder, StructuredEncoder, ALIGN_32, ALIGN_DEFAULT,
};

mod buffer;
mod empty;
mod encoder;
// mod evm;
// mod hash;
mod encoder_macros;
#[cfg(test)]
mod encoder_tests;
mod macros_codec;
mod macros_common;
mod primitive_impls;
#[cfg(test)]
mod primitive_tests;
mod vec_impls;
#[cfg(test)]
mod vec_tests;
// #[cfg(test)]
// mod tests;
// #[cfg(test)]
// mod buffer_tests;
#[cfg(test)]
mod macros_codec_tests;
