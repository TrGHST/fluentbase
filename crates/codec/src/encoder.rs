use byteorder::ByteOrder;
use phantom_type::PhantomType;

use crate::buffer::{ReadableBuffer, ReadableBufferImpl};
use crate::{header_item_size, WritableBuffer, WritableBufferImpl};

pub const ALIGN_DEFAULT: usize = 0;
pub const ALIGN_32: usize = 32;
pub const HEADER_ITEM_SIZE_DEFAULT: usize = 4;

pub trait Serializable<E: ByteOrder, const A: usize, T: Sized> {
    /// returns: new bytes written
    fn serialize<B: WritableBuffer<E>>(&self, buffer: &mut B, offset: usize) -> usize;

    fn deserialize<B: ReadableBuffer<E>>(buffer: &B, offset: usize, result: &mut T);
}

pub trait SimpleEncoder<E: ByteOrder, const A: usize, T: Sized> {
    /// returns: new bytes written
    fn encode<W: WritableBuffer<E>>(&self, buffer: &mut W, offset: usize) -> usize;

    fn decode<B: ReadableBuffer<E>>(buffer: &B, offset: usize, result: &mut T);
}

#[macro_export]
macro_rules! simple_encoder_call {
    (@enc $typ:ty, $encoder_type:ident, $endianness:ident, $align:ident, $buffer:expr, $offset:expr, $val:expr) => {
        <$typ as $crate::encoder::$encoder_type<$endianness, $align, $typ>>::encode(
            $val, $buffer, $offset,
        );
    };
    (@dec $typ:ty, $encoder_type:ident, $endianness:ident, $align:ident, $buffer:expr, $offset:expr, $out:expr) => {
        <$typ as $crate::encoder::$encoder_type<$endianness, $align, $typ>>::decode(
            $buffer, $offset, $out,
        );
    };
}

// #[deprecated]
pub trait StructuredEncoder<E: ByteOrder, const A: usize, T: Sized + SimpleEncoder<E, A, T>> {
    const HEADER_ITEM_SIZE: usize = header_item_size!(A, T);
    const HEADER_SIZE: usize = header_item_size!(A, T);

    /// returns: new bytes written
    fn encode<B: WritableBuffer<E>>(&self, buffer: &mut B, offset: usize) -> usize;

    fn decode<B: ReadableBuffer<E>>(buffer: &B, offset: usize, result: &mut T);
}

#[macro_export]
macro_rules! structured_encoder_const_val {
    ($typ:ty, $endianness:ident, $align:ident, $const_ident:ident) => {
        <$typ as $crate::StructuredEncoder<$endianness, $align, $typ>>::$const_ident
    };
}

#[macro_export]
macro_rules! structured_encoder_call {
    (@enc $typ:ty, $endianness:ident, $align:ident, $buffer:expr, $offset:expr, $val_in:expr $(,)?) => {
        <$typ as $crate::StructuredEncoder<$endianness, $align, $typ>>::encode(
            $val_in, $buffer, $offset,
        );
    };
    (@dec $typ:ty, $endianness:ident, $align:ident, $buffer:expr, $offset:expr, $out_out:expr $(,)?) => {
        <$typ as $crate::StructuredEncoder<$endianness, $align, $typ>>::decode(
            $buffer, $offset, $out_out,
        );
    };
}

pub struct FieldMeta<E: ByteOrder, const A: usize, T: Sized, const FIELD_OFFSET: usize>(
    PhantomType<E>,
    PhantomType<T>,
);

impl<E: ByteOrder, const A: usize, T: Sized + Encoder<E, A, T>, const FIELD_OFFSET: usize>
    FieldMeta<E, A, T, FIELD_OFFSET>
{
    pub const FIELD_OFFSET: usize = FIELD_OFFSET;
    pub const FIELD_SIZE: usize = <T as Encoder<E, A, T>>::HEADER_SIZE;

    pub fn decode_field_header(buffer: &[u8], result: &mut T) -> (usize, usize) {
        Self::decode_field_header_at(buffer, Self::FIELD_OFFSET, result)
    }

    pub fn decode_field_header_at(
        buffer: &[u8],
        field_offset: usize,
        result: &mut T,
    ) -> (usize, usize) {
        let mut buffer_decoder = ReadableBufferImpl::new(buffer);
        T::decode(&mut buffer_decoder, field_offset, result)
    }

    pub fn decode_field_body(buffer: &[u8], result: &mut T) {
        Self::decode_field_body_at(buffer, Self::FIELD_OFFSET, result)
    }

    pub fn decode_field_body_at(buffer: &[u8], field_offset: usize, result: &mut T) {
        let mut buffer_decoder = ReadableBufferImpl::new(buffer);
        T::decode_body(&mut buffer_decoder, field_offset, result)
    }
}

pub trait Encoder<E: ByteOrder, const A: usize, T: Sized> {
    const HEADER_SIZE: usize;
    fn header_size(&self) -> usize {
        Self::HEADER_SIZE
    }
    fn encode_to_vec(&self, offset: usize) -> Vec<u8> {
        let mut buffer_encoder = WritableBufferImpl::<E>::new(Self::HEADER_SIZE, None);
        self.encode(&mut buffer_encoder, offset);
        buffer_encoder.finalize()
    }

    /// returns: new bytes written
    fn encode<B: WritableBuffer<E>>(&self, buffer: &mut B, offset: usize) -> usize;

    fn decode<B: ReadableBuffer<E>>(buffer: &B, offset: usize, result: &mut T) -> (usize, usize);

    fn decode_body<B: ReadableBuffer<E>>(buffer: &B, offset: usize, result: &mut T) {
        Self::decode(buffer, offset, result);
    }
}

#[macro_export]
macro_rules! encoder_const_val {
    ($typ:ty, $endianness:ident, $align:ident, $const_ident:ident) => {
        <$typ as $crate::Encoder<$endianness, $align, $typ>>::$const_ident
    };
}

#[macro_export]
macro_rules! encoder_call {
    (@encode $typ:ty, $endianness:ident, $align:ident, $buffer:expr, $offset:expr, $val:expr $(,)?) => {
        <$typ as Encoder<$endianness, $align, $typ>>::encode($val, $buffer, $offset);
    };
    (@decode_body $typ:ty, $endianness:ident, $align:ident, $buffer:expr, $offset:expr, $val:expr $(,)?) => {
        <$typ as Encoder<$endianness, $align, $typ>>::decode_body($buffer, $offset, $val);
    };
}
