use alloc::vec::Vec;

use byteorder::{BE, LE};

use crate::buffer::ReadableBufferImpl;
use crate::encoder::{SimpleEncoder, StructuredEncoder, ALIGN_32, ALIGN_DEFAULT};
use crate::{
    dynamic_size_aligned, header_item_size, header_size, size_aligned, size_of,
    structured_encoder_call, structured_encoder_const_val, writable_buffer_call,
    WritableBufferImpl,
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
    let mut buffer = WritableBufferImpl::<End>::new(header_len, None);
    writable_buffer_call!(@enc
        SimpleEncoder,
        End,
        ALIGN,
        V1Type,
        WritableBufferImpl,
        &mut buffer,
        offset,
        &v1_in
    );
    let encoded_value = buffer.finalize();
    let expected = "0001020304050607";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer = ReadableBufferImpl::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();
    writable_buffer_call!(@dec
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
    let mut buffer = WritableBufferImpl::<End>::new(header_len, None);
    writable_buffer_call!(@enc
        SimpleEncoder,
        End,
        ALIGN,
        V1Type,
        WritableBufferImpl,
        &mut buffer,
        offset,
        &v1_in
    );
    let encoded_value = buffer.finalize();
    let expected = "0001020304050607000000000000000000000000000000000000000000000000";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer = ReadableBufferImpl::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();
    writable_buffer_call!(@dec
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
    let mut buffer = WritableBufferImpl::<End>::new(header_len, None);
    writable_buffer_call!(@enc
        SimpleEncoder,
        End,
        ALIGN,
        V1Type,
        WritableBufferImpl,
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
    let mut buffer = ReadableBufferImpl::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();
    writable_buffer_call!(@dec
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
    let mut buffer = WritableBufferImpl::<End>::new(header_len, None);
    writable_buffer_call!(@enc
        SimpleEncoder,
        End,
        ALIGN,
        V1Type,
        WritableBufferImpl,
        &mut buffer,
        offset,
        &v1_in
    );
    let encoded_value = buffer.finalize();
    let expected = "0000000100020003000400050006000700000000000000000000000000000000";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer = ReadableBufferImpl::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();
    writable_buffer_call!(@dec
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
    type End = LE;
    const ALIGN: usize = ALIGN_DEFAULT;
    type V1ItemItemType = u16;
    type V1ItemType = [V1ItemItemType; 7];
    type V1Type = Vec<V1ItemType>;
    let v1_in: V1Type = Vec::from([[0, 1, 2, 3, 4, 5, 6], [7, 8, 9, 10, 11, 12, 13]]);
    let offset = 0;
    let header_len = dynamic_size_aligned!(ALIGN, v1_in.len());
    let mut buffer = WritableBufferImpl::<End>::new(header_len, None);
    writable_buffer_call!(@enc
        SimpleEncoder,
        End,
        ALIGN,
        V1Type,
        WritableBufferImpl,
        &mut buffer,
        offset,
        &v1_in
    );
    let encoded_value = buffer.finalize();
    let expected = "00000100020003000400050006000700080009000a000b000c000d00";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer = ReadableBufferImpl::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();
    writable_buffer_call!(@dec
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
fn test_simple_encoder_be_ad_vec_of_fixed() {
    type End = BE;
    const ALIGN: usize = ALIGN_DEFAULT;
    type V1ItemItemType = u16;
    type V1ItemType = [V1ItemItemType; 7];
    type V1Type = Vec<V1ItemType>;
    let v1_in: V1Type = Vec::from([[0, 1, 2, 3, 4, 5, 6], [7, 8, 9, 10, 11, 12, 13]]);
    let offset = 0;
    let header_len = dynamic_size_aligned!(ALIGN, v1_in.len());
    let mut buffer = WritableBufferImpl::<End>::new(header_len, None);
    writable_buffer_call!(@enc
        SimpleEncoder,
        End,
        ALIGN,
        V1Type,
        WritableBufferImpl,
        &mut buffer,
        offset,
        &v1_in
    );
    let encoded_value = buffer.finalize();
    let expected = "0000000100020003000400050006000700080009000a000b000c000d";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer = ReadableBufferImpl::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();
    writable_buffer_call!(@dec
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
fn test_simple_encoder_be_a32_vec_of_fixed() {
    type End = BE;
    const ALIGN: usize = ALIGN_32;
    type V1ItemItemType = u16;
    type V1ItemType = [V1ItemItemType; 7];
    type V1Type = Vec<V1ItemType>;
    let v1_in: V1Type = Vec::from([[0, 1, 2, 3, 4, 5, 6], [7, 8, 9, 10, 11, 12, 13]]);
    let offset = 0;
    let header_len = v1_in.len() * dynamic_size_aligned!(ALIGN, size_of!(V1ItemType));
    let mut buffer = WritableBufferImpl::<End>::new(header_len, None);
    writable_buffer_call!(@enc
        SimpleEncoder,
        End,
        ALIGN,
        V1Type,
        WritableBufferImpl,
        &mut buffer,
        offset,
        &v1_in
    );
    let encoded_value = buffer.finalize();
    let expected = "\
    0000000000000000000000000000000000000000000100020003000400050006\
    000000000000000000000000000000000000000700080009000a000b000c000d";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer = ReadableBufferImpl::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();
    writable_buffer_call!(@dec
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
fn test_structured_encoder_be_ad_vec_u8() {
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
        structured_encoder_const_val!(V1Type, End, ALIGN, HEADER_SIZE)
    );
    let mut buffer = WritableBufferImpl::<End>::new(header_size, None);
    structured_encoder_call!(@enc V1Type, End, ALIGN, &mut buffer, offset, &v1_in);
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
    let buffer = ReadableBufferImpl::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();
    structured_encoder_call!(@dec V1Type, End, ALIGN, &buffer, offset, &mut v1_out);
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_structured_encoder_be_a32_vec_u8() {
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
        structured_encoder_const_val!(V1Type, End, ALIGN, HEADER_SIZE)
    );
    let mut buffer = WritableBufferImpl::<End>::new(header_size, None);
    structured_encoder_call!(@enc V1Type, End, ALIGN, &mut buffer, offset, &v1_in);
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
    let buffer = ReadableBufferImpl::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();

    structured_encoder_call!(@dec V1Type, End, ALIGN, &buffer, offset, &mut v1_out,);
    let v1_in_len_aligned = dynamic_size_aligned!(ALIGN, v1_in.len(), V1ItemType);
    v1_in.resize(v1_in_len_aligned, 0);
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_structured_encoder_be_ad_vec_u16() {
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
        structured_encoder_const_val!(V1Type, End, ALIGN, HEADER_SIZE)
    );
    let mut buffer = WritableBufferImpl::<End>::new(header_size, None);
    structured_encoder_call!(@enc V1Type, End, ALIGN, &mut buffer, offset, &v1_in);
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
    let buffer = ReadableBufferImpl::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();

    structured_encoder_call!(@dec V1Type, End, ALIGN, &buffer, offset, &mut v1_out,);
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_structured_encoder_be_a32_vec_u16() {
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
        structured_encoder_const_val!(V1Type, End, ALIGN, HEADER_SIZE)
    );
    let mut buffer = WritableBufferImpl::<End>::new(header_size, None);
    structured_encoder_call!(@enc V1Type, End, ALIGN, &mut buffer, offset, &v1_in);
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
    let buffer = ReadableBufferImpl::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();

    structured_encoder_call!(@dec V1Type, End, ALIGN, &buffer, offset, &mut v1_out,);
    let v1_in_len_aligned = dynamic_size_aligned!(ALIGN, v1_in.len(), V1ItemType);
    v1_in.resize(v1_in_len_aligned, 0);
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_structured_encoder_be_ad_vec_u16_empty() {
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
        structured_encoder_const_val!(V1Type, End, ALIGN, HEADER_SIZE)
    );
    let mut buffer = WritableBufferImpl::<End>::new(header_size, None);
    structured_encoder_call!(@enc V1Type, End, ALIGN, &mut buffer, offset, &v1_in);
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
    let buffer = ReadableBufferImpl::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();

    structured_encoder_call!(@dec V1Type, End, ALIGN, &buffer, offset, &mut v1_out,);
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_structured_encoder_be_a32_vec_u16_empty() {
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
        structured_encoder_const_val!(V1Type, End, ALIGN, HEADER_SIZE)
    );
    let mut buffer = WritableBufferImpl::<End>::new(header_size, None);
    structured_encoder_call!(@enc V1Type, End, ALIGN, &mut buffer, offset, &v1_in);
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
    let buffer = ReadableBufferImpl::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();

    structured_encoder_call!(@dec V1Type, End, ALIGN, &buffer, offset, &mut v1_out,);
    let v1_in_len_aligned = dynamic_size_aligned!(ALIGN, v1_in.len(), V1ItemType);
    v1_in.resize(v1_in_len_aligned, 0);
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_structured_encoder_be_ad_vec_of_fixed16() {
    type End = BE;
    const ALIGN: usize = ALIGN_DEFAULT;
    type V1ItemItemType = u8;
    type V1ItemType = [V1ItemItemType; 7];
    type V1Type = Vec<V1ItemType>;
    let mut v1_in: V1Type = Vec::from([[1, 2, 3, 4, 5, 6, 7]]);
    let offset = 0;
    let header_item_size = header_item_size!(ALIGN);
    let header_size = header_size!(ALIGN, 3);
    assert_eq!(
        header_size,
        structured_encoder_const_val!(V1Type, End, ALIGN, HEADER_SIZE),
    );
    let mut buffer = WritableBufferImpl::<End>::new(header_size, None);
    structured_encoder_call!(@enc V1Type, End, ALIGN, &mut buffer, offset, &v1_in);
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
    00000001\
    0000000c\
    00000007\
    01020304050607\
    ";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let buffer = ReadableBufferImpl::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();

    structured_encoder_call!(@dec V1Type, End, ALIGN, &buffer, offset, &mut v1_out,);
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_structured_encoder_be_a32_vec_of_fixed16() {
    type End = BE;
    const ALIGN: usize = ALIGN_32;
    type V1ItemType = [u16; 3];
    type V1Type = Vec<V1ItemType>;
    let mut v1_in: V1Type = Vec::from([[1, 2, 3]]);
    let offset = 0;
    let header_item_size = header_item_size!(ALIGN);
    let header_size = header_size!(ALIGN, 3);
    assert_eq!(
        header_size,
        structured_encoder_const_val!(V1Type, End, ALIGN, HEADER_SIZE)
    );
    let mut buffer = WritableBufferImpl::<End>::new(header_size, None);
    structured_encoder_call!(@enc V1Type, End, ALIGN, &mut buffer, offset, &v1_in);
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
    0000000000000000000000000000000000000000000000000000000000000001\
    0000000000000000000000000000000000000000000000000000000000000060\
    0000000000000000000000000000000000000000000000000000000000000006\
    0000000000000000000000000000000000000000000000000000000100020003\
    ";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let buffer = ReadableBufferImpl::<End>::new(&encoded_value);
    let mut v1_out: V1Type = Vec::new();

    structured_encoder_call!(@dec V1Type, End, ALIGN, &buffer, offset, &mut v1_out,);
    // let v1_in_len_aligned = dynamic_size_aligned!(ALIGN, v1_in.len(), V1ItemType);
    // v1_in.resize(v1_in_len_aligned, 0);
    assert_eq!(v1_in, v1_out);
}
