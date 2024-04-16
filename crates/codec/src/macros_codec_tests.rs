use byteorder::LE;

use crate::encoder::{Encoder, FieldEncoder, ALIGN_DEFAULT};
use crate::{define_codec_struct, header_item_size};

#[test]
fn test_simple_type_alignment_default_u_le() {
    type Endianness = LE;
    const ALIGN: usize = ALIGN_DEFAULT;
    let header_item_size = header_item_size!(ALIGN);

    define_codec_struct! {
        Endianness,
        ALIGN,
        pub struct SimpleTypeU {
            a: u64,
            b: u32,
            c: u16,
        }
    }

    let value0 = SimpleTypeU {
        a: 100,
        b: 20,
        c: 3,
    };
    assert_eq!(
        <SimpleTypeU as Encoder<Endianness, ALIGN, SimpleTypeU>>::HEADER_SIZE,
        <u64 as Encoder<Endianness, ALIGN, u64>>::HEADER_SIZE
            + <u32 as Encoder<Endianness, ALIGN, u32>>::HEADER_SIZE
            + <u16 as Encoder<Endianness, ALIGN, u16>>::HEADER_SIZE
    );
    // let encoded_value = {
    //     let mut buffer_encoder = BufferEncoder::<Endianness, ALIGN>::new(
    //         <SimpleTypeU as Encoder<Endianness, ALIGN, SimpleTypeU>>::HEADER_SIZE,
    //         None,
    //     );
    //     <SimpleTypeU as Encoder<Endianness, ALIGN, SimpleTypeU>>::encode::<
    //         BufferEncoder<Endianness, 0>,
    //     >(&value0, &mut buffer_encoder, 0);
    //     buffer_encoder.finalize()
    // };
    // println!("{}", hex::encode(&encoded_value));
    // let mut buffer_decoder = BufferDecoder::<Endianness>::new(encoded_value.as_slice());
    // let mut value1 = Default::default();
    // <SimpleTypeU as Encoder<Endianness, ALIGN, SimpleTypeU>>::decode_body(
    //     &mut buffer_decoder,
    //     0,
    //     &mut value1,
    // );
    // assert_eq!(value0, value1);
}

// #[test]
// fn test_simple_type_alignment_32_u_be() {
//     type Endianness = BE;
//     const ALIGN: usize = ALIGN_32;
//     let header_item_size = header_item_size!(ALIGN);
//
//     define_codec_struct! {
//         Endianness,
//         ALIGN,
//         pub struct SimpleTypeU {
//             a: u64,
//             b: u32,
//             c: u16,
//             boo: u8,
//         }
//     }
//
//     let value0 = SimpleTypeU {
//         a: 100,
//         b: 20,
//         c: 3,
//         boo: 1,
//     };
//     assert_eq!(
//         <SimpleTypeU as Encoder<Endianness, ALIGN, SimpleTypeU>>::HEADER_SIZE,
//         <u64 as Encoder<Endianness, ALIGN, u64>>::HEADER_SIZE
//             + <u32 as Encoder<Endianness, ALIGN, u32>>::HEADER_SIZE
//             + <u16 as Encoder<Endianness, ALIGN, u16>>::HEADER_SIZE
//             + <bool as Encoder<Endianness, ALIGN, bool>>::HEADER_SIZE
//     );
//     let encoded_value = {
//         let mut buffer_encoder = BufferEncoder::<Endianness, ALIGN>::new(
//             // <SimpleTypeU as Encoder<Endianness, ALIGN, SimpleTypeU>>::HEADER_SIZE,
//             header_size!(SimpleTypeU, Endianness, ALIGN),
//             None,
//         );
//         <SimpleTypeU as Encoder<Endianness, ALIGN, SimpleTypeU>>::encode(
//             &value0,
//             &mut buffer_encoder,
//             0,
//         );
//         buffer_encoder.finalize()
//     };
//     let expected = "\
//         0000000000000064000000000000000000000000000000000000000000000000\
//         0000001400000000000000000000000000000000000000000000000000000000\
//         0003000000000000000000000000000000000000000000000000000000000000\
//         0100000000000000000000000000000000000000000000000000000000000000\
//         ";
//     let fact = hex::encode(&encoded_value);
//     assert_eq!(expected, fact);
//     let mut buffer_decoder = BufferDecoder::<Endianness>::new(encoded_value.as_slice());
//     let mut value1 = Default::default();
//     call_decode_body!(
//         SimpleTypeU,
//         Endianness,
//         ALIGN,
//         &mut buffer_decoder,
//         0,
//         &mut value1
//     );
//     assert_eq!(value0, value1);
// }
//
// #[test]
// fn test_simple_type_s_alignment_default_le() {
//     type Endianness = LE;
//     const ALIGN: usize = ALIGN_DEFAULT;
//     define_codec_struct! {
//         Endianness,
//         ALIGN,
//         pub struct SimpleTypeS {
//             a: i64,
//             b: i32,
//             c: i16,
//         }
//     }
//     let value0 = SimpleTypeS {
//         a: -100,
//         b: -20,
//         c: -3,
//     };
//     assert_eq!(
//         <SimpleTypeS as Encoder<Endianness, ALIGN, SimpleTypeS>>::HEADER_SIZE,
//         <i64 as Encoder<Endianness, ALIGN, i64>>::HEADER_SIZE
//             + <i32 as Encoder<Endianness, ALIGN, i32>>::HEADER_SIZE
//             + <i16 as Encoder<Endianness, ALIGN, i16>>::HEADER_SIZE
//     );
//     let encoded_value = {
//         let mut buffer_encoder = BufferEncoder::<Endianness, ALIGN>::new(
//             <SimpleTypeS as Encoder<Endianness, ALIGN, SimpleTypeS>>::HEADER_SIZE,
//             None,
//         );
//         call_encode!(
//             SimpleTypeS,
//             Endianness,
//             ALIGN,
//             &value0,
//             &mut buffer_encoder,
//             0
//         );
//         buffer_encoder.finalize()
//     };
//     let expected = "\
//         9cffffffffffffff\
//         ecffffff\
//         fdff\
//         ";
//     let fact = hex::encode(&encoded_value);
//     assert_eq!(expected, fact);
//     let mut buffer_decoder = BufferDecoder::<Endianness>::new(encoded_value.as_slice());
//     let mut value1 = Default::default();
//     call_decode_body!(
//         SimpleTypeS,
//         Endianness,
//         ALIGN,
//         &mut buffer_decoder,
//         0,
//         &mut value1
//     );
//     assert_eq!(value0, value1);
// }
//
// #[test]
// fn test_simple_type_s_alignment_32_be() {
//     type Endianness = BE;
//     const ALIGN: usize = ALIGN_32;
//     define_codec_struct! {
//         Endianness,
//         ALIGN,
//         pub struct SimpleTypeS {
//             a: i64,
//             b: i32,
//             c: i16,
//         }
//     }
//     let value0 = SimpleTypeS {
//         a: -100,
//         b: -20,
//         c: -3,
//     };
//
//     assert_eq!(
//         <SimpleTypeS as Encoder<Endianness, ALIGN, SimpleTypeS>>::HEADER_SIZE,
//         <i64 as Encoder<Endianness, ALIGN, i64>>::HEADER_SIZE
//             + <i32 as Encoder<Endianness, ALIGN, i32>>::HEADER_SIZE
//             + <i16 as Encoder<Endianness, ALIGN, i16>>::HEADER_SIZE
//     );
//     let encoded_value = {
//         let mut buffer_encoder = BufferEncoder::<Endianness, ALIGN>::new(
//             <SimpleTypeS as Encoder<Endianness, ALIGN, SimpleTypeS>>::HEADER_SIZE,
//             None,
//         );
//         call_encode!(
//             SimpleTypeS,
//             Endianness,
//             ALIGN,
//             &value0,
//             &mut buffer_encoder,
//             0
//         );
//         buffer_encoder.finalize()
//     };
//     let expected = "\
//         ffffffffffffff9c000000000000000000000000000000000000000000000000\
//         ffffffec00000000000000000000000000000000000000000000000000000000\
//         fffd000000000000000000000000000000000000000000000000000000000000\
//         ";
//     let fact = hex::encode(&encoded_value);
//     assert_eq!(expected, fact);
//     let mut buffer_decoder = BufferDecoder::<Endianness>::new(encoded_value.as_slice());
//     let mut value1 = Default::default();
//     call_decode_body!(
//         SimpleTypeS,
//         Endianness,
//         ALIGN,
//         &mut buffer_decoder,
//         0,
//         &mut value1
//     );
//     assert_eq!(value0, value1);
// }
//
// #[test]
// fn test_decode_specific_field_alignment_default_le() {
//     type Endianness = LE;
//     const ALIGN: usize = ALIGN_DEFAULT;
//     define_codec_struct! {
//         Endianness,
//         ALIGN,
//         pub struct SimpleTypeU {
//             a: u64,
//             b: u32,
//             c: u16,
//         }
//     }
//     let value = SimpleTypeU {
//         a: 100,
//         b: 20,
//         c: 3,
//     };
//     // check offsets
//     assert_eq!(<SimpleTypeU as ISimpleTypeU>::A::FIELD_OFFSET, 0);
//     assert_eq!(
//         header_item_size!(ALIGN, u64),
//         <SimpleTypeU as ISimpleTypeU>::B::FIELD_OFFSET,
//     );
//     assert_eq!(
//         header_item_size!(ALIGN, u64) + header_item_size!(ALIGN, u32),
//         <SimpleTypeU as ISimpleTypeU>::C::FIELD_OFFSET,
//     );
//     // check sizes
//     assert_eq!(<SimpleTypeU as ISimpleTypeU>::A::FIELD_SIZE, 8);
//     assert_eq!(<SimpleTypeU as ISimpleTypeU>::B::FIELD_SIZE, 4);
//     assert_eq!(<SimpleTypeU as ISimpleTypeU>::C::FIELD_SIZE, 2);
//     // encode entire struct
//     let encoded_value =
//         <SimpleTypeU as Encoder<Endianness, ALIGN, SimpleTypeU>>::encode_to_vec(&value, 0);
//     let mut encoded_value = encoded_value.as_slice();
//     // decode only field `a`
//     {
//         let mut a: u64 = 0;
//         <SimpleTypeU as ISimpleTypeU>::A::decode_field_header(&mut encoded_value, &mut a);
//         assert_eq!(a, value.a);
//     }
//     // decode only field `b`
//     {
//         let mut b: u32 = 0;
//         <SimpleTypeU as ISimpleTypeU>::B::decode_field_header(&mut encoded_value, &mut b);
//         assert_eq!(b, value.b);
//     }
//     // decode only field `c`
//     {
//         let mut c: u16 = 0;
//         <SimpleTypeU as ISimpleTypeU>::C::decode_field_header(&mut encoded_value, &mut c);
//         assert_eq!(c, value.c);
//     }
// }
//
// #[test]
// fn test_decode_specific_field_alignment_32_be() {
//     type Endianness = BE;
//     const ALIGN: usize = ALIGN_32;
//     define_codec_struct! {
//         Endianness,
//         ALIGN,
//         pub struct SimpleTypeU {
//             a: u64,
//             b: u32,
//             c: u16,
//         }
//     }
//     let value = SimpleTypeU {
//         a: 100,
//         b: 20,
//         c: 3,
//     };
//     // check offsets
//     assert_eq!(<SimpleTypeU as ISimpleTypeU>::A::FIELD_OFFSET, 0);
//     // assert_eq!(
//     //     <SimpleTypeU as ISimpleTypeU>::B::FIELD_SIZE,
//     //     header_item_size!(ALIGN, u64)
//     // );
//     assert_eq!(
//         <SimpleTypeU as ISimpleTypeU>::B::FIELD_OFFSET,
//         header_item_size!(ALIGN, u64)
//     );
//     assert_eq!(
//         <SimpleTypeU as ISimpleTypeU>::C::FIELD_OFFSET,
//         header_item_size!(ALIGN, u64) + header_item_size!(ALIGN, u32)
//     );
//     // check sizes
//     assert_eq!(
//         <SimpleTypeU as ISimpleTypeU>::A::FIELD_SIZE,
//         header_item_size!(ALIGN, u64)
//     );
//     assert_eq!(
//         <SimpleTypeU as ISimpleTypeU>::B::FIELD_SIZE,
//         header_item_size!(ALIGN, u32)
//     );
//     assert_eq!(
//         <SimpleTypeU as ISimpleTypeU>::C::FIELD_SIZE,
//         header_item_size!(ALIGN, u16)
//     );
//     // encode entire struct
//     let encoded_value =
//         <SimpleTypeU as Encoder<Endianness, ALIGN, SimpleTypeU>>::encode_to_vec(&value, 0);
//     let mut encoded_value = encoded_value.as_slice();
//     let encoded_value_hex = hex::encode(encoded_value);
//     let expected_encoded_value_hex = "\
//         0000000000000064000000000000000000000000000000000000000000000000\
//         0000001400000000000000000000000000000000000000000000000000000000\
//         0003000000000000000000000000000000000000000000000000000000000000";
//     assert_eq!(expected_encoded_value_hex, encoded_value_hex);
//     // decode only field `a`
//     {
//         let mut a: u64 = 0;
//         <SimpleTypeU as ISimpleTypeU>::A::decode_field_header(&mut encoded_value, &mut a);
//         assert_eq!(a, value.a);
//     }
//     // decode only field `b`
//     {
//         let mut b: u32 = 0;
//         <SimpleTypeU as ISimpleTypeU>::B::decode_field_header(&mut encoded_value, &mut b);
//         assert_eq!(b, value.b);
//     }
//     // decode only field `c`
//     {
//         let mut c: u16 = 0;
//         <SimpleTypeU as ISimpleTypeU>::C::decode_field_header(&mut encoded_value, &mut c);
//         assert_eq!(c, value.c);
//     }
// }
//
// #[test]
// fn test_complicated_type_alignment_default_le() {
//     type Endianness = LE;
//     const ALIGN: usize = ALIGN_DEFAULT;
//     define_codec_struct! {
//         Endianness,
//         ALIGN,
//         pub struct SimpleTypeU {
//             a: u64,
//             b: u32,
//             c: u16,
//         }
//     }
//     define_codec_struct! {
//         Endianness, ALIGN,
//         pub struct ComplicatedType {
//             values: Vec<SimpleTypeU>,
//             maps: HashMap<u32, ComplicatedType>,
//         }
//     }
//
//     let value0 = ComplicatedType {
//         values: vec![
//             SimpleTypeU {
//                 a: 100,
//                 b: 20,
//                 c: 3,
//             },
//             SimpleTypeU {
//                 a: u64::MAX,
//                 b: u32::MAX,
//                 c: u16::MAX,
//             },
//         ],
//         maps: HashMap::from([(
//             7,
//             ComplicatedType {
//                 values: vec![
//                     SimpleTypeU { a: 1, b: 2, c: 3 },
//                     SimpleTypeU { a: 4, b: 5, c: 6 },
//                 ],
//                 maps: Default::default(),
//             },
//         )]),
//     };
//     assert_eq!(
//         <ComplicatedType as Encoder<Endianness, ALIGN, ComplicatedType>>::HEADER_SIZE,
//         <Vec<SimpleTypeU> as Encoder<Endianness, ALIGN, Vec<SimpleTypeU>>>::HEADER_SIZE
//             + <HashMap::<u32, SimpleTypeU> as Encoder<
//                 Endianness,
//                 ALIGN,
//                 HashMap::<u32, SimpleTypeU>,
//             >>::HEADER_SIZE
//     );
//     let encoded_value = {
//         let mut buffer_encoder = BufferEncoder::<Endianness, ALIGN>::new(
//             <ComplicatedType as Encoder<Endianness, ALIGN, ComplicatedType>>::HEADER_SIZE,
//             None,
//         );
//         call_encode!(
//             ComplicatedType,
//             Endianness,
//             ALIGN,
//             &value0,
//             &mut buffer_encoder,
//             0
//         );
//         buffer_encoder.finalize()
//     };
//     let fact = hex::encode(&encoded_value);
//     let expected = "\
//         02000000200000001c000000010000003c00000004000000400000003c000000\
//         6400000000000000140000000300ffffffffffffffffffffffffffff07000000\
//         02000000200000001c000000000000003c000000000000003c00000000000000\
//         01000000000000000200000003000400000000000000050000000600";
//     assert_eq!(expected, fact);
//     let mut buffer_decoder = BufferDecoder::<Endianness>::new(encoded_value.as_slice());
//     let mut value1 = Default::default();
//     call_decode_body!(
//         ComplicatedType,
//         Endianness,
//         ALIGN,
//         &mut buffer_decoder,
//         0,
//         &mut value1
//     );
//     assert_eq!(value0, value1);
// }
//
// #[ignore] // TODO
// #[test]
// fn test_complicated_type_alignment_32_be() {
//     type Endianness = BE;
//     const ALIGN: usize = ALIGN_32;
//     define_codec_struct! {
//         Endianness,
//         ALIGN,
//         pub struct SimpleTypeU {
//             a: u64,
//             b: u32,
//             c: u16,
//         }
//     }
//     define_codec_struct! {
//         Endianness, ALIGN,
//         pub struct ComplicatedType {
//             values: Vec<SimpleTypeU>,
//             maps: HashMap<u32, ComplicatedType>,
//         }
//     }
//
//     let value0 = ComplicatedType {
//         values: vec![
//             SimpleTypeU {
//                 a: 100,
//                 b: 20,
//                 c: 3,
//             },
//             SimpleTypeU {
//                 a: u64::MAX,
//                 b: u32::MAX,
//                 c: u16::MAX,
//             },
//         ],
//         maps: HashMap::from([(
//             7,
//             ComplicatedType {
//                 values: vec![
//                     SimpleTypeU { a: 1, b: 2, c: 3 },
//                     SimpleTypeU { a: 4, b: 5, c: 6 },
//                 ],
//                 maps: Default::default(),
//             },
//         )]),
//     };
//     assert_eq!(
//         <ComplicatedType as Encoder<Endianness, ALIGN, ComplicatedType>>::HEADER_SIZE,
//         <Vec<SimpleTypeU> as Encoder<Endianness, ALIGN, Vec<SimpleTypeU>>>::HEADER_SIZE
//             + <HashMap::<u32, SimpleTypeU> as Encoder<
//                 Endianness,
//                 ALIGN,
//                 HashMap::<u32, SimpleTypeU>,
//             >>::HEADER_SIZE
//     );
//     let encoded_value = {
//         let mut buffer_encoder = BufferEncoder::<Endianness, ALIGN>::new(
//             <ComplicatedType as Encoder<Endianness, ALIGN, ComplicatedType>>::HEADER_SIZE,
//             None,
//         );
//         call_encode!(
//             ComplicatedType,
//             Endianness,
//             ALIGN,
//             &value0,
//             &mut buffer_encoder,
//             0
//         );
//         buffer_encoder.finalize()
//     };
//     let fact = hex::encode(&encoded_value);
//     let expected = "\
//         00000002000000ac000000000000000100000000000000000000000000000000\
//         0000000000000000000000000000016c00000000000000000000000000000000\
//         0000000000000000000000000000002000000000000000000000000000000000\
//         0000000000000000000000000000018c00000000000000000000000000000000\
//         0000000000000000000000000000018000000000000000000000000000000000\
//         0000000000000000000000000000000000000064000000000000000000000000\
//         0000000000000000000000000000001400000000000000000000000000000000\
//         0000000000000000000000000003000000000000000000000000000000000000\
//         000000000000000000000000ffffffffffffffff000000000000000000000000\
//         000000000000000000000000ffffffff00000000000000000000000000000000\
//         000000000000000000000000ffff000000000000000000000000000000000000\
//         0000000000000000000000000000000700000000000000000000000000000000\
//         00000000000000000000000000000002000000ac000000000000000000000000\
//         0000000000000000000000000000000000000000000000000000016c00000000\
//         0000000000000000000000000000000000000000000000000000000000000000\
//         0000000000000000000000000000000000000000000000000000016c00000000\
//         0000000000000000000000000000000000000000000000000000000000000000\
//         0000000000000000000000000000000000000000000000000000000000000001\
//         0000000000000000000000000000000000000000000000000000000200000000\
//         0000000000000000000000000000000000000000000000000003000000000000\
//         0000000000000000000000000000000000000000000000000000000000000004\
//         0000000000000000000000000000000000000000000000000000000500000000\
//         0000000000000000000000000000000000000000000000000006000000000000\
//         0000000000000000000000000000000000000000000000000000000000000000\
//         000000000000000000000000";
//     assert_eq!(expected, fact);
//     let mut buffer_decoder = BufferDecoder::<Endianness>::new(encoded_value.as_slice());
//     let mut value1 = Default::default();
//     call_decode_body!(
//         ComplicatedType,
//         Endianness,
//         ALIGN,
//         &mut buffer_decoder,
//         0,
//         &mut value1
//     );
//     assert_eq!(value0, value1);
// }
//
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
//         let mut buffer_encoder = BufferEncoder::<Endianness, ALIGN>::new(hs, None);
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
//     let mut buffer_decoder = BufferDecoder::<Endianness>::new(encoded_value.as_slice());
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
