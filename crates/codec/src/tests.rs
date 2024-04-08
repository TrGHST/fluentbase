use super::{BufferDecoder, BufferEncoder, Encoder};
use alloy_primitives::Bytes;
use byteorder::LittleEndian;
use hashbrown::{HashMap, HashSet};

#[test]
fn test_i16() {
    let values: i16 = 12345;
    let result = {
        let mut buffer_encoder = BufferEncoder::<LittleEndian>::new(
            <i16 as Encoder<LittleEndian, i16>>::HEADER_SIZE,
            None,
        );
        values.encode(&mut buffer_encoder, 0);
        buffer_encoder.finalize()
    };
    let result_encoded = hex::encode(&result);
    assert_eq!("3930", result_encoded);
    let mut buffer_decoder = BufferDecoder::<LittleEndian>::new(result.as_slice());
    let mut values2 = Default::default();
    <i16>::decode_body(&mut buffer_decoder, 0, &mut values2);
    assert_eq!(values, values2);
}

#[test]
fn test_vec() {
    let values = vec![0, 1, 2, 3];
    let result = {
        let mut buffer_encoder = BufferEncoder::<LittleEndian>::new(
            <Vec<i32> as Encoder<LittleEndian, Vec<i32>>>::HEADER_SIZE,
            None,
        );
        values.encode(&mut buffer_encoder, 0);
        buffer_encoder.finalize()
    };
    let result_encoded = hex::encode(&result);
    println!("{}", result_encoded);
    assert_eq!(
        "\
        04000000\
        0c000000\
        10000000\
        00000000\
        01000000\
        02000000\
        03000000\
        ",
        result_encoded,
    );
    let mut buffer_decoder = BufferDecoder::<LittleEndian>::new(result.as_slice());
    let mut values2 = Default::default();
    Vec::<i32>::decode_body(&mut buffer_decoder, 0, &mut values2);
    assert_eq!(values, values2);
}

#[test]
fn test_bytes() {
    let values = Bytes::from_static("Hello, World".as_bytes());
    let result = {
        let mut buffer_encoder = BufferEncoder::<LittleEndian>::new(
            <Bytes as Encoder<LittleEndian, Bytes>>::HEADER_SIZE,
            None,
        );
        values.encode(&mut buffer_encoder, 0);
        buffer_encoder.finalize()
    };
    println!("{}", hex::encode(&result));
    let mut buffer_decoder = BufferDecoder::<LittleEndian>::new(result.as_slice());
    let mut values2 = Default::default();
    Bytes::decode_body(&mut buffer_decoder, 0, &mut values2);
    assert_eq!(values, values2);
}

#[test]
fn test_nested_vec() {
    let values = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
    let result = {
        let mut buffer_encoder = BufferEncoder::<LittleEndian>::new(
            <Vec<i32> as Encoder<LittleEndian, Vec<i32>>>::HEADER_SIZE,
            None,
        );
        values.encode(&mut buffer_encoder, 0);
        buffer_encoder.finalize()
    };
    let result_encoded = hex::encode(&result);
    println!("{}", result_encoded);
    assert_eq!(
        "\
        03000000\
        0c000000\
        48000000\

        03000000\
        24000000\
        0c000000\

        03000000\
        30000000\
        0c000000\

        03000000\
        3c000000\
        0c000000\

        00000000\
        01000000\
        02000000\

        03000000\
        04000000\
        05000000\

        06000000\
        07000000\
        08000000\
        ",
        result_encoded,
    );
    let mut buffer_decoder = BufferDecoder::<LittleEndian>::new(result.as_slice());
    let mut values2 = Default::default();
    Vec::<Vec<i32>>::decode_body(&mut buffer_decoder, 0, &mut values2);
    assert_eq!(values, values2);
}

#[test]
fn test_empty_vec() {
    let values: Vec<u32> = vec![];
    let result = {
        let mut buffer_encoder = BufferEncoder::<LittleEndian>::new(
            <Vec<i32> as Encoder<LittleEndian, Vec<i32>>>::HEADER_SIZE,
            None,
        );
        values.encode(&mut buffer_encoder, 0);
        buffer_encoder.finalize()
    };
    println!("{}", hex::encode(&result));
    let mut buffer_decoder = BufferDecoder::<LittleEndian>::new(result.as_slice());
    let mut values2 = Default::default();
    Vec::<u32>::decode_body(&mut buffer_decoder, 0, &mut values2);
    assert_eq!(values, values2);
}

#[test]
fn test_map() {
    let mut values: HashMap<i32, i32> = HashMap::new();
    values.insert(100, 20);
    values.insert(3, 5);
    values.insert(1000, 60);
    let result = {
        let mut buffer_encoder = BufferEncoder::<LittleEndian>::new(
            <HashMap<i32, i32> as Encoder<LittleEndian, HashMap<i32, i32>>>::HEADER_SIZE,
            None,
        );
        values.encode(&mut buffer_encoder, 0);
        buffer_encoder.finalize()
    };
    println!("{}", hex::encode(&result));
    let mut buffer_decoder = BufferDecoder::<LittleEndian>::new(result.as_slice());
    let mut values2 = Default::default();
    HashMap::decode_body(&mut buffer_decoder, 0, &mut values2);
    assert_eq!(values, values2);
}

#[test]
fn test_set() {
    let values = HashSet::from([1, 2, 3]);
    let result = {
        let mut buffer_encoder = BufferEncoder::<LittleEndian>::new(
            <HashSet<i32> as Encoder<LittleEndian, HashSet<i32>>>::HEADER_SIZE,
            None,
        );
        values.encode(&mut buffer_encoder, 0);
        buffer_encoder.finalize()
    };
    println!("{}", hex::encode(&result));
    let mut buffer_decoder = BufferDecoder::<LittleEndian>::new(result.as_slice());
    let mut values2 = Default::default();
    HashSet::decode_body(&mut buffer_decoder, 0, &mut values2);
    assert_eq!(values, values2);
}

#[test]
fn test_set_is_sorted() {
    let result1 = {
        let values = HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let mut buffer_encoder = BufferEncoder::<LittleEndian>::new(
            <HashSet<i32> as Encoder<LittleEndian, HashSet<i32>>>::HEADER_SIZE,
            None,
        );
        values.encode(&mut buffer_encoder, 0);
        buffer_encoder.finalize()
    };
    let result2 = {
        let values = HashSet::from([8, 3, 2, 4, 5, 9, 7, 1, 6]);
        let mut buffer_encoder = BufferEncoder::<LittleEndian>::new(
            <HashSet<i32> as Encoder<LittleEndian, HashSet<i32>>>::HEADER_SIZE,
            None,
        );
        values.encode(&mut buffer_encoder, 0);
        buffer_encoder.finalize()
    };
    assert_eq!(result1, result2);
}

#[test]
fn test_nested_map() {
    let mut values = HashMap::new();
    values.insert(100, HashMap::from([(1, 2), (3, 4)]));
    values.insert(3, HashMap::new());
    values.insert(1000, HashMap::from([(7, 8), (9, 4)]));
    let result = {
        let mut buffer_encoder = BufferEncoder::<LittleEndian>::new(
            <HashMap<i32, HashMap<i32, i32>> as Encoder<
                LittleEndian,
                HashMap<i32, HashMap<i32, i32>>,
            >>::HEADER_SIZE,
            None,
        );
        values.encode(&mut buffer_encoder, 0);
        buffer_encoder.finalize()
    };
    println!("{}", hex::encode(&result));
    let mut buffer_decoder = BufferDecoder::<LittleEndian>::new(result.as_slice());
    let mut values2 = Default::default();
    HashMap::decode_body(&mut buffer_decoder, 0, &mut values2);
    assert_eq!(values, values2);
}

#[test]
fn test_vector_of_maps() {
    let mut values = Vec::new();
    values.push(HashMap::from([(1, 2), (3, 4)]));
    values.push(HashMap::new());
    values.push(HashMap::from([(7, 8), (9, 4)]));
    let result = {
        let mut buffer_encoder = BufferEncoder::<LittleEndian>::new(
            <Vec<HashMap<i32, i32>> as Encoder<LittleEndian, Vec<HashMap<i32, i32>>>>::HEADER_SIZE,
            None,
        );
        values.encode(&mut buffer_encoder, 0);
        buffer_encoder.finalize()
    };
    println!("{}", hex::encode(&result));
    let mut buffer_decoder = BufferDecoder::<LittleEndian>::new(result.as_slice());
    let mut values2 = Default::default();
    Vec::decode_body(&mut buffer_decoder, 0, &mut values2);
    assert_eq!(values, values2);
}

#[test]
fn test_map_of_vectors() {
    let mut values = HashMap::new();
    values.insert(vec![0, 1, 2], vec![3, 4, 5]);
    values.insert(vec![3, 1, 2], vec![3, 4, 5]);
    values.insert(vec![0, 1, 6], vec![3, 4, 5]);
    let result = {
        let mut buffer_encoder =
            BufferEncoder::<LittleEndian>::new(
                <HashMap<Vec<i32>, Vec<i32>> as Encoder<
                    LittleEndian,
                    HashMap<Vec<i32>, Vec<i32>>,
                >>::HEADER_SIZE,
                None,
            );
        values.encode(&mut buffer_encoder, 0);
        buffer_encoder.finalize()
    };
    println!("{}", hex::encode(&result));
    let mut buffer_decoder = BufferDecoder::<LittleEndian>::new(result.as_slice());
    let mut values2 = Default::default();
    HashMap::decode_body(&mut buffer_decoder, 0, &mut values2);
    assert_eq!(values, values2);
}

#[test]
fn test_static_array() {
    type T = [i32; 3];
    let values: T = [1, 2, 3];
    let result = {
        let mut buffer_encoder =
            BufferEncoder::<LittleEndian>::new(<T as Encoder<LittleEndian, T>>::HEADER_SIZE, None);
        values.encode(&mut buffer_encoder, 0);
        buffer_encoder.finalize()
    };
    println!("{}", hex::encode(&result));
    let mut buffer_decoder = BufferDecoder::<LittleEndian>::new(result.as_slice());
    let mut values2 = Default::default();
    <[i32; 3]>::decode_body(&mut buffer_decoder, 0, &mut values2);
    assert_eq!(values, values2);
}

#[test]
fn test_empty_static_array() {
    type T = [u8; 0];
    let values: T = [];
    let result = {
        let mut buffer_encoder =
            BufferEncoder::<LittleEndian>::new(<T as Encoder<LittleEndian, T>>::HEADER_SIZE, None);
        values.encode(&mut buffer_encoder, 0);
        buffer_encoder.finalize()
    };
    println!("hex: '{}'", hex::encode(&result));
    let mut buffer_decoder = BufferDecoder::<LittleEndian>::new(result.as_slice());
    let mut values2 = Default::default();
    <[u8; 0]>::decode_body(&mut buffer_decoder, 0, &mut values2);
    assert_eq!(values, values2);
}

#[test]
fn test_static_array_of_arrays() {
    type T = [[i32; 3]; 3];
    let values: T = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let result = {
        let mut buffer_encoder =
            BufferEncoder::<LittleEndian>::new(<T as Encoder<LittleEndian, T>>::HEADER_SIZE, None);
        values.encode(&mut buffer_encoder, 0);
        buffer_encoder.finalize()
    };
    println!("{}", hex::encode(&result));
    let mut buffer_decoder = BufferDecoder::<LittleEndian>::new(result.as_slice());
    let mut values2 = Default::default();
    <[[i32; 3]; 3]>::decode_body(&mut buffer_decoder, 0, &mut values2);
    assert_eq!(values, values2);
}

#[test]
fn test_option() {
    type T = Option<u32>;
    let value1: T = Some(0x7bu32);
    let value2: T = None;
    let result = {
        // TODO doesnt work. bug?
        // let mut buffer_encoder =
        //     BufferEncoder::<LittleEndian>::new(<T as Encoder<LittleEndian, T>>::HEADER_SIZE, None);
        // assert_eq!(<T as Encoder<LittleEndian, T>>::HEADER_SIZE, 5 + 5);
        let mut buffer_encoder = BufferEncoder::<LittleEndian>::new(5 + 5, None);
        value1.encode(&mut buffer_encoder, 0);
        value2.encode(&mut buffer_encoder, 5);
        buffer_encoder.finalize()
    };
    println!("{}", hex::encode(&result));
    let mut buffer_decoder = BufferDecoder::<LittleEndian>::new(result.as_slice());
    let mut decoded1 = Default::default();
    let mut decoded2 = Default::default();
    Option::<u32>::decode_header(&mut buffer_decoder, 0, &mut decoded1);
    Option::<u32>::decode_header(&mut buffer_decoder, 5, &mut decoded2);
    assert_eq!(value1, decoded1);
    assert_eq!(value2, decoded2);
}

#[test]
fn test_option_non_primitive() {
    type T = Option<Vec<u32>>;
    let value: T = Some(vec![1, 2, 3]);
    let result = {
        let mut buffer_encoder =
            BufferEncoder::<LittleEndian>::new(<T as Encoder<LittleEndian, T>>::HEADER_SIZE, None);
        value.encode(&mut buffer_encoder, 0);
        buffer_encoder.finalize()
    };
    println!("{}", hex::encode(&result));
    let mut buffer_decoder = BufferDecoder::<LittleEndian>::new(result.as_slice());
    let mut decoded_value = Default::default();
    Option::<Vec<u32>>::decode_body(&mut buffer_decoder, 0, &mut decoded_value);
    assert_eq!(value, decoded_value);
}
