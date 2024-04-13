use byteorder::ByteOrder;
use paste::paste;

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
