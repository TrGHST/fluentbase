#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate core;

pub use crate::buffer::{DynamicBuffer, WritableBuffer};

mod buffer;
mod empty;
mod encoder;
// mod evm;
// mod hash;
// mod macros;
mod encoder_helpers;
#[cfg(test)]
mod encoder_tests;
mod macros_common;
mod primitive;
#[cfg(test)]
mod primitive_tests;
mod vec;
#[cfg(test)]
mod vec_tests;
// #[cfg(test)]
// mod tests;
// #[cfg(test)]
// mod buffer_tests;
// #[cfg(test)]
// mod macros_tests;
