use byteorder::ByteOrder;
use paste::paste;

use crate::buffer::ReadableBuffer;
use crate::encoder::{Serializable, SimpleEncoder};
use crate::{fixed_type_size_aligned_padding, size_of, WritableBuffer};

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
//                 fn decode(b: &$crate::buffer::ReadableBuffer<E>, offset: usize, result: &mut $typ) {
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
                fn serialize<W: WritableBuffer<E>>(&self, b: &mut W, offset: usize) {
                    // let padding = fixed_type_size_aligned_padding!(A, Self);
                    // b.fill_bytes(offset, padding, 0);
                    b.[<write_ $typ>](offset/* + padding*/, *self);
                }

                fn deserialize(b: &ReadableBuffer<E>, offset: usize, result: &mut Self) {
                    // let padding = fixed_type_size_aligned_padding!(A, Self);
                    *result = b.[<read_ $typ>](offset/* + padding*/);
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
    }

    fn deserialize(b: &ReadableBuffer<E>, offset: usize, result: &mut Self) {
        for i in 0..COUNT {
            ITEM::deserialize(b, offset + i * size_of!(ITEM), &mut result[i]);
        }
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

            fn decode(b: &ReadableBuffer<E>, offset: usize, result: &mut $typ) {
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

// impl<E: ByteOrder, const A: usize, T: Sized + Serializable<E, A, T>> SimpleEncoder<E, A, T> for T {
//     fn encode<W: WritableBuffer<E>>(&self, b: &mut W, offset: usize) {
//         let padding = fixed_type_size_aligned_padding!(A, T);
//         b.fill_bytes(offset, padding, 0);
//         <T as Serializable<E, A, T>>::serialize(&self, b, offset + padding);
//     }
//
//     fn decode(b: &ReadableBuffer<E>, offset: usize, result: &mut T) {
//         let padding = fixed_type_size_aligned_padding!(A, Self);
//         T::deserialize(b, offset + padding, result);
//     }
// }

#[macro_export]
macro_rules! dynamic_buffer_encode {
    ($encoder_type:ident, $end:ident, $align:ident, $typ:ty, $buffer_ty:ident, $buffer:expr, $offset:expr, $val:expr) => {
        <$typ as $crate::encoder::$encoder_type<$end, $align, $typ>>::encode::<
            $crate::buffer::$buffer_ty<$end>,
        >($val, $buffer, $offset);
    };
}

#[macro_export]
macro_rules! dynamic_buffer_decode {
    ($encoder_type:ident, $end:ident, $align:ident, $typ:ty, $buffer:expr, $offset:expr, $out:expr) => {
        <$typ as $crate::encoder::$encoder_type<$end, $align, $typ>>::decode(
            $buffer, $offset, $out,
        );
    };
}
