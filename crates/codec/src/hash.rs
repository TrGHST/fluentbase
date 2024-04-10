use alloc::vec::Vec;
use core::hash::Hash;

use byteorder::ByteOrder;
use hashbrown::{HashMap, HashSet};

use crate::{buffer::WritableBuffer, header_item_size, BufferDecoder, BufferEncoder, Encoder};

impl<
        E: ByteOrder,
        const A: usize,
        K: Default + Sized + Encoder<E, A, K> + Eq + Hash + Ord,
        V: Default + Sized + Encoder<E, A, V>,
    > Encoder<E, A, HashMap<K, V>> for HashMap<K, V>
{
    // length + keys (bytes) + values (bytes)
    const HEADER_SIZE: usize =
        header_item_size!(A) + header_item_size!(A) * 2 + header_item_size!(A) * 2;

    fn encode<W: WritableBuffer<E, A>>(&self, encoder: &mut W, field_offset: usize) {
        // encode length
        encoder.write_u32(field_offset, self.len() as u32);
        // make sure keys & values are sorted
        let mut entries: Vec<_> = self.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        // encode keys
        let mut key_encoder = BufferEncoder::<E, A>::new(K::HEADER_SIZE * self.len(), None);
        for (i, obj) in entries.iter().enumerate() {
            obj.0.encode(&mut key_encoder, K::HEADER_SIZE * i);
        }
        encoder.write_bytes(
            field_offset + header_item_size!(A),
            key_encoder.finalize().as_slice(),
        );
        // encode values
        let mut value_encoder = BufferEncoder::<E, A>::new(V::HEADER_SIZE * self.len(), None);
        for (i, obj) in entries.iter().enumerate() {
            obj.1.encode(&mut value_encoder, V::HEADER_SIZE * i);
        }
        encoder.write_bytes(
            field_offset + header_item_size!(A) * 3,
            value_encoder.finalize().as_slice(),
        );
    }

    fn decode_header(
        decoder: &mut BufferDecoder<E, A>,
        field_offset: usize,
        result: &mut HashMap<K, V>,
    ) -> (usize, usize) {
        // read length and reserve required capacity in hashmap
        let length = decoder.read_u32(field_offset) as usize;
        result.reserve(length);
        // read bytes header to calculate hint
        let (keys_offset, keys_length) =
            decoder.read_bytes_header(field_offset + header_item_size!(A));
        let (_, values_length) = decoder.read_bytes_header(field_offset + header_item_size!(A) * 3);
        // sum of keys and values are total body length
        (keys_offset, keys_length + values_length)
    }

    fn decode_body(
        decoder: &mut BufferDecoder<E, A>,
        field_offset: usize,
        result: &mut HashMap<K, V>,
    ) {
        // decode length, keys and values
        let length = decoder.read_u32(field_offset) as usize;
        let (key_bytes, value_bytes) = decoder.read_bytes2(
            field_offset + header_item_size!(A),
            field_offset + header_item_size!(A) * 3,
        );
        // decode keys
        let mut key_decoder = BufferDecoder::new(key_bytes);
        let keys = (0..length).map(|i| {
            let mut result = Default::default();
            K::decode_body(&mut key_decoder, K::HEADER_SIZE * i, &mut result);
            result
        });
        // decode values
        let mut value_decoder = BufferDecoder::new(value_bytes);
        let values = (0..length).map(|i| {
            let mut result = Default::default();
            V::decode_body(&mut value_decoder, V::HEADER_SIZE * i, &mut result);
            result
        });
        // zip into map
        *result = keys.zip(values).collect()
    }
}

impl<E: ByteOrder, const A: usize, T: Default + Sized + Encoder<E, A, T> + Eq + Hash + Ord>
    Encoder<E, A, HashSet<T>> for HashSet<T>
{
    // length + keys (bytes)
    const HEADER_SIZE: usize = header_item_size!(A) + header_item_size!(A) * 2; // 4 + 8

    fn encode<W: WritableBuffer<E, A>>(&self, encoder: &mut W, field_offset: usize) {
        // encode length
        encoder.write_u32(field_offset, self.len() as u32);
        // make sure set is sorted
        let mut entries: Vec<_> = self.iter().collect();
        entries.sort();
        // encode values
        let mut value_encoder = BufferEncoder::<E, A>::new(T::HEADER_SIZE * self.len(), None);
        for (i, obj) in entries.iter().enumerate() {
            obj.encode(&mut value_encoder, T::HEADER_SIZE * i);
        }
        encoder.write_bytes(field_offset + 4, value_encoder.finalize().as_slice());
    }

    fn decode_header(
        decoder: &mut BufferDecoder<E, A>,
        field_offset: usize,
        result: &mut HashSet<T>,
    ) -> (usize, usize) {
        // read set size and reserve required memory
        let length = decoder.read_u32(field_offset) as usize;
        result.reserve(length);
        // read bytes header
        let (value_offset, value_length) =
            decoder.read_bytes_header(field_offset + header_item_size!(A));
        (value_offset, value_length)
    }

    fn decode_body(
        decoder: &mut BufferDecoder<E, A>,
        field_offset: usize,
        result: &mut HashSet<T>,
    ) {
        // decode length, keys and values
        let length = decoder.read_u32(field_offset) as usize;
        let value_bytes = decoder.read_bytes(field_offset + header_item_size!(A));
        // decode values
        let mut value_decoder = BufferDecoder::new(value_bytes);
        let values = (0..length).map(|i| {
            let mut result = Default::default();
            T::decode_body(&mut value_decoder, T::HEADER_SIZE * i, &mut result);
            result
        });
        // zip into map
        *result = values.collect()
    }
}
