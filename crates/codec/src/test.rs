use crate::{BufferDecoder, Encoder, FieldEncoder, WritableBuffer};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ComplicatedType {
    pub values1: Vec<u8>,
    pub a1: bool,
    pub values2: Vec<u8>,
    pub a2: bool
}

impl<E: ::byteorder::ByteOrder, const A: usize> Encoder<E, A, ComplicatedType> for ComplicatedType {
    const HEADER_SIZE: usize = <Vec<u8> as Encoder<E, A, Vec<u8>>>::HEADER_SIZE + <bool as Encoder<E, A, bool>>::HEADER_SIZE + <Vec<u8> as Encoder<E, A, Vec<u8>>>::HEADER_SIZE + <bool as Encoder<E, A, bool>>::HEADER_SIZE;
    fn encode<W: WritableBuffer<E>>(&self, encoder: &mut W, mut field_offset: usize) {
        <Vec<u8> as Encoder<E, A, Vec<u8>>>::encode(&self.values1, encoder, field_offset);
        field_offset += <Vec<u8> as Encoder<E, A, Vec<u8>>>::HEADER_SIZE;
        <bool as Encoder<E, A, bool>>::encode(&self.a1, encoder, field_offset);
        field_offset += <bool as Encoder<E, A, bool>>::HEADER_SIZE;
        <Vec<u8> as Encoder<E, A, Vec<u8>>>::encode(&self.values2, encoder, field_offset);
        field_offset += <Vec<u8> as Encoder<E, A, Vec<u8>>>::HEADER_SIZE;
        <bool as Encoder<E, A, bool>>::encode(&self.a2, encoder, field_offset);
    }
    fn decode_header(decoder: &mut BufferDecoder<E>, mut field_offset: usize, result: &mut ComplicatedType) -> (usize, usize) {
        <Vec<u8> as Encoder<E, A, Vec<u8>>>::decode_body(decoder, field_offset, &mut result.values1);
        field_offset += <Vec<u8> as Encoder<E, A, Vec<u8>>>::HEADER_SIZE;
        <bool as Encoder<E, A, bool>>::decode_body(decoder, field_offset, &mut result.a1);
        field_offset += <bool as Encoder<E, A, bool>>::HEADER_SIZE;
        <Vec<u8> as Encoder<E, A, Vec<u8>>>::decode_body(decoder, field_offset, &mut result.values2);
        field_offset += <Vec<u8> as Encoder<E, A, Vec<u8>>>::HEADER_SIZE;
        <bool as Encoder<E, A, bool>>::decode_body(decoder, field_offset, &mut result.a2);
        (0, 0)
    }
}

impl From<Vec<u8>> for ComplicatedType {
    fn from(value: Vec<u8>) -> Self {
        let mut result = Self::default();
        let mut buffer_decoder = BufferDecoder::<E>::new(value.as_slice());
        <ComplicatedType as Encoder<E, A, ComplicatedType>>::decode_body(&mut buffer_decoder, 0, &mut result);
        result
    }
}

pub trait IComplicatedType {
    type Values1;
    type A1;
    type Values2;
    type A2;
}

impl IComplicatedType for ComplicatedType {
    type Values1 = FieldEncoder<E, A, Vec<u8>, { 0 }>;
    type A1 = FieldEncoder<E, A, bool, { (0 + <Vec<u8> as Encoder<E, A, Vec<u8>>>::HEADER_SIZE) }>;
    type Values2 = FieldEncoder<E, A, Vec<u8>, { ((0 + <Vec<u8> as Encoder<E, A, Vec<u8>>>::HEADER_SIZE) + <bool as Encoder<E, A, bool>>::HEADER_SIZE) }>;
    type A2 = FieldEncoder<E, A, bool, { (((0 + <Vec<u8> as Encoder<E, A, Vec<u8>>>::HEADER_SIZE) + <bool as Encoder<E, A, bool>>::HEADER_SIZE) + <Vec<u8> as Encoder<E, A, Vec<u8>>>::HEADER_SIZE) }>;
}