#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate core;

#[cfg(feature = "big_endian")]
use byteorder::BE as Endianness;
#[cfg(not(feature = "big_endian"))]
use byteorder::LE as Endianness;

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
#[cfg(test)]
mod tests;
mod vec;
