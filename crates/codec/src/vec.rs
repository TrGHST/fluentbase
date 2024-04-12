use alloc::vec::Vec;

use byteorder::ByteOrder;

use crate::buffer::ReadableBuffer;
use crate::encoder::{FieldEncoder, SimpleEncoder};
use crate::{
    buffer::WritableBuffer, dynamic_size_aligned_padding, field_encoder_const_val,
    header_item_size, header_size, if_align_default_then, simple_encoder_decode,
    simple_encoder_encode,
};

impl<E: ByteOrder, const A: usize> SimpleEncoder<E, A, Vec<u8>> for Vec<u8> {
    fn encode<W: WritableBuffer<E>>(&self, buffer: &mut W, offset: usize) {
        buffer.write_bytes(offset, &self);
        if_align_default_then!(A, {}, {
            let padding_count = dynamic_size_aligned_padding!(A, self.len());
            buffer.fill_bytes(offset + self.len(), padding_count, 0);
        });
    }

    fn decode(decoder: &ReadableBuffer<E>, offset: usize, result: &mut Vec<u8>) {
        if_align_default_then!(
            A,
            {
                *result = decoder.read_bytes(offset, result.len()).to_vec();
            },
            {
                *result = decoder.read_bytes(offset, result.len()).to_vec();
            }
        );
    }
}

impl<E: ByteOrder, const A: usize> FieldEncoder<E, A, Vec<u8>> for Vec<u8> {
    const HEADER_ITEM_SIZE: usize = header_item_size!(A);
    const HEADER_SIZE: usize = header_size!(A, 3);

    fn encode<W: WritableBuffer<E>>(&self, buffer: &mut W, offset: usize) {
        // encode format: header(elems_count, data_offset, data_size) data(bytes)
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
        // let data_offset = offset + field_encoder_const_val!(Self, E, A, HEADER_SIZE);
        let data_offset = buffer.len();
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
        let data_len = self.len();
        simple_encoder_encode!(
            u32,
            SimpleEncoder,
            E,
            A,
            buffer,
            header_item_offset,
            &(data_len as u32)
        );
        header_item_offset += field_encoder_const_val!(Self, E, A, HEADER_ITEM_SIZE);
        simple_encoder_encode!(Self, SimpleEncoder, E, A, buffer, header_item_offset, self);
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
        if (*result).len() < result_tail_offset {
            (*result).resize(result_tail_offset, 0);
        }
        (*result)[offset..result_tail_offset]
            .copy_from_slice(&buffer.read_bytes(header_item_offset, data_size as usize));
    }
}
