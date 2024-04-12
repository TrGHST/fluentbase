use alloc::vec::Vec;

use byteorder::BE;

use crate::buffer::ReadableBuffer;
use crate::encoder::{FieldEncoder, SimpleEncoder, ALIGN_32, ALIGN_DEFAULT};
use crate::{field_encoder_const_val, header_item_size, header_size, DynamicBuffer};

#[test]
fn test_field_encoder_be_a32_vec_u8() {
    type End = BE;
    const ALIGN: usize = ALIGN_32;
    type VType1 = Vec<u8>;
    let v1_in: VType1 = Vec::from([0, 1, 2, 3, 4, 5, 6, 7]);
    let offset = 0;
    let header_item_size = header_item_size!(ALIGN);
    let header_size = header_size!(ALIGN, 3);
    assert_eq!(
        header_size,
        field_encoder_const_val!(VType1, End, ALIGN, HEADER_SIZE)
    );
    let mut buffer = DynamicBuffer::<End>::new(header_size, None);
    <VType1 as FieldEncoder<End, ALIGN, VType1>>::encode(&v1_in, &mut buffer, offset);
    let encoded_value = buffer.finalize();
    for (i, v) in encoded_value
        .as_slice()
        .chunks(header_item_size)
        .enumerate()
    {
        let chunk_encoded = hex::encode(v);
        println!("fact chunk {i}: {chunk_encoded}")
    }
    let expected = "\
    0000000000000000000000000000000000000000000000000000000000000008\
    0000000000000000000000000000000000000000000000000000000000000060\
    0000000000000000000000000000000000000000000000000000000000000008\
    0001020304050607000000000000000000000000000000000000000000000000\
    ";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let buffer = ReadableBuffer::<End>::new(&encoded_value);
    let mut v1_out: VType1 = Vec::new();
    <VType1 as FieldEncoder<End, ALIGN, VType1>>::decode(&buffer, offset, &mut v1_out);
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_field_encoder_be_ad_vec_u8() {
    type End = BE;
    const ALIGN: usize = ALIGN_DEFAULT;
    type VType1 = Vec<u8>;
    let v1_in: VType1 = Vec::from([0, 1, 2, 3, 4, 5, 6, 7]);
    let offset = 0;
    let header_item_size = header_item_size!(ALIGN);
    let header_size = header_size!(ALIGN, 3);
    assert_eq!(
        header_size,
        field_encoder_const_val!(VType1, End, ALIGN, HEADER_SIZE)
    );
    let mut buffer = DynamicBuffer::<End>::new(header_size, None);
    <VType1 as FieldEncoder<End, ALIGN, VType1>>::encode(&v1_in, &mut buffer, offset);
    let encoded_value = buffer.finalize();
    for (i, v) in encoded_value
        .as_slice()
        .chunks(header_item_size)
        .enumerate()
    {
        let chunk_encoded = hex::encode(v);
        println!("fact chunk {i}: {chunk_encoded}")
    }
    let expected = "\
    00000008\
    0000000c\
    00000008\
    0001020304050607\
    ";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let buffer = ReadableBuffer::<End>::new(&encoded_value);
    let mut v1_out: VType1 = Vec::new();
    <VType1 as FieldEncoder<End, ALIGN, VType1>>::decode(&buffer, offset, &mut v1_out);
    assert_eq!(v1_in, v1_out);
}
