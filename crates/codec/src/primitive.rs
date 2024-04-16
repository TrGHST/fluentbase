use byteorder::ByteOrder;
use paste::paste;

use crate::buffer::{ReadableBuffer, ReadableBufferImpl};
use crate::encoder::{Serializable, SimpleEncoder};
use crate::{
    fixed_type_size_aligned_padding, header_item_size, size_of, FieldEncoder, WritableBuffer,
};

// macro_rules! impl_simple_encoder {
//     ($typ:ty) => {
//         paste! {
//             impl<E: ByteOrder, const A: usize> $crate::encoder::SimpleEncoder<E, A, $typ> for $typ {
//                 fn encode<W: $crate::buffer::WritableBuffer<E>>(&self, b: &mut W, offset: usize) {
//                     let padding = $crate::fixed_type_size_aligned_padding!(A, $typ);
//                     b.fill_bytes(offset, padding, 0);
//                     b.[<write_ $typ>](offset + padding, *self);
//                 }
//
//                 fn decode(b: &$crate::buffer::ReadableBufferImpl<E>, offset: usize, result: &mut $typ) {
//                     let padding = $crate::fixed_type_size_aligned_padding!(A, $typ);
//                     *result = b.[<read_ $typ>](offset + padding);
//                 }
//             }
//         }
//     };
// }
//
// impl_simple_encoder!(u8);
// impl_simple_encoder!(u16);
// impl_simple_encoder!(u32);
// impl_simple_encoder!(u64);
// impl_simple_encoder!(i8);
// impl_simple_encoder!(i16);
// impl_simple_encoder!(i32);
// impl_simple_encoder!(i64);

macro_rules! impl_serializable {
    ($typ:ty) => {
        paste! {
            impl<E: ByteOrder, const A: usize> Serializable<E, A, $typ> for $typ {
                fn serialize<B: $crate::WritableBuffer<E>>(&self, b: &mut B, offset: usize) {
                    b.[<write_ $typ>](offset, *self);
                }

                fn deserialize<B: $crate::ReadableBuffer<E>>(b: &B, offset: usize, result: &mut Self) {
                    *result = b.[<read_ $typ>](offset);
                }
            }
        }
    };
}
impl_serializable!(u8);
impl_serializable!(u16);
impl_serializable!(u32);
impl_serializable!(u64);
impl_serializable!(i8);
impl_serializable!(i16);
impl_serializable!(i32);
impl_serializable!(i64);

impl<E: ByteOrder, const A: usize, const COUNT: usize, ITEM: Sized + Serializable<E, A, ITEM>>
    Serializable<E, A, [ITEM; COUNT]> for [ITEM; COUNT]
{
    fn serialize<W: WritableBuffer<E>>(&self, b: &mut W, offset: usize) {
        for (i, item) in self.iter().enumerate() {
            item.serialize(b, offset + i * size_of!(ITEM));
        }
        // let padding = fixed_type_size_aligned_padding!(A, ITEM);
        // for (i, item) in self.iter().enumerate() {
        //     b.fill_bytes(offset, padding, 0);
        //     let item_offset = offset + padding + i * (padding + size_of!(ITEM));
        //     item.serialize(b, item_offset);
        // }
    }

    fn deserialize<B: ReadableBuffer<E>>(b: &B, offset: usize, result: &mut Self) {
        for i in 0..COUNT {
            ITEM::deserialize(b, offset + i * size_of!(ITEM), &mut result[i]);
        }
        // let padding = fixed_type_size_aligned_padding!(A, Self);
        // for i in 0..COUNT {
        //     let item_offset = offset + padding + i * (padding + size_of!(ITEM));
        //     ITEM::deserialize(b, item_offset, &mut result[i]);
        // }
    }
}

macro_rules! impl_simple_encoder_primitive {
    ($typ:ty) => {
        impl<E: ByteOrder, const A: usize> SimpleEncoder<E, A, $typ> for $typ {
            fn encode<W: WritableBuffer<E>>(&self, b: &mut W, offset: usize) {
                let padding = fixed_type_size_aligned_padding!(A, $typ);
                b.fill_bytes(offset, padding, 0);
                <$typ as Serializable<E, A, $typ>>::serialize(&self, b, offset + padding);
            }

            fn decode<B: ReadableBuffer<E>>(b: &B, offset: usize, result: &mut $typ) {
                let padding = fixed_type_size_aligned_padding!(A, Self);
                <$typ as Serializable<E, A, $typ>>::deserialize(b, offset + padding, result);
            }
        }
    };
}

impl_simple_encoder_primitive!(u8);
impl_simple_encoder_primitive!(u16);
impl_simple_encoder_primitive!(u32);
impl_simple_encoder_primitive!(u64);
impl_simple_encoder_primitive!(i8);
impl_simple_encoder_primitive!(i16);
impl_simple_encoder_primitive!(i32);
impl_simple_encoder_primitive!(i64);

#[macro_export]
macro_rules! dynamic_buffer_call {
    (@enc $encoder_type:ident, $end:ident, $align:ident, $typ:ty, $buffer_ty:ident, $buffer:expr, $offset:expr, $val:expr) => {
        <$typ as $crate::encoder::$encoder_type<$end, $align, $typ>>::encode::<
            $crate::buffer::$buffer_ty<$end>,
        >($val, $buffer, $offset);
    };
    (@dec $encoder_type:ident, $end:ident, $align:ident, $typ:ty, $buffer:expr, $offset:expr, $out:expr) => {
        <$typ as $crate::encoder::$encoder_type<$end, $align, $typ>>::decode(
            $buffer, $offset, $out,
        );
    };
}

macro_rules! impl_field_encoder_primitive {
    ($typ:ty) => {
        impl<E: ByteOrder, const A: usize> FieldEncoder<E, A, $typ> for $typ {
            const HEADER_ITEM_SIZE: usize = header_item_size!(A, Self);
            const HEADER_SIZE: usize = <Self as FieldEncoder<E, A, Self>>::HEADER_ITEM_SIZE;

            fn encode<W: WritableBuffer<E>>(&self, buffer: &mut W, offset: usize) {
                <Self as SimpleEncoder<E, A, Self>>::encode(self, buffer, offset);
            }

            fn decode(buffer: &ReadableBufferImpl<E>, offset: usize, result: &mut Self) {
                <Self as SimpleEncoder<E, A, Self>>::decode(buffer, offset, result);
            }
        }
    };
}

impl_field_encoder_primitive!(u8);
impl_field_encoder_primitive!(u16);
impl_field_encoder_primitive!(u32);
impl_field_encoder_primitive!(u64);
impl_field_encoder_primitive!(i8);
impl_field_encoder_primitive!(i16);
impl_field_encoder_primitive!(i32);
impl_field_encoder_primitive!(i64);
