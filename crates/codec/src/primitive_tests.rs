use byteorder::{BE, LE};

use crate::buffer::ReadableBufferImpl;
use crate::encoder::{SimpleEncoder, ALIGN_32, ALIGN_DEFAULT};
use crate::{
    fixed_type_size_aligned, header_item_size, structured_encoder_call,
    structured_encoder_const_val, writable_buffer_call, WritableBufferImpl,
};

#[test]
fn test_simple_encoder_le_ad_i32() {
    type End = LE;
    const ALIGN: usize = ALIGN_DEFAULT;
    type VType1 = i32;
    let v1_in: VType1 = 10;
    let offset = 0;
    let mut buffer = WritableBufferImpl::<End>::new(fixed_type_size_aligned!(ALIGN, VType1), None);
    writable_buffer_call!(@enc
        SimpleEncoder,
        End,
        ALIGN,
        VType1,
        WritableBufferImpl,
        &mut buffer,
        offset,
        &v1_in
    );
    let encoded_value = buffer.finalize();
    let expected = "0a000000";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer = ReadableBufferImpl::<End>::new(&encoded_value);
    let mut v1_out: VType1 = 0;
    writable_buffer_call!(@dec
        SimpleEncoder,
        End,
        ALIGN,
        VType1,
        &mut buffer,
        offset,
        &mut v1_out
    );
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_simple_encoder_be_a32_i32() {
    type End = BE;
    const ALIGN: usize = ALIGN_32;
    type VType1 = i32;
    let v1_in: VType1 = 10;
    let offset = 0;
    let mut buffer = WritableBufferImpl::<End>::new(fixed_type_size_aligned!(ALIGN, VType1), None);
    writable_buffer_call!(@enc
        SimpleEncoder,
        End,
        ALIGN,
        VType1,
        WritableBufferImpl,
        &mut buffer,
        offset,
        &v1_in
    );
    let encoded_value = buffer.finalize();
    let expected = "000000000000000000000000000000000000000000000000000000000000000a";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer = ReadableBufferImpl::<End>::new(&encoded_value);
    let mut v1_out: VType1 = 0;
    writable_buffer_call!(@dec
        SimpleEncoder,
        End,
        ALIGN,
        VType1,
        &mut buffer,
        offset,
        &mut v1_out
    );
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_simple_encoder_le_ad_i64() {
    type End = LE;
    const ALIGN: usize = ALIGN_DEFAULT;
    type VType1 = i64;
    let v1_in: VType1 = 10;
    let offset = 0;
    let mut buffer = WritableBufferImpl::<End>::new(fixed_type_size_aligned!(ALIGN, VType1), None);
    writable_buffer_call!(@enc
        SimpleEncoder,
        End,
        ALIGN,
        VType1,
        WritableBufferImpl,
        &mut buffer,
        offset,
        &v1_in
    );
    let encoded_value = buffer.finalize();
    let expected = "0a00000000000000";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer = ReadableBufferImpl::<End>::new(&encoded_value);
    let mut v1_out: VType1 = 0;
    writable_buffer_call!(@dec
        SimpleEncoder,
        End,
        ALIGN,
        VType1,
        &mut buffer,
        offset,
        &mut v1_out
    );
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_simple_encoder_be_a32_i64() {
    type End = BE;
    const ALIGN: usize = ALIGN_32;
    type VType1 = i64;
    let v1_in: VType1 = 10;
    let offset = 0;
    let mut buffer = WritableBufferImpl::<End>::new(fixed_type_size_aligned!(ALIGN, VType1), None);
    writable_buffer_call!(@enc
        SimpleEncoder,
        End,
        ALIGN,
        VType1,
        WritableBufferImpl,
        &mut buffer,
        offset,
        &v1_in
    );
    let encoded_value = buffer.finalize();
    let expected = "000000000000000000000000000000000000000000000000000000000000000a";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer = ReadableBufferImpl::<End>::new(&encoded_value);
    let mut v1_out: VType1 = 0;
    writable_buffer_call!(@dec
        SimpleEncoder,
        End,
        ALIGN,
        VType1,
        &mut buffer,
        offset,
        &mut v1_out
    );
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_structured_encoder_le_ad_u64() {
    type End = LE;
    const ALIGN: usize = ALIGN_DEFAULT;
    type V1Type = u64;
    let v1_in: V1Type = 12345;
    let offset = 0;
    let header_item_size = header_item_size!(ALIGN, V1Type);
    let header_size = header_item_size;
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
    3930000000000000\
    ";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let buffer = ReadableBufferImpl::<End>::new(&encoded_value);
    let mut v1_out: V1Type = 0;
    structured_encoder_call!(@dec V1Type, End, ALIGN, &buffer, offset, &mut v1_out);
    assert_eq!(v1_in, v1_out);
}

#[test]
fn test_structured_encoder_be_a32_u64() {
    type End = BE;
    const ALIGN: usize = ALIGN_32;
    type V1Type = u64;
    let mut v1_in: V1Type = 12345;
    let offset = 0;
    let header_item_size = header_item_size!(ALIGN, V1Type);
    let header_size = header_item_size;
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
    0000000000000000000000000000000000000000000000000000000000003039\
    ";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let buffer = ReadableBufferImpl::<End>::new(&encoded_value);
    let mut v1_out: V1Type = 0;

    structured_encoder_call!(@dec V1Type, End, ALIGN, &buffer, offset, &mut v1_out,);
    // let v1_in_len_aligned = dynamic_size_aligned!(ALIGN, size_of!(V1Type));
    // v1_in.resize(v1_in_len_aligned, 0);
    assert_eq!(v1_in, v1_out);
}
