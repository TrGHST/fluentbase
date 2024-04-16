use byteorder::ByteOrder;
use phantom_type::PhantomType;

use crate::buffer::ReadableBuffer;
use crate::{header_item_size, DynamicBuffer, WritableBuffer};

use alloc::vec::Vec;

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

pub trait FieldEncoder<E: ByteOrder, const A: usize, T: Sized + SimpleEncoder<E, A, T>> {
    const HEADER_ITEM_SIZE: usize = header_item_size!(A, T);
    const HEADER_SIZE: usize = header_item_size!(A, T);

    fn encode<W: WritableBuffer<E>>(&self, buffer: &mut W, offset: usize);

    fn decode(buffer: &ReadableBuffer<E>, offset: usize, result: &mut T);
}

#[macro_export]
macro_rules! field_encoder_const_val {
    ($self_ty:ty, $endianness:ident, $align:ident, $const_ident:ident) => {
        <$self_ty as $crate::FieldEncoder<$endianness, $align, $self_ty>>::$const_ident
    };
}

#[macro_export]
macro_rules! field_encoder_call {
    (@enc $typ:ty, $endianness:ident, $align:ident, $buffer:expr, $offset:expr, $val_in:expr $(,)?) => {
        <$typ as $crate::FieldEncoder<$endianness, $align, $typ>>::encode(
            $val_in, $buffer, $offset,
        );
    };
    (@dec $typ:ty, $endianness:ident, $align:ident, $buffer:expr, $offset:expr, $out_out:expr $(,)?) => {
        <$typ as $crate::FieldEncoder<$endianness, $align, $typ>>::decode(
            $buffer, $offset, $out_out,
        );
    };
}

pub trait Encoder<E: ByteOrder, const A: usize, T: Sized> {
    const HEADER_SIZE: usize;
    fn header_size(&self) -> usize {
        Self::HEADER_SIZE
    }
    fn encode_to_vec(&self, field_offset: usize) -> Vec<u8> {
        let mut buffer_encoder = DynamicBuffer::<E>::new(Self::HEADER_SIZE, None);
        self.encode(&mut buffer_encoder, field_offset);
        buffer_encoder.finalize()
    }

    fn encode<W: WritableBuffer<E>>(&self, encoder: &mut W, field_offset: usize);

    fn decode_header(
        decoder: &mut ReadableBuffer<E>,
        field_offset: usize,
        result: &mut T,
    ) -> (usize, usize);

    fn decode_body(decoder: &mut ReadableBuffer<E>, field_offset: usize, result: &mut T) {
        Self::decode_header(decoder, field_offset, result);
    }
}

pub struct FieldMeta<
    E: ByteOrder,
    const A: usize,
    T: Sized + SimpleEncoder<E, A, T> + FieldEncoder<E, A, T>,
    const FIELD_OFFSET: usize,
>(PhantomType<E>, PhantomType<T>);

impl<
        E: ByteOrder,
        const A: usize,
        T: Sized + SimpleEncoder<E, A, T> + FieldEncoder<E, A, T> + Encoder<E, A, T>,
        const FIELD_OFFSET: usize,
    > FieldMeta<E, A, T, FIELD_OFFSET>
{
    pub const FIELD_OFFSET: usize = FIELD_OFFSET;
    pub const FIELD_SIZE: usize = <T as FieldEncoder<E, A, T>>::HEADER_SIZE;

    pub fn decode_field_header(buffer: &[u8], result: &mut T) -> (usize, usize) {
        Self::decode_field_header_at(buffer, Self::FIELD_OFFSET, result)
    }

    pub fn decode_field_header_at(
        buffer: &[u8],
        field_offset: usize,
        result: &mut T,
    ) -> (usize, usize) {
        let mut buffer_decoder = ReadableBuffer::new(buffer);
        T::decode_header(&mut buffer_decoder, field_offset, result)
    }

    pub fn decode_field_body(buffer: &[u8], result: &mut T) {
        Self::decode_field_body_at(buffer, Self::FIELD_OFFSET, result)
    }

    pub fn decode_field_body_at(buffer: &[u8], field_offset: usize, result: &mut T) {
        let mut buffer_decoder = ReadableBuffer::new(buffer);
        T::decode_body(&mut buffer_decoder, field_offset, result)
    }
}
