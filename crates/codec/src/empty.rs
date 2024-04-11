use byteorder::ByteOrder;

use crate::{header_item_size, BufferDecoder, Encoder, WritableBuffer};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct EmptyVec;

impl<E: ByteOrder, const A: usize> Encoder<E, A, EmptyVec> for EmptyVec {
    const HEADER_SIZE: usize = header_item_size!(A) * 3;

    fn encode<W: WritableBuffer<E>>(&self, encoder: &mut W, field_offset: usize) {
        // first 4 bytes are number of elements
        encoder.write_u32(field_offset, 0);
        // remaining 4+4 are offset and length
        encoder.write_bytes(field_offset + header_item_size!(A), &[]);
    }

    fn decode_header(
        decoder: &mut BufferDecoder<E>,
        field_offset: usize,
        _result: &mut EmptyVec,
    ) -> (usize, usize) {
        let count = decoder.read_u32(field_offset);
        debug_assert_eq!(count, 0);
        decoder.read_bytes_header(field_offset + header_item_size!(A))
    }
}
