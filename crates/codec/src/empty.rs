use crate::encoder::{ALIGNMENT_DEFAULT, HEADER_ITEM_SIZE_DEFAULT};
use crate::{BufferDecoder, Encoder, WritableBuffer};
use byteorder::ByteOrder;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct EmptyVec<const A: usize>;

impl<E: ByteOrder, const A: usize> Encoder<E, A, EmptyVec<A>> for EmptyVec<A> {
    const HEADER_SIZE: usize = A * 3;

    fn encode<W: WritableBuffer<E, A>>(&self, encoder: &mut W, field_offset: usize) {
        // first 4 bytes are number of elements
        encoder.write_u32(field_offset, 0);
        // remaining 4+4 are offset and length
        let header_item_size = if A != ALIGNMENT_DEFAULT {
            A
        } else {
            HEADER_ITEM_SIZE_DEFAULT
        };
        encoder.write_bytes(field_offset + header_item_size, &[]);
    }

    fn decode_header(
        decoder: &mut BufferDecoder<E, A>,
        field_offset: usize,
        _result: &mut EmptyVec<A>,
    ) -> (usize, usize) {
        let count = decoder.read_u32(field_offset);
        debug_assert_eq!(count, 0);
        let header_item_size = if A != ALIGNMENT_DEFAULT {
            A
        } else {
            HEADER_ITEM_SIZE_DEFAULT
        };
        decoder.read_bytes_header(field_offset + header_item_size)
    }
}
