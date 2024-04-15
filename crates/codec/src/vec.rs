use alloc::vec::Vec;

use byteorder::ByteOrder;

use crate::buffer::ReadableBuffer;
use crate::encoder::{FieldEncoder, SimpleEncoder, ALIGN_DEFAULT};
use crate::{
    buffer::WritableBuffer, dynamic_size_aligned_padding, field_encoder_const_val,
    header_item_size, header_size, simple_encoder_decode, simple_encoder_encode,
};

macro_rules! impl_simple_encoder_vec {
    ($typ:ty) => {
        impl<E: ByteOrder, const A: usize> SimpleEncoder<E, A, Vec<$typ>> for Vec<$typ> {
            fn encode<W: WritableBuffer<E>>(&self, buffer: &mut W, offset: usize) {
                let item_size = $crate::size_of!($typ);
                let len = item_size * self.len();
                buffer.fill_bytes(offset, len, 0);
                for (i, v) in self.iter().enumerate() {
                    <$typ as SimpleEncoder<E, ALIGN_DEFAULT, $typ>>::encode(
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

            fn decode(buffer: &ReadableBuffer<E>, offset: usize, result: &mut Vec<$typ>) {
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

macro_rules! impl_field_encoder_primitive {
    ($typ:ty) => {
        impl<E: ByteOrder, const A: usize> FieldEncoder<E, A, Vec<$typ>> for Vec<$typ> {
            const HEADER_ITEM_SIZE: usize = header_item_size!(A);
            const HEADER_SIZE: usize = header_size!(A, 3);

            fn encode<W: WritableBuffer<E>>(&self, buffer: &mut W, offset: usize) {
                // encode format: header(elems_count, data_offset, data_size) data(bytes)
                let elems_count = self.len();
                let data_size = elems_count * $crate::size_of!($typ);
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

            fn decode(buffer: &ReadableBuffer<E>, offset: usize, result: &mut Self) {
                // encode format: header(elems_count, data_offset, data_size) data(bytes)
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
                let result_tail_offset = offset + data_size as usize;
                let item_size = $crate::size_of!($typ);
                <Self as SimpleEncoder<E, A, Self>>::decode(buffer, data_offset as usize, result);
            }
        }
    };
}

impl_field_encoder_primitive!(u8);
impl_field_encoder_primitive!(u16);
impl_field_encoder_primitive!(u32);
impl_field_encoder_primitive!(u64);
