use alloc::vec::Vec;

use byteorder::{BE, LE};

use crate::buffer::ReadableBuffer;
use crate::encoder::{FieldEncoder, SimpleEncoder, ALIGN_32, ALIGN_DEFAULT};
use crate::{
    dynamic_buffer_decode, dynamic_buffer_encode, dynamic_size_aligned, field_encoder_const_val,
    header_item_size, header_size, size_aligned, DynamicBuffer,
};

#[test]
fn test_simple_encoder_le_ad_vec_u8() {
    type End = LE;
    const ALIGN: usize = ALIGN_DEFAULT;
    type V1ItemType = u8;
    type V1Type = Vec<V1ItemType>;
    let v1_in: V1Type = Vec::from([0, 1, 2, 3, 4, 5, 6, 7]);
    let offset = 0;
    let header_len = dynamic_size_aligned!(ALIGN, v1_in.len());
    let mut buffer = DynamicBuffer::<End>::new(header_len, None);
    dynamic_buffer_encode!(
        SimpleEncoder,
        End,
        ALIGN,
        V1Type,
        DynamicBuffer,
        &mut buffer,
        offset,
        &v1_in
    );
    let encoded_value = buffer.finalize();
    let expected = "0001020304050607";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer = ReadableBuffer::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();
    dynamic_buffer_decode!(
        SimpleEncoder,
        End,
        ALIGN,
        V1Type,
        &mut buffer,
        offset,
        &mut v1_out
    );
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_simple_encoder_be_a32_vec_u8() {
    type End = BE;
    const ALIGN: usize = ALIGN_32;
    type V1ItemType = u8;
    type V1Type = Vec<V1ItemType>;
    let mut v1_in: V1Type = Vec::from([0, 1, 2, 3, 4, 5, 6, 7]);
    let offset = 0;
    let header_len = dynamic_size_aligned!(ALIGN, v1_in.len());
    let mut buffer = DynamicBuffer::<End>::new(header_len, None);
    dynamic_buffer_encode!(
        SimpleEncoder,
        End,
        ALIGN,
        V1Type,
        DynamicBuffer,
        &mut buffer,
        offset,
        &v1_in
    );
    let encoded_value = buffer.finalize();
    let expected = "0001020304050607000000000000000000000000000000000000000000000000";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer = ReadableBuffer::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();
    dynamic_buffer_decode!(
        SimpleEncoder,
        End,
        ALIGN,
        V1Type,
        &mut buffer,
        offset,
        &mut v1_out
    );
    v1_in.resize(size_aligned!(ALIGN, v1_in.len()), 0);
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_simple_encoder_le_ad_vec_u16() {
    type End = LE;
    const ALIGN: usize = ALIGN_DEFAULT;
    type V1ItemType = u16;
    type V1Type = Vec<V1ItemType>;
    let v1_in: V1Type = Vec::from([0, 1, 2, 3, 4, 5, 6, 7]);
    let offset = 0;
    let header_len = dynamic_size_aligned!(ALIGN, v1_in.len());
    let mut buffer = DynamicBuffer::<End>::new(header_len, None);
    dynamic_buffer_encode!(
        SimpleEncoder,
        End,
        ALIGN,
        V1Type,
        DynamicBuffer,
        &mut buffer,
        offset,
        &v1_in
    );
    let encoded_value = buffer.finalize();
    let expected = "\
    0000\
    0100\
    0200\
    0300\
    0400\
    0500\
    0600\
    0700\
    ";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer = ReadableBuffer::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();
    dynamic_buffer_decode!(
        SimpleEncoder,
        End,
        ALIGN,
        V1Type,
        &mut buffer,
        offset,
        &mut v1_out
    );
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_simple_encoder_be_a32_vec_u16() {
    type End = BE;
    const ALIGN: usize = ALIGN_32;
    type V1ItemType = u16;
    type V1Type = Vec<V1ItemType>;
    let mut v1_in: V1Type = Vec::from([0, 1, 2, 3, 4, 5, 6, 7]);
    let offset = 0;
    let header_len = dynamic_size_aligned!(ALIGN, v1_in.len());
    let mut buffer = DynamicBuffer::<End>::new(header_len, None);
    dynamic_buffer_encode!(
        SimpleEncoder,
        End,
        ALIGN,
        V1Type,
        DynamicBuffer,
        &mut buffer,
        offset,
        &v1_in
    );
    let encoded_value = buffer.finalize();
    let expected = "0000000100020003000400050006000700000000000000000000000000000000";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer = ReadableBuffer::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();
    dynamic_buffer_decode!(
        SimpleEncoder,
        End,
        ALIGN,
        V1Type,
        &mut buffer,
        offset,
        &mut v1_out
    );
    let v1_in_len_aligned = dynamic_size_aligned!(ALIGN, v1_in.len(), V1ItemType);
    v1_in.resize(v1_in_len_aligned, 0);
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_simple_encoder_le_ad_vec_of_fixed() {
    type End = BE;
    const ALIGN: usize = ALIGN_DEFAULT;
    type V1ItemItemType = u16;
    type V1ItemType = [V1ItemItemType; 7];
    type V1Type = Vec<V1ItemType>;
    let v1_in: V1Type = Vec::from([[0, 1, 2, 3, 4, 5, 6], [7, 8, 9, 10, 11, 12, 13]]);
    let offset = 0;
    let header_len = dynamic_size_aligned!(ALIGN, v1_in.len());
    let mut buffer = DynamicBuffer::<End>::new(header_len, None);
    dynamic_buffer_encode!(
        SimpleEncoder,
        End,
        ALIGN,
        V1Type,
        DynamicBuffer,
        &mut buffer,
        offset,
        &v1_in
    );
    let encoded_value = buffer.finalize();
    let expected = "0000000100020003000400050006000700080009000a000b000c000d";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer = ReadableBuffer::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();
    dynamic_buffer_decode!(
        SimpleEncoder,
        End,
        ALIGN,
        V1Type,
        &mut buffer,
        offset,
        &mut v1_out
    );
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_field_encoder_be_ad_vec_u8() {
    type End = BE;
    const ALIGN: usize = ALIGN_DEFAULT;
    type V1ItemType = u8;
    type V1Type = Vec<V1ItemType>;
    let v1_in: V1Type = Vec::from([0, 1, 2, 3, 4, 5, 6, 7]);
    let offset = 0;
    let header_item_size = header_item_size!(ALIGN);
    let header_size = header_size!(ALIGN, 3);
    assert_eq!(
        header_size,
        field_encoder_const_val!(V1Type, End, ALIGN, HEADER_SIZE)
    );
    let mut buffer = DynamicBuffer::<End>::new(header_size, None);
    <V1Type as FieldEncoder<End, ALIGN, V1Type>>::encode(&v1_in, &mut buffer, offset);
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
    let mut v1_out: V1Type = Vec::new();
    <V1Type as FieldEncoder<End, ALIGN, V1Type>>::decode(&buffer, offset, &mut v1_out);
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_field_encoder_be_a32_vec_u8() {
    type End = BE;
    const ALIGN: usize = ALIGN_32;
    type V1ItemType = u8;
    type V1Type = Vec<V1ItemType>;
    let mut v1_in: V1Type = Vec::from([0, 1, 2, 3, 4, 5, 6, 7]);
    let offset = 0;
    let header_item_size = header_item_size!(ALIGN);
    let header_size = header_size!(ALIGN, 3);
    assert_eq!(
        header_size,
        field_encoder_const_val!(V1Type, End, ALIGN, HEADER_SIZE)
    );
    let mut buffer = DynamicBuffer::<End>::new(header_size, None);
    <V1Type as FieldEncoder<End, ALIGN, V1Type>>::encode(&v1_in, &mut buffer, offset);
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
    let mut v1_out: V1Type = Vec::new();
    <V1Type as FieldEncoder<End, ALIGN, V1Type>>::decode(&buffer, offset, &mut v1_out);
    let v1_in_len_aligned = dynamic_size_aligned!(ALIGN, v1_in.len(), V1ItemType);
    v1_in.resize(v1_in_len_aligned, 0);
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_field_encoder_be_ad_vec_u16() {
    type End = BE;
    const ALIGN: usize = ALIGN_DEFAULT;
    type V1ItemType = u16;
    type V1Type = Vec<V1ItemType>;
    let mut v1_in: V1Type = Vec::from([0, 1, 2, 3, 4, 5, 6, 7]);
    let offset = 0;
    let header_item_size = header_item_size!(ALIGN);
    let header_size = header_size!(ALIGN, 3);
    assert_eq!(
        header_size,
        field_encoder_const_val!(V1Type, End, ALIGN, HEADER_SIZE)
    );
    let mut buffer = DynamicBuffer::<End>::new(header_size, None);
    <V1Type as FieldEncoder<End, ALIGN, V1Type>>::encode(&v1_in, &mut buffer, offset);
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
    00000010\
    00000001000200030004000500060007\
    ";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let buffer = ReadableBuffer::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();
    <V1Type as FieldEncoder<End, ALIGN, V1Type>>::decode(&buffer, offset, &mut v1_out);
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_field_encoder_be_a32_vec_u16() {
    type End = BE;
    const ALIGN: usize = ALIGN_32;
    type V1ItemType = u16;
    type V1Type = Vec<V1ItemType>;
    let mut v1_in: V1Type = Vec::from([0, 1, 2, 3, 4, 5, 6, 7]);
    let offset = 0;
    let header_item_size = header_item_size!(ALIGN);
    let header_size = header_size!(ALIGN, 3);
    assert_eq!(
        header_size,
        field_encoder_const_val!(V1Type, End, ALIGN, HEADER_SIZE)
    );
    let mut buffer = DynamicBuffer::<End>::new(header_size, None);
    <V1Type as FieldEncoder<End, ALIGN, V1Type>>::encode(&v1_in, &mut buffer, offset);
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
    0000000000000000000000000000000000000000000000000000000000000010\
    0000000100020003000400050006000700000000000000000000000000000000\
    ";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let buffer = ReadableBuffer::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();
    <V1Type as FieldEncoder<End, ALIGN, V1Type>>::decode(&buffer, offset, &mut v1_out);
    let v1_in_len_aligned = dynamic_size_aligned!(ALIGN, v1_in.len(), V1ItemType);
    v1_in.resize(v1_in_len_aligned, 0);
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_field_encoder_be_ad_vec_u16_empty() {
    type End = BE;
    const ALIGN: usize = ALIGN_DEFAULT;
    type V1ItemType = u16;
    type V1Type = Vec<V1ItemType>;
    let mut v1_in: V1Type = Vec::from([]);
    let offset = 0;
    let header_item_size = header_item_size!(ALIGN);
    let header_size = header_size!(ALIGN, 3);
    assert_eq!(
        header_size,
        field_encoder_const_val!(V1Type, End, ALIGN, HEADER_SIZE)
    );
    let mut buffer = DynamicBuffer::<End>::new(header_size, None);
    <V1Type as FieldEncoder<End, ALIGN, V1Type>>::encode(&v1_in, &mut buffer, offset);
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
    00000000\
    0000000c\
    00000000\
    ";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let buffer = ReadableBuffer::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();
    <V1Type as FieldEncoder<End, ALIGN, V1Type>>::decode(&buffer, offset, &mut v1_out);
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_field_encoder_be_a32_vec_u16_empty() {
    type End = BE;
    const ALIGN: usize = ALIGN_32;
    type V1ItemType = u16;
    type V1Type = Vec<V1ItemType>;
    let mut v1_in: V1Type = Vec::from([]);
    let offset = 0;
    let header_item_size = header_item_size!(ALIGN);
    let header_size = header_size!(ALIGN, 3);
    assert_eq!(
        header_size,
        field_encoder_const_val!(V1Type, End, ALIGN, HEADER_SIZE)
    );
    let mut buffer = DynamicBuffer::<End>::new(header_size, None);
    <V1Type as FieldEncoder<End, ALIGN, V1Type>>::encode(&v1_in, &mut buffer, offset);
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
    0000000000000000000000000000000000000000000000000000000000000000\
    0000000000000000000000000000000000000000000000000000000000000060\
    0000000000000000000000000000000000000000000000000000000000000000\
    ";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let buffer = ReadableBuffer::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();
    <V1Type as FieldEncoder<End, ALIGN, V1Type>>::decode(&buffer, offset, &mut v1_out);
    let v1_in_len_aligned = dynamic_size_aligned!(ALIGN, v1_in.len(), V1ItemType);
    v1_in.resize(v1_in_len_aligned, 0);
    assert_eq!(v1_in, v1_out);
}

// #[test]
// fn test_field_encoder_be_ad_vec_of_fixed16() {
//     type End = BE;
//     const ALIGN: usize = ALIGN_DEFAULT;
//     type V1ItemItemType = u8;
//     type V1ItemType = [V1ItemItemType; 7];
//     type V1Type = Vec<V1ItemType>;
//     let mut v1_in: V1Type = Vec::from([[1, 2, 3, 4, 5, 6, 7]]);
//     let offset = 0;
//     let header_item_size = header_item_size!(ALIGN);
//     let header_size = header_size!(ALIGN, 3);
//     assert_eq!(
//         header_size,
//         // field_encoder_const_val!(V1Type, End, ALIGN, HEADER_SIZE),
//         <V1Type as FieldEncoder<End, ALIGN, V1Type>>::HEADER_SIZE,
//     );
//     let mut buffer = DynamicBuffer::<End>::new(header_size, None);
//     <V1Type as FieldEncoder<End, ALIGN, V1Type>>::encode(&v1_in, &mut buffer, offset);
//     let encoded_value = buffer.finalize();
//     for (i, v) in encoded_value
//         .as_slice()
//         .chunks(header_item_size)
//         .enumerate()
//     {
//         let chunk_encoded = hex::encode(v);
//         println!("fact chunk {i}: {chunk_encoded}")
//     }
//     let expected = "\
//     00000000\
//     0000000c\
//     00000000\
//     ";
//     let fact = hex::encode(&encoded_value);
//     assert_eq!(expected, fact);
//     let buffer = ReadableBuffer::<End>::new(&encoded_value);
//     let mut v1_out: V1Type = Vec::new();
//     <V1Type as FieldEncoder<End, ALIGN, V1Type>>::decode(&buffer, offset, &mut v1_out);
//     assert_eq!(v1_in, v1_out);
// }

// #[test]
// fn test_field_encoder_be_a32_vec_of_fixed16() {
//     type End = BE;
//     const ALIGN: usize = ALIGN_32;
//     type V1ItemType = u16;
//     type V1Type = Vec<V1ItemType>;
//     let mut v1_in: V1Type = Vec::from([]);
//     let offset = 0;
//     let header_item_size = header_item_size!(ALIGN);
//     let header_size = header_size!(ALIGN, 3);
//     assert_eq!(
//         header_size,
//         field_encoder_const_val!(V1Type, End, ALIGN, HEADER_SIZE)
//     );
//     let mut buffer = DynamicBuffer::<End>::new(header_size, None);
//     <V1Type as FieldEncoder<End, ALIGN, V1Type>>::encode(&v1_in, &mut buffer, offset);
//     let encoded_value = buffer.finalize();
//     for (i, v) in encoded_value
//         .as_slice()
//         .chunks(header_item_size)
//         .enumerate()
//     {
//         let chunk_encoded = hex::encode(v);
//         println!("fact chunk {i}: {chunk_encoded}")
//     }
//     let expected = "\
//     0000000000000000000000000000000000000000000000000000000000000000\
//     0000000000000000000000000000000000000000000000000000000000000060\
//     0000000000000000000000000000000000000000000000000000000000000000\
//     ";
//     let fact = hex::encode(&encoded_value);
//     assert_eq!(expected, fact);
//     let buffer = ReadableBuffer::<End>::new(&encoded_value);
//     let mut v1_out: V1Type = Vec::new();
//     <V1Type as FieldEncoder<End, ALIGN, V1Type>>::decode(&buffer, offset, &mut v1_out);
//     let v1_in_len_aligned = dynamic_size_aligned!(ALIGN, v1_in.len(), V1ItemType);
//     v1_in.resize(v1_in_len_aligned, 0);
//     assert_eq!(v1_in, v1_out);
// }
