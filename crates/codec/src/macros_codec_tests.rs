use byteorder::{BE, LE};

use crate::encoder::{Encoder, StructuredEncoder, ALIGN_DEFAULT};
use crate::{
    define_codec_struct, encoder_call, encoder_const_val, header_item_size, ReadableBufferImpl,
    WritableBufferImpl, ALIGN_32,
};

#[test]
fn test_simple_type_alignment_default_u_le() {
    type Endianness = LE;
    const ALIGN: usize = ALIGN_DEFAULT;
    let header_item_size = header_item_size!(ALIGN);

    define_codec_struct! {
        Endianness,
        ALIGN,
        pub struct Inner{
            a21: u64,
            a22: u32,
        }
    };

    define_codec_struct! {
        Endianness,
        ALIGN,
        pub struct SimpleTypeU {
            a1: u64,
            b2: u32,
            c3: u8,
            // inner: Inner,
        }
    }

    let value0 = SimpleTypeU {
        a1: 100,
        b2: 20,
        c3: 3,
        // inner: Inner { a21: 321 },
    };
    assert_eq!(
        13,
        0 + encoder_const_val!(SimpleTypeU, Endianness, ALIGN, HEADER_SIZE)
    );
    assert_eq!(
        <SimpleTypeU as Encoder<Endianness, ALIGN, SimpleTypeU>>::HEADER_SIZE,
        0 + encoder_const_val!(u64, Endianness, ALIGN, HEADER_SIZE)
            + encoder_const_val!(u32, Endianness, ALIGN, HEADER_SIZE)
            + encoder_const_val!(u8, Endianness, ALIGN, HEADER_SIZE)
    );
    let encoded_value = {
        let mut buffer_encoder = WritableBufferImpl::<Endianness>::new(
            <SimpleTypeU as Encoder<Endianness, ALIGN, SimpleTypeU>>::HEADER_SIZE,
            None,
        );
        <SimpleTypeU as Encoder<Endianness, ALIGN, SimpleTypeU>>::encode::<
            WritableBufferImpl<Endianness>,
        >(&value0, &mut buffer_encoder, 0);
        buffer_encoder.finalize()
    };
    let expected = "64000000000000001400000003";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer_decoder = ReadableBufferImpl::<Endianness>::new(encoded_value.as_slice());
    let mut value1 = Default::default();
    <SimpleTypeU as Encoder<Endianness, ALIGN, SimpleTypeU>>::decode_body(
        &mut buffer_decoder,
        0,
        &mut value1,
    );
    assert_eq!(value0, value1);
}

#[test]
fn test_simple_type_alignment_32_u_be() {
    type Endianness = BE;
    const ALIGN: usize = ALIGN_32;
    let header_item_size = header_item_size!(ALIGN);

    define_codec_struct! {
        Endianness,
        ALIGN,
        pub struct SimpleTypeU {
            a: u64,
            b: u32,
            c: u16,
            bool: bool,
        }
    }

    let value0 = SimpleTypeU {
        a: 100,
        b: 20,
        c: 3,
        bool: true,
    };
    assert_eq!(
        <SimpleTypeU as Encoder<Endianness, ALIGN, SimpleTypeU>>::HEADER_SIZE,
        <u64 as Encoder<Endianness, ALIGN, u64>>::HEADER_SIZE
            + <u32 as Encoder<Endianness, ALIGN, u32>>::HEADER_SIZE
            + <u16 as Encoder<Endianness, ALIGN, u16>>::HEADER_SIZE
            + <bool as Encoder<Endianness, ALIGN, bool>>::HEADER_SIZE
    );
    let encoded_value = {
        let mut buffer_encoder = WritableBufferImpl::<Endianness>::new(
            <SimpleTypeU as Encoder<Endianness, ALIGN, SimpleTypeU>>::HEADER_SIZE,
            None,
        );
        <SimpleTypeU as Encoder<Endianness, ALIGN, SimpleTypeU>>::encode(
            &value0,
            &mut buffer_encoder,
            0,
        );
        buffer_encoder.finalize()
    };
    let expected = "\
        0000000000000000000000000000000000000000000000000000000000000064\
        0000000000000000000000000000000000000000000000000000000000000014\
        0000000000000000000000000000000000000000000000000000000000000003\
        0000000000000000000000000000000000000000000000000000000000000001\
        ";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer_decoder = ReadableBufferImpl::<Endianness>::new(encoded_value.as_slice());
    let mut value1 = Default::default();
    <SimpleTypeU as Encoder<Endianness, ALIGN, SimpleTypeU>>::decode_body(
        &mut buffer_decoder,
        0,
        &mut value1,
    );
    assert_eq!(value0, value1);
}

#[test]
fn test_simple_type_s_alignment_default_le() {
    type Endianness = LE;
    const ALIGN: usize = ALIGN_DEFAULT;
    define_codec_struct! {
        Endianness,
        ALIGN,
        pub struct SimpleTypeS {
            a: i64,
            b: i32,
            c: i16,
        }
    }
    let value0 = SimpleTypeS {
        a: -100,
        b: -20,
        c: -3,
    };
    assert_eq!(
        <SimpleTypeS as Encoder<Endianness, ALIGN, SimpleTypeS>>::HEADER_SIZE,
        <i64 as Encoder<Endianness, ALIGN, i64>>::HEADER_SIZE
            + <i32 as Encoder<Endianness, ALIGN, i32>>::HEADER_SIZE
            + <i16 as Encoder<Endianness, ALIGN, i16>>::HEADER_SIZE
    );
    let encoded_value = {
        let mut buffer_encoder = WritableBufferImpl::<Endianness>::new(
            <SimpleTypeS as Encoder<Endianness, ALIGN, SimpleTypeS>>::HEADER_SIZE,
            None,
        );
        encoder_call!(@encode
            SimpleTypeS,
            Endianness,
            ALIGN,
            &mut buffer_encoder,
            0,
            &value0,
        );
        buffer_encoder.finalize()
    };
    let expected = "\
        9cffffffffffffff\
        ecffffff\
        fdff\
        ";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer_decoder = ReadableBufferImpl::<Endianness>::new(encoded_value.as_slice());
    let mut value1 = SimpleTypeS::default();
    encoder_call!(@decode_body
        SimpleTypeS,
        Endianness,
        ALIGN,
        &mut buffer_decoder,
        0,
        &mut value1
    );
    assert_eq!(value0, value1);
}

#[test]
fn test_simple_type_s_alignment_32_be() {
    type Endianness = BE;
    const ALIGN: usize = ALIGN_32;
    define_codec_struct! {
        Endianness,
        ALIGN,
        pub struct SimpleTypeS {
            a: i64,
            b: i32,
            c: i16,
        }
    }
    let value0 = SimpleTypeS {
        a: -100,
        b: -20,
        c: -3,
    };

    assert_eq!(
        <SimpleTypeS as Encoder<Endianness, ALIGN, SimpleTypeS>>::HEADER_SIZE,
        <i64 as Encoder<Endianness, ALIGN, i64>>::HEADER_SIZE
            + <i32 as Encoder<Endianness, ALIGN, i32>>::HEADER_SIZE
            + <i16 as Encoder<Endianness, ALIGN, i16>>::HEADER_SIZE
    );
    let encoded_value = {
        let mut buffer_encoder = WritableBufferImpl::<Endianness>::new(
            <SimpleTypeS as Encoder<Endianness, ALIGN, SimpleTypeS>>::HEADER_SIZE,
            None,
        );
        encoder_call!(@encode
            SimpleTypeS,
            Endianness,
            ALIGN,
            &mut buffer_encoder,
            0,
            &value0,
        );
        buffer_encoder.finalize()
    };
    let expected = "\
    000000000000000000000000000000000000000000000000ffffffffffffff9c\
    00000000000000000000000000000000000000000000000000000000ffffffec\
    000000000000000000000000000000000000000000000000000000000000fffd\
    ";
    let fact = hex::encode(&encoded_value);
    assert_eq!(expected, fact);
    let mut buffer_decoder = ReadableBufferImpl::<Endianness>::new(encoded_value.as_slice());
    let mut value1 = Default::default();
    encoder_call!(@decode_body
        SimpleTypeS,
        Endianness,
        ALIGN,
        &mut buffer_decoder,
        0,
        &mut value1
    );
    assert_eq!(value0, value1);
}

#[test]
fn test_decode_specific_field_alignment_default_le() {
    type Endianness = LE;
    const ALIGN: usize = ALIGN_DEFAULT;
    define_codec_struct! {
        Endianness,
        ALIGN,
        pub struct SimpleTypeU {
            a: u64,
            b: u32,
            c: u16,
        }
    }
    let value = SimpleTypeU {
        a: 100,
        b: 20,
        c: 3,
    };
    // check offsets
    assert_eq!(<SimpleTypeU as ISimpleTypeU>::A::FIELD_OFFSET, 0);
    assert_eq!(
        header_item_size!(ALIGN, u64),
        <SimpleTypeU as ISimpleTypeU>::B::FIELD_OFFSET,
    );
    assert_eq!(
        header_item_size!(ALIGN, u64) + header_item_size!(ALIGN, u32),
        <SimpleTypeU as ISimpleTypeU>::C::FIELD_OFFSET,
    );
    // check sizes
    assert_eq!(<SimpleTypeU as ISimpleTypeU>::A::FIELD_SIZE, 8);
    assert_eq!(<SimpleTypeU as ISimpleTypeU>::B::FIELD_SIZE, 4);
    assert_eq!(<SimpleTypeU as ISimpleTypeU>::C::FIELD_SIZE, 2);
    // encode entire struct
    let encoded_value =
        <SimpleTypeU as Encoder<Endianness, ALIGN, SimpleTypeU>>::encode_to_vec(&value, 0);
    let mut encoded_value = encoded_value.as_slice();
    // decode only field `a`
    {
        let mut a: u64 = 0;
        <SimpleTypeU as ISimpleTypeU>::A::decode_field_header(&mut encoded_value, &mut a);
        assert_eq!(a, value.a);
    }
    // decode only field `b`
    {
        let mut b: u32 = 0;
        <SimpleTypeU as ISimpleTypeU>::B::decode_field_header(&mut encoded_value, &mut b);
        assert_eq!(b, value.b);
    }
    // decode only field `c`
    {
        let mut c: u16 = 0;
        <SimpleTypeU as ISimpleTypeU>::C::decode_field_header(&mut encoded_value, &mut c);
        assert_eq!(c, value.c);
    }
}

#[test]
fn test_decode_specific_field_alignment_32_be() {
    type Endianness = BE;
    const ALIGN: usize = ALIGN_32;
    define_codec_struct! {
        Endianness,
        ALIGN,
        pub struct SimpleTypeU {
            a: u64,
            b: u32,
            c: u16,
        }
    }
    let value = SimpleTypeU {
        a: 100,
        b: 20,
        c: 3,
    };
    // check offsets
    assert_eq!(<SimpleTypeU as ISimpleTypeU>::A::FIELD_OFFSET, 0);
    assert_eq!(
        <SimpleTypeU as ISimpleTypeU>::B::FIELD_OFFSET,
        header_item_size!(ALIGN, u64)
    );
    assert_eq!(
        <SimpleTypeU as ISimpleTypeU>::C::FIELD_OFFSET,
        header_item_size!(ALIGN, u64) + header_item_size!(ALIGN, u32)
    );
    // check sizes
    assert_eq!(
        <SimpleTypeU as ISimpleTypeU>::A::FIELD_SIZE,
        header_item_size!(ALIGN, u64)
    );
    assert_eq!(
        <SimpleTypeU as ISimpleTypeU>::B::FIELD_SIZE,
        header_item_size!(ALIGN, u32)
    );
    assert_eq!(
        <SimpleTypeU as ISimpleTypeU>::C::FIELD_SIZE,
        header_item_size!(ALIGN, u16)
    );
    // encode entire struct
    let encoded_value =
        <SimpleTypeU as Encoder<Endianness, ALIGN, SimpleTypeU>>::encode_to_vec(&value, 0);
    let mut encoded_value = encoded_value.as_slice();
    let encoded_value_hex = hex::encode(encoded_value);
    let expected_encoded_value_hex = "\
    0000000000000000000000000000000000000000000000000000000000000064\
    0000000000000000000000000000000000000000000000000000000000000014\
    0000000000000000000000000000000000000000000000000000000000000003\
    ";
    assert_eq!(expected_encoded_value_hex, encoded_value_hex);
    // decode only field `a`
    {
        let mut a: u64 = 0;
        <SimpleTypeU as ISimpleTypeU>::A::decode_field_header(&mut encoded_value, &mut a);
        assert_eq!(a, value.a);
    }
    // decode only field `b`
    {
        let mut b: u32 = 0;
        <SimpleTypeU as ISimpleTypeU>::B::decode_field_header(&mut encoded_value, &mut b);
        assert_eq!(b, value.b);
    }
    // decode only field `c`
    {
        let mut c: u16 = 0;
        <SimpleTypeU as ISimpleTypeU>::C::decode_field_header(&mut encoded_value, &mut c);
        assert_eq!(c, value.c);
    }
}

// #[ignore]
#[test]
fn test_complicated_type_alignment_default_le() {
    type Endianness = LE;
    const ALIGN: usize = ALIGN_DEFAULT;
    define_codec_struct! {
        Endianness,
        ALIGN,
        pub struct SimpleTypeU {
            a: u64,
            b: u32,
            c: u16,
        }
    }
    define_codec_struct! {
        Endianness, ALIGN,
        pub struct ComplicatedType {
            values: Vec<SimpleTypeU>,
            // maps: HashMap<u32, ComplicatedType>,
        }
    }

    let value0 = ComplicatedType {
        values: vec![
            SimpleTypeU {
                a: 100,
                b: 20,
                c: 3,
            },
            SimpleTypeU {
                a: u64::MAX,
                b: u32::MAX,
                c: u16::MAX,
            },
        ],
        // maps: HashMap::from([(
        //     7,
        //     ComplicatedType {
        //         values: vec![
        //             SimpleTypeU { a: 1, b: 2, c: 3 },
        //             SimpleTypeU { a: 4, b: 5, c: 6 },
        //         ],
        //         maps: Default::default(),
        //     },
        // )]),
    };
    assert_eq!(
        <ComplicatedType as Encoder<Endianness, ALIGN, ComplicatedType>>::HEADER_SIZE,
        <Vec<SimpleTypeU> as Encoder<Endianness, ALIGN, Vec<SimpleTypeU>>>::HEADER_SIZE // TODO add map support
                                                                                        // + <HashMap<u32, SimpleTypeU> as Encoder<
                                                                                        //     Endianness,
                                                                                        //     ALIGN,
                                                                                        //     HashMap<u32, SimpleTypeU>,
                                                                                        // >>::HEADER_SIZE
    );
    let encoded_value = {
        let mut buffer_encoder = WritableBufferImpl::<Endianness>::new(
            <ComplicatedType as Encoder<Endianness, ALIGN, ComplicatedType>>::HEADER_SIZE,
            None,
        );
        encoder_call!(@encode
            ComplicatedType,
            Endianness,
            ALIGN,
            &mut buffer_encoder,
            0,
            &value0,
        );
        buffer_encoder.finalize()
    };
    let fact = hex::encode(&encoded_value);
    let expected = "\
    02000000\
    0c000000\
    1c000000\
    6400000000000000\
    14000000\
    0300\
    ffffffffffffffff\
    ffffffff\
    ffff\
    ";
    assert_eq!(expected, fact);
    let mut buffer_decoder = ReadableBufferImpl::<Endianness>::new(encoded_value.as_slice());
    let mut value1 = Default::default();
    encoder_call!(@decode_body
        ComplicatedType,
        Endianness,
        ALIGN,
        &mut buffer_decoder,
        0,
        &mut value1
    );
    assert_eq!(value0, value1);
}

#[test]
fn test_complicated_type_alignment_32_be() {
    type Endianness = BE;
    const ALIGN: usize = ALIGN_32;
    define_codec_struct! {
        Endianness,
        ALIGN,
        pub struct SimpleTypeU {
            a: u64,
            b: u32,
            c: u16,
        }
    }
    define_codec_struct! {
        Endianness, ALIGN,
        pub struct ComplicatedType {
            values: Vec<SimpleTypeU>,
            // maps: HashMap<u32, ComplicatedType>,
        }
    }

    let value0 = ComplicatedType {
        values: vec![
            SimpleTypeU {
                a: 100,
                b: 20,
                c: 3,
            },
            SimpleTypeU {
                a: u64::MAX,
                b: u32::MAX,
                c: u16::MAX,
            },
        ],
        // maps: HashMap::from([(
        //     7,
        //     ComplicatedType {
        //         values: vec![
        //             SimpleTypeU { a: 1, b: 2, c: 3 },
        //             SimpleTypeU { a: 4, b: 5, c: 6 },
        //         ],
        //         maps: Default::default(),
        //     },
        // )]),
    };
    assert_eq!(
        <ComplicatedType as Encoder<Endianness, ALIGN, ComplicatedType>>::HEADER_SIZE,
        <Vec<SimpleTypeU> as Encoder<Endianness, ALIGN, Vec<SimpleTypeU>>>::HEADER_SIZE // + <HashMap::<u32, SimpleTypeU> as Encoder<
                                                                                        //     Endianness,
                                                                                        //     ALIGN,
                                                                                        //     HashMap::<u32, SimpleTypeU>,
                                                                                        // >>::HEADER_SIZE
    );
    let encoded_value = {
        let mut buffer_encoder = WritableBufferImpl::<Endianness>::new(
            <ComplicatedType as Encoder<Endianness, ALIGN, ComplicatedType>>::HEADER_SIZE,
            None,
        );
        encoder_call!(@encode
            ComplicatedType,
            Endianness,
            ALIGN,
            &mut buffer_encoder,
            0,
            &value0,
        );
        buffer_encoder.finalize()
    };
    let fact = hex::encode(&encoded_value);
    let expected = "\
    0000000000000000000000000000000000000000000000000000000000000002\
    0000000000000000000000000000000000000000000000000000000000000060\
    00000000000000000000000000000000000000000000000000000000000000c0\
    0000000000000000000000000000000000000000000000000000000000000064\
    0000000000000000000000000000000000000000000000000000000000000014\
    0000000000000000000000000000000000000000000000000000000000000003\
    000000000000000000000000000000000000000000000000ffffffffffffffff\
    00000000000000000000000000000000000000000000000000000000ffffffff\
    000000000000000000000000000000000000000000000000000000000000ffff";
    assert_eq!(expected, fact);
    let mut buffer_decoder = ReadableBufferImpl::<Endianness>::new(encoded_value.as_slice());
    let mut value1 = Default::default();
    encoder_call!(@decode_body
        ComplicatedType,
        Endianness,
        ALIGN,
        &mut buffer_decoder,
        0,
        &mut value1
    );
    assert_eq!(value0, value1);
}

// #[test]
// fn test_solidity_abi_compatible_1() {
//     type Endianness = BE;
//     const ALIGN: usize = ALIGN_32;
//
//     type Bytes = Vec<u8>;
//     type UInt = B256;
//     define_codec_struct! {
//         Endianness,
//         ALIGN,
//         pub struct Inner {
//             x: Bytes,
//             // y: [UInt; 3],
//         }
//     }
//     define_codec_struct! {
//         Endianness,
//         ALIGN,
//         pub struct FuncParams {
//             // a: bool,
//             b: Inner,
//             // c: [UInt; 2],
//         }
//     }
//     let value0 = FuncParams {
//         // TODO need right alignment for simple types
//         // a: true,
//         b: Inner {
//             x: "abcd".to_string().into_bytes(),
//             // y: [
//             //     UInt::left_padding_from(&[11]),
//             //     UInt::left_padding_from(&[12]),
//             //     UInt::left_padding_from(&[13]),
//             // ],
//         },
//         // c: [
//         //     UInt::right_padding_from("a".as_bytes()),
//         //     UInt::right_padding_from("b".as_bytes()),
//         // ],
//     };
//
//     // assert_eq!(
//     //     <Inner as Encoder<Endianness, ALIGN, Inner>>::HEADER_SIZE,
//     //     <i64 as Encoder<Endianness, ALIGN, i64>>::HEADER_SIZE
//     //         + <i32 as Encoder<Endianness, ALIGN, i32>>::HEADER_SIZE
//     //         + <i16 as Encoder<Endianness, ALIGN, i16>>::HEADER_SIZE
//     // );
//     let encoded_value = {
//         let hs = <FuncParams as Encoder<Endianness, ALIGN, FuncParams>>::HEADER_SIZE;
//         let mut buffer_encoder = WritableBufferImpl::<Endianness, ALIGN>::new(hs, None);
//         call_encode!(
//             FuncParams,
//             Endianness,
//             ALIGN,
//             &value0,
//             &mut buffer_encoder,
//             0
//         );
//         buffer_encoder.finalize()
//     };
//     let encoded_value_len = encoded_value.len();
//     // expected:
//     // 0: 0x0000000000000000000000000000000000000000000000000000000000000001 a
//     // 1: 0x0000000000000000000000000000000000000000000000000000000000000080 offset of b
//     // 2: 0x6100000000000000000000000000000000000000000000000000000000000000 c[0]
//     // 3: 0x6200000000000000000000000000000000000000000000000000000000000000 c[1]
//     // 4: 0x0000000000000000000000000000000000000000000000000000000000000080 offset of b.x
//     // 5: 0x000000000000000000000000000000000000000000000000000000000000000b b.y[0]
//     // 6: 0x000000000000000000000000000000000000000000000000000000000000000c b.y[1]
//     // 7: 0x000000000000000000000000000000000000000000000000000000000000000d b.y[2]
//     // 8: 0x0000000000000000000000000000000000000000000000000000000000000004 b.x (length field)
//     // 9: 0x6162636400000000000000000000000000000000000000000000000000000000 b.x (data)
//     let expected = "\
//         0000000000000000000000000000000000000000000000000000000000000001\
//         0000000000000000000000000000000000000000000000000000000000000080\
//         6100000000000000000000000000000000000000000000000000000000000000\
//         6200000000000000000000000000000000000000000000000000000000000000\
//         0000000000000000000000000000000000000000000000000000000000000080\
//         000000000000000000000000000000000000000000000000000000000000000b\
//         000000000000000000000000000000000000000000000000000000000000000c\
//         000000000000000000000000000000000000000000000000000000000000000d\
//         0000000000000000000000000000000000000000000000000000000000000004\
//         6162636400000000000000000000000000000000000000000000000000000000\
//         ";
//     // fact:
//     // 0: 0100000000000000000000000000000000000000000000000000000000000000
//     // 1: 0000000400000000000000000000000000000000000000000000000000000000
//     // 2: 0000012000000000000000000000000000000000000000000000000000000000
//     // 3: 0000002000000000000000000000000000000000000000000000000000000000
//     // 4: 000000000000000000000000000000000000000000000000000000000000000b
//     // 5: 000000000000000000000000000000000000000000000000000000000000000c
//     // 6: 000000000000000000000000000000000000000000000000000000000000000d
//     // 7: 6100000000000000000000000000000000000000000000000000000000000000
//     // 8: 6200000000000000000000000000000000000000000000000000000000000000
//     // 9: 6162636400000000000000000000000000000000000000000000000000000000
//
//     let fact = hex::encode(&encoded_value);
//     for (i, v) in encoded_value.as_slice().chunks(ALIGN).enumerate() {
//         let chunk_encoded = hex::encode(v);
//         println!("{i}: {chunk_encoded}")
//     }
//     let fact_len = fact.len();
//     let fact_items_aligned_count = fact_len / ALIGN / 2;
//     println!("fact len {fact_len} fact aligned items count (align considered): {fact_items_aligned_count}");
//     assert_eq!(expected, fact);
//     let mut buffer_decoder = ReadableBufferImpl::<Endianness>::new(encoded_value.as_slice());
//     let mut value1 = Default::default();
//     call_decode_body!(
//         FuncParams,
//         Endianness,
//         ALIGN,
//         &mut buffer_decoder,
//         0,
//         &mut value1
//     );
//     assert_eq!(value0, value1);
// }
