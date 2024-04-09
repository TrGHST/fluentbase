use crate::{buffer::WritableBuffer, BufferDecoder, Encoder};
use byteorder::ByteOrder;
use paste::paste;

impl<E: ByteOrder, const A: usize> Encoder<E, A, u8> for u8 {
    const HEADER_SIZE: usize = core::mem::size_of::<u8>();
    fn encode<W: WritableBuffer<E, A>>(&self, encoder: &mut W, field_offset: usize) {
        encoder.write_u8(field_offset, *self);
    }
    fn decode_header(
        decoder: &mut BufferDecoder<E, A>,
        field_offset: usize,
        result: &mut u8,
    ) -> (usize, usize) {
        *result = decoder.read_u8(field_offset);
        (0, 0)
    }
}
impl<E: ByteOrder, const A: usize> Encoder<E, A, bool> for bool {
    const HEADER_SIZE: usize = core::mem::size_of::<bool>();
    fn encode<W: WritableBuffer<E, A>>(&self, encoder: &mut W, field_offset: usize) {
        encoder.write_u8(field_offset, *self as u8);
    }
    fn decode_header(
        decoder: &mut BufferDecoder<E, A>,
        field_offset: usize,
        result: &mut bool,
    ) -> (usize, usize) {
        *result = decoder.read_u8(field_offset) != 0;
        (0, 0)
    }
}

macro_rules! impl_encoder {
    ($typ:ty) => {
        paste! {
            impl<E: ByteOrder, const A:usize> Encoder<E, A, $typ> for $typ {
                const HEADER_SIZE: usize = core::mem::size_of::<$typ>();
                fn encode<W: WritableBuffer<E, A>>(&self, encoder: &mut W, field_offset: usize) {
                    encoder.[<write_ $typ>](field_offset, *self);
                }
                fn decode_header(
                    decoder: &mut BufferDecoder<E, A>,
                    field_offset: usize,
                    result: &mut $typ,
                ) -> (usize, usize) {
                    *result = decoder.[<read_ $typ>](field_offset);
                    (0, 0)
                }
            }
        }
    };
}

impl_encoder!(u16);
impl_encoder!(u32);
impl_encoder!(u64);
impl_encoder!(i16);
impl_encoder!(i32);
impl_encoder!(i64);

impl<E: ByteOrder, const A: usize, T: Sized + Encoder<E, A, T>, const N: usize>
    Encoder<E, A, [T; N]> for [T; N]
{
    const HEADER_SIZE: usize = T::HEADER_SIZE * N;

    fn encode<W: WritableBuffer<E, A>>(&self, encoder: &mut W, field_offset: usize) {
        (0..N).for_each(|i| {
            self[i].encode(encoder, field_offset + i * T::HEADER_SIZE);
        });
    }

    fn decode_header(
        decoder: &mut BufferDecoder<E, A>,
        field_offset: usize,
        result: &mut [T; N],
    ) -> (usize, usize) {
        (0..N).for_each(|i| {
            T::decode_body(decoder, field_offset + i * T::HEADER_SIZE, &mut result[i]);
        });
        (0, 0)
    }
}

impl<E: ByteOrder, const A: usize, T: Sized + Encoder<E, A, T> + Default> Encoder<E, A, Option<T>>
    for Option<T>
{
    const HEADER_SIZE: usize = 1 + T::HEADER_SIZE;

    fn encode<W: WritableBuffer<E, A>>(&self, encoder: &mut W, field_offset: usize) {
        let option_flag = if self.is_some() { 1u8 } else { 0u8 };
        option_flag.encode(encoder, field_offset);
        if let Some(value) = &self {
            value.encode(encoder, field_offset + 1);
        } else {
            T::default().encode(encoder, field_offset + 1);
        }
    }

    fn decode_header(
        decoder: &mut BufferDecoder<E, A>,
        field_offset: usize,
        result: &mut Option<T>,
    ) -> (usize, usize) {
        let mut option_flag: u8 = 0;
        u8::decode_header(decoder, field_offset, &mut option_flag);
        *result = if option_flag != 0 {
            let mut result_inner: T = Default::default();
            T::decode_header(decoder, field_offset + 1, &mut result_inner);
            Some(result_inner)
        } else {
            None
        };
        (0, 0)
    }

    fn decode_body(decoder: &mut BufferDecoder<E, A>, field_offset: usize, result: &mut Option<T>) {
        let mut option_flag: u8 = 0;
        u8::decode_header(decoder, field_offset, &mut option_flag);
        *result = if option_flag != 0 {
            let mut result_inner: T = Default::default();
            T::decode_body(decoder, field_offset + 1, &mut result_inner);
            Some(result_inner)
        } else {
            None
        };
    }
}
