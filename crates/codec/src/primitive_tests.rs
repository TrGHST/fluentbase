use crate::buffer::ReadableBuffer;
use crate::encoder::{SimpleEncoder, ALIGN_32, ALIGN_DEFAULT};
use crate::{
    dynamic_buffer_decode, dynamic_buffer_encode, dynamic_size_aligned, fixed_type_size_aligned,
    DynamicBuffer,
};
use byteorder::{BE, LE};

#[test]
fn test_simple_encoder_le_ad_i32() {
    type End = LE;
    const ALIGN: usize = ALIGN_DEFAULT;
    type VType1 = i32;
    let v1_in: VType1 = 10;
    let offset = 0;
    let mut buffer = DynamicBuffer::<End>::new(fixed_type_size_aligned!(ALIGN, VType1), None);
    dynamic_buffer_encode!(
        SimpleEncoder,
        End,
        ALIGN,
        VType1,
        DynamicBuffer,
        &mut buffer,
        offset,
        &v1_in
    );
    let encoded_value = buffer.finalize();
    let expected = "0a000000";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer = ReadableBuffer::<End>::new(&encoded_value);
    let mut v1_out: VType1 = 0;
    dynamic_buffer_decode!(
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
    let mut buffer = DynamicBuffer::<End>::new(fixed_type_size_aligned!(ALIGN, VType1), None);
    dynamic_buffer_encode!(
        SimpleEncoder,
        End,
        ALIGN,
        VType1,
        DynamicBuffer,
        &mut buffer,
        offset,
        &v1_in
    );
    let encoded_value = buffer.finalize();
    let expected = "000000000000000000000000000000000000000000000000000000000000000a";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer = ReadableBuffer::<End>::new(&encoded_value);
    let mut v1_out: VType1 = 0;
    dynamic_buffer_decode!(
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
    let mut buffer = DynamicBuffer::<End>::new(fixed_type_size_aligned!(ALIGN, VType1), None);
    dynamic_buffer_encode!(
        SimpleEncoder,
        End,
        ALIGN,
        VType1,
        DynamicBuffer,
        &mut buffer,
        offset,
        &v1_in
    );
    let encoded_value = buffer.finalize();
    let expected = "0a00000000000000";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer = ReadableBuffer::<End>::new(&encoded_value);
    let mut v1_out: VType1 = 0;
    dynamic_buffer_decode!(
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
    let mut buffer = DynamicBuffer::<End>::new(fixed_type_size_aligned!(ALIGN, VType1), None);
    dynamic_buffer_encode!(
        SimpleEncoder,
        End,
        ALIGN,
        VType1,
        DynamicBuffer,
        &mut buffer,
        offset,
        &v1_in
    );
    let encoded_value = buffer.finalize();
    let expected = "000000000000000000000000000000000000000000000000000000000000000a";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer = ReadableBuffer::<End>::new(&encoded_value);
    let mut v1_out: VType1 = 0;
    dynamic_buffer_decode!(
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
fn test_simple_encoder_le_ad_vec_u8() {
    type End = LE;
    const ALIGN: usize = ALIGN_DEFAULT;
    type VType1 = Vec<u8>;
    let v1_in: VType1 = Vec::from([0, 1, 2, 3, 4, 5, 6, 7]);
    let offset = 0;
    let header_len = dynamic_size_aligned!(ALIGN, v1_in.len());
    let mut buffer = DynamicBuffer::<End>::new(header_len, None);
    dynamic_buffer_encode!(
        SimpleEncoder,
        End,
        ALIGN,
        VType1,
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
    let mut v1_out: VType1 = Vec::from([0u8; 8]);
    dynamic_buffer_decode!(
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
fn test_simple_encoder_be_a32_vec_u8() {
    type End = BE;
    const ALIGN: usize = ALIGN_32;
    type VType1 = Vec<u8>;
    let v1_in: VType1 = Vec::from([0, 1, 2, 3, 4, 5, 6, 7]);
    let offset = 0;
    let header_len = dynamic_size_aligned!(ALIGN, v1_in.len());
    let mut buffer = DynamicBuffer::<End>::new(header_len, None);
    dynamic_buffer_encode!(
        SimpleEncoder,
        End,
        ALIGN,
        VType1,
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
    let mut v1_out: VType1 = Vec::from([0u8; 8]);
    dynamic_buffer_decode!(
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
