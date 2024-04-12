use byteorder::ByteOrder;
use paste::paste;

use crate::buffer::ReadableBuffer;
use crate::encoder::{FieldEncoder, SimpleEncoder};
use crate::WritableBuffer;

macro_rules! impl_simple_encoder {
    ($typ:ty) => {
        paste! {
            impl<E: ByteOrder, const A: usize> $crate::encoder::SimpleEncoder<E, A, $typ> for $typ {
                fn encode<W: $crate::buffer::WritableBuffer<E>>(&self, b: &mut W, offset: usize) {
                    let padding = $crate::fixed_type_size_aligned_padding!(A, $typ);
                    b.fill_bytes(offset, padding, 0);
                    b.[<write_ $typ>](offset + padding, *self);
                }

                fn decode(b: & $crate::buffer::ReadableBuffer<E>, offset: usize, result: &mut $typ) {
                    let padding = $crate::fixed_type_size_aligned_padding!(A, $typ);
                    *result = b.[<read_ $typ>](offset + padding);
                }
            }
        }
    };
}

impl_simple_encoder!(u8);
impl_simple_encoder!(u16);
impl_simple_encoder!(u32);
impl_simple_encoder!(u64);
impl_simple_encoder!(i16);
impl_simple_encoder!(i32);
impl_simple_encoder!(i64);

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

// impl<E: ByteOrder, const A: usize, T: Sized> FieldEncoder<E, A, T> for u8 {
//     fn encode<W: WritableBuffer<E>, S: SimpleEncoder<E, A, T>>(
//         &self,
//         encoder: S,
//         buffer: &mut W,
//         offset: usize,
//     ) {
//         encoder.encode(buffer, offset);
//     }
//
//     fn decode<S: SimpleEncoder<E, A, T>>(
//         buffer: &ReadableBuffer<E>,
//         offset: usize,
//         result: &mut T,
//     ) {
//         S::decode(buffer, offset, result);
//     }
// }

// impl<E: ByteOrder, const A: usize> Encoder<E, A, u8> for u8 {
//     const HEADER_SIZE: usize = header_item_size!(A, Self);
//     fn encode<W: WritableBuffer<E>>(&self, encoder: &mut W, field_offset: usize) {
//         let field_offset_fact = effective_offset!(field_offset, A, Self);
//         encoder.write_u8(field_offset_fact, *self);
//     }
//     fn decode_header(
//         decoder: &mut BufferDecoder<E>,
//         field_offset: usize,
//         result: &mut u8,
//     ) -> (usize, usize) {
//         *result = decoder.read_u8(field_offset);
//         (0, 0)
//     }
// }
// impl<E: ByteOrder, const A: usize> Encoder<E, A, bool> for bool {
//     const HEADER_SIZE: usize = header_item_size!(A, Self);
//     fn encode<W: WritableBuffer<E>>(&self, encoder: &mut W, field_offset: usize) {
//         let field_offset_fact = effective_offset!(field_offset, A, Self);
//         encoder.write_u8(field_offset_fact, *self as u8);
//     }
//     fn decode_header(
//         decoder: &mut BufferDecoder<E>,
//         field_offset: usize,
//         result: &mut bool,
//     ) -> (usize, usize) {
//         *result = decoder.read_u8(field_offset) != 0;
//         (0, 0)
//     }
// }

// macro_rules! impl_encoder {
//     ($typ:ty) => {
//         paste! {
//             impl<E: ByteOrder, const A:usize> Encoder<E, A, $typ> for $typ {
//                 const HEADER_SIZE: usize = $crate::header_item_size!(A, Self);
//                 fn encode<W: WritableBuffer<E>>(&self, encoder: &mut W, field_offset: usize) {
//                     let field_offset_fact = effective_offset!(field_offset, A, Self);
//                     encoder.[<write_ $typ>](field_offset_fact, *self);
//                 }
//                 fn decode_header(
//                     decoder: &mut BufferDecoder<E>,
//                     field_offset: usize,
//                     result: &mut $typ,
//                 ) -> (usize, usize) {
//                     *result = decoder.[<read_ $typ>](field_offset);
//                     (0, 0)
//                 }
//             }
//         }
//     };
// }
//
// impl_encoder!(u16);
// impl_encoder!(u32);
// impl_encoder!(u64);
// impl_encoder!(i16);
// impl_encoder!(i32);
// impl_encoder!(i64);

// impl<E: ByteOrder, const A: usize, T: Sized + Encoder<E, A, T>, const N: usize>
//     Encoder<E, A, [T; N]> for [T; N]
// {
//     const HEADER_SIZE: usize = T::HEADER_SIZE * N;
//
//     fn encode<W: WritableBuffer<E>>(&self, encoder: &mut W, field_offset: usize) {
//         (0..N).for_each(|i| {
//             Encoder::<E, A, T>::encode(&self[i], encoder, field_offset + i * T::HEADER_SIZE);
//         });
//     }
//
//     fn decode_header(
//         decoder: &mut BufferDecoder<E>,
//         field_offset: usize,
//         result: &mut [T; N],
//     ) -> (usize, usize) {
//         (0..N).for_each(|i| {
//             T::decode_body(decoder, field_offset + i * T::HEADER_SIZE, &mut result[i]);
//         });
//         (0, 0)
//     }
// }
//
// impl<E: ByteOrder, const A: usize, T: Sized + Encoder<E, A, T> + Default> Encoder<E, A, Option<T>>
//     for Option<T>
// {
//     const HEADER_SIZE: usize = 1 + T::HEADER_SIZE;
//
//     fn encode<W: WritableBuffer<E>>(&self, encoder: &mut W, field_offset: usize) {
//         let option_flag = if self.is_some() { 1u8 } else { 0u8 };
//         <u8 as Encoder<E, A, u8>>::encode::<W>(&option_flag, encoder, field_offset);
//         if let Some(value) = &self {
//             value.encode(encoder, field_offset + 1);
//         } else {
//             T::default().encode(encoder, field_offset + 1);
//         }
//     }
//
//     fn decode_header(
//         decoder: &mut BufferDecoder<E>,
//         field_offset: usize,
//         result: &mut Option<T>,
//     ) -> (usize, usize) {
//         let mut option_flag: u8 = 0;
//         <u8 as Encoder<E, A, u8>>::decode_header(decoder, field_offset, &mut option_flag);
//         *result = if option_flag != 0 {
//             let mut result_inner: T = Default::default();
//             T::decode_header(decoder, field_offset + 1, &mut result_inner);
//             Some(result_inner)
//         } else {
//             None
//         };
//         (0, 0)
//     }
//
//     fn decode_body(decoder: &mut BufferDecoder<E>, field_offset: usize, result: &mut Option<T>) {
//         let mut option_flag: u8 = 0;
//         <u8 as Encoder<E, A, u8>>::decode_header(decoder, field_offset, &mut option_flag);
//         *result = if option_flag != 0 {
//             let mut result_inner: T = Default::default();
//             T::decode_body(decoder, field_offset + 1, &mut result_inner);
//             Some(result_inner)
//         } else {
//             None
//         };
//     }
// }
