use alloc::vec::Vec;

use byteorder::ByteOrder;

use crate::buffer::ReadableBufferImpl;
use crate::encoder::{FieldEncoder, Serializable, SimpleEncoder, ALIGN_DEFAULT};
use crate::{
    buffer::WritableBuffer, dynamic_size_aligned_padding, field_encoder_const_val,
    fixed_type_size_aligned, fixed_type_size_aligned_padding, header_item_size, header_size,
    if_align_default_then, simple_encoder_decode, simple_encoder_encode, size_of, ReadableBuffer,
};

macro_rules! impl_simple_encoder_vec {
    ($typ:ty) => {
        impl<E: ByteOrder, const A: usize> $crate::SimpleEncoder<E, A, Vec<$typ>> for Vec<$typ> {
            fn encode<W: WritableBuffer<E>>(&self, buffer: &mut W, offset: usize) {
                let item_size = $crate::size_of!($typ);
                let len = item_size * self.len();
                buffer.fill_bytes(offset, len, 0);
                for (i, v) in self.iter().enumerate() {
                    <$typ as $crate::SimpleEncoder<E, ALIGN_DEFAULT, $typ>>::encode(
                        &v,
                        buffer,
                        offset + i * item_size,
                    );
                }
                $crate::if_align_default_then!(A, {}, {
                    let padding_count = dynamic_size_aligned_padding!(A, len);
                    buffer.fill_bytes(offset + self.len() * item_size, padding_count, 0);
                });
            }

            fn decode<B: $crate::ReadableBuffer<E>>(
                buffer: &B,
                offset: usize,
                result: &mut Vec<$typ>,
            ) {
                let size_of_item = $crate::size_of!($typ);
                result.resize((buffer.len() - offset) / size_of_item, <$typ>::default());
                for (i, v) in (*result).iter_mut().enumerate() {
                    <$typ as SimpleEncoder<E, ALIGN_DEFAULT, $typ>>::decode(
                        buffer,
                        offset + i * size_of_item,
                        v,
                    );
                }
            }
        }
    };
}

impl_simple_encoder_vec!(u8);
impl_simple_encoder_vec!(u16);
impl_simple_encoder_vec!(u32);
impl_simple_encoder_vec!(u64);
impl_simple_encoder_vec!(i8);
impl_simple_encoder_vec!(i16);
impl_simple_encoder_vec!(i32);
impl_simple_encoder_vec!(i64);

impl<
        E: ByteOrder,
        const A: usize,
        const COUNT: usize,
        ITEM: Sized + Clone + Serializable<E, A, ITEM>,
    > SimpleEncoder<E, A, [ITEM; COUNT]> for [ITEM; COUNT]
{
    fn encode<W: WritableBuffer<E>>(&self, buffer: &mut W, offset: usize) {
        let elem_padding = fixed_type_size_aligned_padding!(A, [ITEM; COUNT]);
        let elem_size = fixed_type_size_aligned!(A, [ITEM; COUNT]);
        let bytes_len = elem_size * self.len();
        buffer.fill_bytes(offset, bytes_len, 0);
        let elem_offset = offset + elem_padding + elem_size;
        <[ITEM; COUNT]>::serialize(self, buffer, elem_offset);
        if_align_default_then!(A, {}, {
            let padding_count = dynamic_size_aligned_padding!(A, bytes_len);
            buffer.fill_bytes(offset + self.len() * elem_size, padding_count, 0);
        });
    }

    fn decode<B: ReadableBuffer<E>>(buffer: &B, offset: usize, result: &mut Self) {
        let elem_padding = fixed_type_size_aligned_padding!(A, [ITEM; COUNT]);
        let elem_size = fixed_type_size_aligned!(A, [ITEM; COUNT]);
        let elem_offset = offset + elem_padding + elem_size;
        <[ITEM; COUNT]>::deserialize(buffer, elem_offset, result);
    }
}

impl<
        E: ByteOrder,
        const A: usize,
        const COUNT: usize,
        ITEM: Sized + Clone + Serializable<E, A, ITEM>,
    > SimpleEncoder<E, A, Vec<[ITEM; COUNT]>> for Vec<[ITEM; COUNT]>
where
    [ITEM; COUNT]: Default, // + SimpleEncoder<E, A, [ITEM; COUNT]>
{
    fn encode<W: WritableBuffer<E>>(&self, buffer: &mut W, offset: usize) {
        let elem_padding = fixed_type_size_aligned_padding!(A, [ITEM; COUNT]);
        let elem_size = fixed_type_size_aligned!(A, [ITEM; COUNT]);
        let bytes_len = elem_size * self.len();
        buffer.fill_bytes(offset, bytes_len, 0);
        for (i, v) in self.iter().enumerate() {
            let elem_offset = offset + elem_padding + i * elem_size;
            <[ITEM; COUNT]>::serialize(v, buffer, elem_offset);
        }
        if_align_default_then!(A, {}, {
            let padding_count = dynamic_size_aligned_padding!(A, bytes_len);
            buffer.fill_bytes(offset + self.len() * elem_size, padding_count, 0);
        });
    }

    fn decode<B: ReadableBuffer<E>>(buffer: &B, offset: usize, result: &mut Self) {
        let elem_padding = fixed_type_size_aligned_padding!(A, [ITEM; COUNT]);
        let elem_size = fixed_type_size_aligned!(A, [ITEM; COUNT]);
        let elem_count = (buffer.len() - offset) / elem_size;
        result.resize(elem_count, <[ITEM; COUNT]>::default());
        for i in 0..elem_count {
            let elem_offset = offset + elem_padding + i * elem_size;
            <[ITEM; COUNT]>::deserialize(buffer, elem_offset, &mut result[i]);
        }
    }
}

impl<E: ByteOrder, const A: usize, T: Sized + SimpleEncoder<E, A, T>> FieldEncoder<E, A, Vec<T>>
    for Vec<T>
where
    Vec<T>: SimpleEncoder<E, A, Vec<T>>,
{
    const HEADER_ITEM_SIZE: usize = header_item_size!(A);
    const HEADER_SIZE: usize = header_size!(Self::HEADER_ITEM_SIZE, 3);

    // encode format: header(elems_count, data_offset, data_size) data(bytes)

    fn encode<W: WritableBuffer<E>>(&self, buffer: &mut W, offset: usize) {
        let elems_count = self.len();
        let data_size = elems_count * size_of!(T);
        let data_offset = buffer.len();
        let mut header_item_offset = offset;
        let header_item_size = field_encoder_const_val!(Self, E, A, HEADER_ITEM_SIZE);
        simple_encoder_encode!(
            u32,
            SimpleEncoder,
            E,
            A,
            buffer,
            header_item_offset,
            &(self.len() as u32)
        );
        header_item_offset += header_item_size;
        simple_encoder_encode!(
            u32,
            SimpleEncoder,
            E,
            A,
            buffer,
            header_item_offset,
            &(data_offset as u32)
        );
        header_item_offset += field_encoder_const_val!(Self, E, A, HEADER_ITEM_SIZE);
        simple_encoder_encode!(
            u32,
            SimpleEncoder,
            E,
            A,
            buffer,
            header_item_offset,
            &(data_size as u32)
        );
        header_item_offset += field_encoder_const_val!(Self, E, A, HEADER_ITEM_SIZE);
        <Self as SimpleEncoder<E, A, Self>>::encode(self, buffer, buffer.len());
    }

    fn decode(buffer: &ReadableBufferImpl<E>, offset: usize, result: &mut Self) {
        let mut header_item_offset = offset;
        let header_item_size = field_encoder_const_val!(Self, E, A, HEADER_ITEM_SIZE);
        let mut elems_count = 0u32;
        let mut data_offset = 0u32;
        let mut data_size = 0u32;
        simple_encoder_decode!(
            u32,
            SimpleEncoder,
            E,
            A,
            buffer,
            header_item_offset,
            &mut elems_count
        );
        header_item_offset += header_item_size;
        simple_encoder_decode!(
            u32,
            SimpleEncoder,
            E,
            A,
            buffer,
            header_item_offset,
            &mut data_offset
        );
        header_item_offset += field_encoder_const_val!(Self, E, A, HEADER_ITEM_SIZE);
        simple_encoder_decode!(
            u32,
            SimpleEncoder,
            E,
            A,
            buffer,
            header_item_offset,
            &mut data_size
        );
        header_item_offset += field_encoder_const_val!(Self, E, A, HEADER_ITEM_SIZE);
        <Self as SimpleEncoder<E, A, Self>>::decode(buffer, data_offset as usize, result);
    }
}
