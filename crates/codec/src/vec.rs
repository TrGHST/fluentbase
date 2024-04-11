use crate::{buffer::WritableBuffer, header_item_size, BufferDecoder, BufferEncoder, Encoder};
use alloc::vec::Vec;
use byteorder::ByteOrder;

///
/// We encode dynamic arrays as following:
/// - header
/// - + length - number of elements inside vector
/// - + offset - offset inside structure
/// - + size - number of encoded bytes
/// - body
/// - + raw bytes of the vector
///
/// We don't encode empty vectors, instead we store 0 as length,
/// it helps to reduce empty vector size from 12 to 4 bytes.
impl<E: ByteOrder, const A: usize, T: Default + Sized + Encoder<E, A, T>> Encoder<E, A, Vec<T>>
    for Vec<T>
{
    // u32: length + values (bytes)
    const HEADER_SIZE: usize = header_item_size!(A) * 3;

    fn encode<W: WritableBuffer<E>>(&self, encoder: &mut W, field_offset: usize) {
        encoder.write_u32(field_offset, self.len() as u32);
        let size_of_t = core::mem::size_of::<T>();
        let mut value_encoder = BufferEncoder::<E, A>::new(size_of_t * self.len(), None);
        for (i, obj) in self.iter().enumerate() {
            obj.encode(&mut value_encoder, i * size_of_t);
        }
        encoder.write_bytes(
            field_offset + header_item_size!(A),
            value_encoder.finalize().as_slice(),
        );
    }

    fn decode_header(
        decoder: &mut BufferDecoder<E>,
        field_offset: usize,
        result: &mut Vec<T>,
    ) -> (usize, usize) {
        let count = decoder.read_u32(field_offset) as usize;
        if count > result.capacity() {
            result.reserve(count - result.capacity());
        }
        let (offset, length) = decoder.read_bytes_header(field_offset + header_item_size!(A));
        (offset, length)
    }

    fn decode_body(decoder: &mut BufferDecoder<E>, field_offset: usize, result: &mut Vec<T>) {
        let input_len = decoder.read_u32(field_offset) as usize;
        if input_len == 0 {
            result.clear();
            return;
        }
        let input_bytes = decoder.read_bytes(field_offset + header_item_size!(A));
        let mut value_decoder = BufferDecoder::new(input_bytes);
        *result = (0..input_len)
            .map(|i| {
                let mut result = T::default();
                T::decode_body(&mut value_decoder, T::HEADER_SIZE * i, &mut result);
                result
            })
            .collect()
    }
}
