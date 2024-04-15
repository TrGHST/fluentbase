use byteorder::ByteOrder;

use crate::buffer::{ReadableBuffer, WritableBuffer};
use crate::header_item_size;

pub const ALIGN_DEFAULT: usize = 0;
pub const ALIGN_32: usize = 32;
pub const HEADER_ITEM_SIZE_DEFAULT: usize = 4;

pub trait Serializable<E: ByteOrder, const A: usize, T: Sized> {
    fn serialize<W: WritableBuffer<E>>(&self, buffer: &mut W, offset: usize);

    fn deserialize(buffer: &ReadableBuffer<E>, offset: usize, result: &mut T);
}

pub trait SimpleEncoder<E: ByteOrder, const A: usize, T: Sized> {
    fn encode<W: WritableBuffer<E>>(&self, buffer: &mut W, offset: usize);

    fn decode(buffer: &ReadableBuffer<E>, offset: usize, result: &mut T);
}

#[macro_export]
macro_rules! simple_encoder_encode {
    ($typ:ty, $encoder_type:ident, $endianness:ident, $align:ident, $buffer:expr, $offset:expr, $val:expr) => {
        <$typ as $crate::encoder::$encoder_type<$endianness, $align, $typ>>::encode(
            $val, $buffer, $offset,
        );
    };
}

#[macro_export]
macro_rules! simple_encoder_decode {
    ($typ:ty, $encoder_type:ident, $endianness:ident, $align:ident, $buffer:expr, $offset:expr, $out:expr) => {
        <$typ as $crate::encoder::$encoder_type<$endianness, $align, $typ>>::decode(
            $buffer, $offset, $out,
        );
    };
}

pub trait FieldEncoder<E: ByteOrder, const A: usize, T: Sized> {
    const HEADER_ITEM_SIZE: usize = header_item_size!(A, T);
    const HEADER_SIZE: usize = header_item_size!(A, T);

    fn encode<W: WritableBuffer<E>>(&self, buffer: &mut W, offset: usize);

    fn decode(buffer: &ReadableBuffer<E>, offset: usize, result: &mut T);
}

#[macro_export]
macro_rules! field_encoder_const_val {
    ($self_ty:ty, $endianness:ident, $align:ident, $const_ident:ident) => {
        <$self_ty as FieldEncoder<$endianness, $align, $self_ty>>::$const_ident
    };
}
