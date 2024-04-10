use crate::define_codec_struct;

#[cfg(test)]
mod tests {
    use alloy_primitives::{B256, U256};
    use byteorder::{BE, LE};
    use hashbrown::HashMap;

    use crate::encoder::{ALIGNMENT_32, ALIGNMENT_DEFAULT};
    use crate::{define_codec_struct, header_item_size, BufferDecoder, BufferEncoder, Encoder};

    #[test]
    fn test_simple_type_alignment_default_u_le() {
        type Endianness = LE;
        const ALIGNMENT: usize = ALIGNMENT_DEFAULT;
        let header_item_size = header_item_size!(ALIGNMENT);

        define_codec_struct! {
            Endianness,
            ALIGNMENT,
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
            <SimpleTypeU as Encoder<Endianness, ALIGNMENT, SimpleTypeU>>::HEADER_SIZE,
            <u64 as Encoder<Endianness, ALIGNMENT, u64>>::HEADER_SIZE
                + <u32 as Encoder<Endianness, ALIGNMENT, u32>>::HEADER_SIZE
                + <u16 as Encoder<Endianness, ALIGNMENT, u16>>::HEADER_SIZE
        );
        let encoded_value = {
            let mut buffer_encoder = BufferEncoder::<Endianness, ALIGNMENT>::new(
                <SimpleTypeU as Encoder<Endianness, ALIGNMENT, SimpleTypeU>>::HEADER_SIZE,
                None,
            );
            value0.encode(&mut buffer_encoder, 0);
            buffer_encoder.finalize()
        };
        println!("{}", hex::encode(&encoded_value));
        let mut buffer_decoder =
            BufferDecoder::<Endianness, ALIGNMENT>::new(encoded_value.as_slice());
        let mut value1 = Default::default();
        SimpleTypeU::decode_body(&mut buffer_decoder, 0, &mut value1);
        assert_eq!(value0, value1);
    }

    #[test]
    fn test_simple_type_alignment_32_u_be() {
        type Endianness = BE;
        const ALIGNMENT: usize = ALIGNMENT_32;
        let header_item_size = header_item_size!(ALIGNMENT);

        define_codec_struct! {
            Endianness,
            ALIGNMENT,
            pub struct SimpleTypeU {
                a: u64,
                b: u32,
                c: u16,
                boo: u8,
            }
        }

        let value0 = SimpleTypeU {
            a: 100,
            b: 20,
            c: 3,
            boo: 1,
        };
        assert_eq!(
            <SimpleTypeU as Encoder<Endianness, ALIGNMENT, SimpleTypeU>>::HEADER_SIZE,
            <u64 as Encoder<Endianness, ALIGNMENT, u64>>::HEADER_SIZE
                + <u32 as Encoder<Endianness, ALIGNMENT, u32>>::HEADER_SIZE
                + <u16 as Encoder<Endianness, ALIGNMENT, u16>>::HEADER_SIZE
                + <bool as Encoder<Endianness, ALIGNMENT, bool>>::HEADER_SIZE
        );
        let encoded_value = {
            let mut buffer_encoder = BufferEncoder::<Endianness, ALIGNMENT>::new(
                <SimpleTypeU as Encoder<Endianness, ALIGNMENT, SimpleTypeU>>::HEADER_SIZE,
                None,
            );
            value0.encode(&mut buffer_encoder, 0);
            buffer_encoder.finalize()
        };
        let expected = "\
        0000000000000064000000000000000000000000000000000000000000000000\
        0000001400000000000000000000000000000000000000000000000000000000\
        0003000000000000000000000000000000000000000000000000000000000000\
        0100000000000000000000000000000000000000000000000000000000000000\
        ";
        let fact = hex::encode(&encoded_value);
        assert_eq!(expected, fact);
        let mut buffer_decoder =
            BufferDecoder::<Endianness, ALIGNMENT>::new(encoded_value.as_slice());
        let mut value1 = Default::default();
        SimpleTypeU::decode_body(&mut buffer_decoder, 0, &mut value1);
        assert_eq!(value0, value1);
    }

    #[test]
    fn test_simple_type_s_alignment_default_le() {
        type Endianness = LE;
        const ALIGNMENT: usize = ALIGNMENT_DEFAULT;
        define_codec_struct! {
            Endianness,
            ALIGNMENT,
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
            <SimpleTypeS as Encoder<Endianness, ALIGNMENT, SimpleTypeS>>::HEADER_SIZE,
            <i64 as Encoder<Endianness, ALIGNMENT, i64>>::HEADER_SIZE
                + <i32 as Encoder<Endianness, ALIGNMENT, i32>>::HEADER_SIZE
                + <i16 as Encoder<Endianness, ALIGNMENT, i16>>::HEADER_SIZE
        );
        let encoded_value = {
            let mut buffer_encoder = BufferEncoder::<Endianness, ALIGNMENT>::new(
                <SimpleTypeS as Encoder<Endianness, ALIGNMENT, SimpleTypeS>>::HEADER_SIZE,
                None,
            );
            value0.encode(&mut buffer_encoder, 0);
            buffer_encoder.finalize()
        };
        let expected = "\
        9cffffffffffffff\
        ecffffff\
        fdff\
        ";
        let fact = hex::encode(&encoded_value);
        assert_eq!(expected, fact);
        let mut buffer_decoder =
            BufferDecoder::<Endianness, ALIGNMENT>::new(encoded_value.as_slice());
        let mut value1 = Default::default();
        SimpleTypeS::decode_body(&mut buffer_decoder, 0, &mut value1);
        assert_eq!(value0, value1);
    }

    #[test]
    fn test_simple_type_s_alignment_32_be() {
        type Endianness = BE;
        const ALIGNMENT: usize = ALIGNMENT_32;
        define_codec_struct! {
            Endianness,
            ALIGNMENT,
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
            <SimpleTypeS as Encoder<Endianness, ALIGNMENT, SimpleTypeS>>::HEADER_SIZE,
            <i64 as Encoder<Endianness, ALIGNMENT, i64>>::HEADER_SIZE
                + <i32 as Encoder<Endianness, ALIGNMENT, i32>>::HEADER_SIZE
                + <i16 as Encoder<Endianness, ALIGNMENT, i16>>::HEADER_SIZE
        );
        let encoded_value = {
            let mut buffer_encoder = BufferEncoder::<Endianness, ALIGNMENT>::new(
                <SimpleTypeS as Encoder<Endianness, ALIGNMENT, SimpleTypeS>>::HEADER_SIZE,
                None,
            );
            value0.encode(&mut buffer_encoder, 0);
            buffer_encoder.finalize()
        };
        let expected = "\
        ffffffffffffff9c000000000000000000000000000000000000000000000000\
        ffffffec00000000000000000000000000000000000000000000000000000000\
        fffd000000000000000000000000000000000000000000000000000000000000\
        ";
        let fact = hex::encode(&encoded_value);
        assert_eq!(expected, fact);
        let mut buffer_decoder =
            BufferDecoder::<Endianness, ALIGNMENT>::new(encoded_value.as_slice());
        let mut value1 = Default::default();
        SimpleTypeS::decode_body(&mut buffer_decoder, 0, &mut value1);
        assert_eq!(value0, value1);
    }

    #[test]
    fn test_decode_specific_field_alignment_default_le() {
        type Endianness = LE;
        const ALIGNMENT: usize = ALIGNMENT_DEFAULT;
        define_codec_struct! {
            Endianness,
            ALIGNMENT,
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
            header_item_size!(ALIGNMENT, u64),
            <SimpleTypeU as ISimpleTypeU>::B::FIELD_OFFSET,
        );
        assert_eq!(
            header_item_size!(ALIGNMENT, u64) + header_item_size!(ALIGNMENT, u32),
            <SimpleTypeU as ISimpleTypeU>::C::FIELD_OFFSET,
        );
        // check sizes
        assert_eq!(<SimpleTypeU as ISimpleTypeU>::A::FIELD_SIZE, 8);
        assert_eq!(<SimpleTypeU as ISimpleTypeU>::B::FIELD_SIZE, 4);
        assert_eq!(<SimpleTypeU as ISimpleTypeU>::C::FIELD_SIZE, 2);
        // encode entire struct
        let encoded_value =
            <SimpleTypeU as Encoder<Endianness, ALIGNMENT, SimpleTypeU>>::encode_to_vec(&value, 0);
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
        const ALIGNMENT: usize = ALIGNMENT_32;
        define_codec_struct! {
            Endianness,
            ALIGNMENT,
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
        // assert_eq!(
        //     <SimpleTypeU as ISimpleTypeU>::B::FIELD_SIZE,
        //     header_item_size!(ALIGNMENT, u64)
        // );
        assert_eq!(
            <SimpleTypeU as ISimpleTypeU>::B::FIELD_OFFSET,
            header_item_size!(ALIGNMENT, u64)
        );
        assert_eq!(
            <SimpleTypeU as ISimpleTypeU>::C::FIELD_OFFSET,
            header_item_size!(ALIGNMENT, u64) + header_item_size!(ALIGNMENT, u32)
        );
        // check sizes
        assert_eq!(
            <SimpleTypeU as ISimpleTypeU>::A::FIELD_SIZE,
            header_item_size!(ALIGNMENT, u64)
        );
        assert_eq!(
            <SimpleTypeU as ISimpleTypeU>::B::FIELD_SIZE,
            header_item_size!(ALIGNMENT, u32)
        );
        assert_eq!(
            <SimpleTypeU as ISimpleTypeU>::C::FIELD_SIZE,
            header_item_size!(ALIGNMENT, u16)
        );
        // encode entire struct
        let encoded_value =
            <SimpleTypeU as Encoder<Endianness, ALIGNMENT, SimpleTypeU>>::encode_to_vec(&value, 0);
        let mut encoded_value = encoded_value.as_slice();
        let encoded_value_hex = hex::encode(encoded_value);
        let expected_encoded_value_hex = "\
        0000000000000064000000000000000000000000000000000000000000000000\
        0000001400000000000000000000000000000000000000000000000000000000\
        0003000000000000000000000000000000000000000000000000000000000000";
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

    #[test]
    fn test_complicated_type_alignment_default_le() {
        type Endianness = LE;
        const ALIGNMENT: usize = ALIGNMENT_DEFAULT;
        define_codec_struct! {
            Endianness,
            ALIGNMENT,
            pub struct SimpleTypeU {
                a: u64,
                b: u32,
                c: u16,
            }
        }
        define_codec_struct! {
            Endianness, ALIGNMENT,
            pub struct ComplicatedType {
                values: Vec<SimpleTypeU>,
                maps: HashMap<u32, ComplicatedType>,
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
            maps: HashMap::from([(
                7,
                ComplicatedType {
                    values: vec![
                        SimpleTypeU { a: 1, b: 2, c: 3 },
                        SimpleTypeU { a: 4, b: 5, c: 6 },
                    ],
                    maps: Default::default(),
                },
            )]),
        };
        assert_eq!(
            <ComplicatedType as Encoder<Endianness, ALIGNMENT, ComplicatedType>>::HEADER_SIZE,
            <Vec<SimpleTypeU> as Encoder<Endianness, ALIGNMENT, Vec<SimpleTypeU>>>::HEADER_SIZE
                + <HashMap::<u32, SimpleTypeU> as Encoder<
                    Endianness,
                    ALIGNMENT,
                    HashMap::<u32, SimpleTypeU>,
                >>::HEADER_SIZE
        );
        let encoded_value = {
            let mut buffer_encoder = BufferEncoder::<Endianness, ALIGNMENT>::new(
                <ComplicatedType as Encoder<Endianness, ALIGNMENT, ComplicatedType>>::HEADER_SIZE,
                None,
            );
            value0.encode(&mut buffer_encoder, 0);
            buffer_encoder.finalize()
        };
        let fact = hex::encode(&encoded_value);
        let expected = "02000000200000001c000000010000003c00000004000000400000003c0000006400000000000000140000000300ffffffffffffffffffffffffffff0700000002000000200000001c000000000000003c000000000000003c0000000000000001000000000000000200000003000400000000000000050000000600";
        assert_eq!(expected, fact);
        let mut buffer_decoder =
            BufferDecoder::<Endianness, ALIGNMENT>::new(encoded_value.as_slice());
        let mut value1 = Default::default();
        ComplicatedType::decode_body(&mut buffer_decoder, 0, &mut value1);
        assert_eq!(value0, value1);
    }

    #[ignore] // TODO
    #[test]
    fn test_complicated_type_alignment_32_be() {
        type Endianness = BE;
        const ALIGNMENT: usize = ALIGNMENT_32;
        define_codec_struct! {
            Endianness,
            ALIGNMENT,
            pub struct SimpleTypeU {
                a: u64,
                b: u32,
                c: u16,
            }
        }
        define_codec_struct! {
            Endianness, ALIGNMENT,
            pub struct ComplicatedType {
                values: Vec<SimpleTypeU>,
                maps: HashMap<u32, ComplicatedType>,
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
            maps: HashMap::from([(
                7,
                ComplicatedType {
                    values: vec![
                        SimpleTypeU { a: 1, b: 2, c: 3 },
                        SimpleTypeU { a: 4, b: 5, c: 6 },
                    ],
                    maps: Default::default(),
                },
            )]),
        };
        assert_eq!(
            <ComplicatedType as Encoder<Endianness, ALIGNMENT, ComplicatedType>>::HEADER_SIZE,
            <Vec<SimpleTypeU> as Encoder<Endianness, ALIGNMENT, Vec<SimpleTypeU>>>::HEADER_SIZE
                + <HashMap::<u32, SimpleTypeU> as Encoder<
                    Endianness,
                    ALIGNMENT,
                    HashMap::<u32, SimpleTypeU>,
                >>::HEADER_SIZE
        );
        let encoded_value = {
            let mut buffer_encoder = BufferEncoder::<Endianness, ALIGNMENT>::new(
                <ComplicatedType as Encoder<Endianness, ALIGNMENT, ComplicatedType>>::HEADER_SIZE,
                None,
            );
            value0.encode(&mut buffer_encoder, 0);
            buffer_encoder.finalize()
        };
        let fact = hex::encode(&encoded_value);
        let expected = "\
        00000002000000ac000000000000000100000000000000000000000000000000\
        0000000000000000000000000000016c00000000000000000000000000000000\
        0000000000000000000000000000002000000000000000000000000000000000\
        0000000000000000000000000000018c00000000000000000000000000000000\
        0000000000000000000000000000018000000000000000000000000000000000\
        0000000000000000000000000000000000000064000000000000000000000000\
        0000000000000000000000000000001400000000000000000000000000000000\
        0000000000000000000000000003000000000000000000000000000000000000\
        000000000000000000000000ffffffffffffffff000000000000000000000000\
        000000000000000000000000ffffffff00000000000000000000000000000000\
        000000000000000000000000ffff000000000000000000000000000000000000\
        0000000000000000000000000000000700000000000000000000000000000000\
        00000000000000000000000000000002000000ac000000000000000000000000\
        0000000000000000000000000000000000000000000000000000016c00000000\
        0000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000016c00000000\
        0000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000001\
        0000000000000000000000000000000000000000000000000000000200000000\
        0000000000000000000000000000000000000000000000000003000000000000\
        0000000000000000000000000000000000000000000000000000000000000004\
        0000000000000000000000000000000000000000000000000000000500000000\
        0000000000000000000000000000000000000000000000000006000000000000\
        0000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000";
        assert_eq!(expected, fact);
        let mut buffer_decoder =
            BufferDecoder::<Endianness, ALIGNMENT>::new(encoded_value.as_slice());
        let mut value1 = Default::default();
        ComplicatedType::decode_body(&mut buffer_decoder, 0, &mut value1);
        assert_eq!(value0, value1);
    }

    #[test]
    fn test_solidity_abi_compatible_1() {
        type Endianness = BE;
        const ALIGNMENT: usize = ALIGNMENT_32;

        type Bytes = Vec<u8>;
        type UInt = [u8; 32];
        define_codec_struct! {
            Endianness,
            ALIGNMENT,
            pub struct Inner {
                x: Bytes,
                y: [UInt; 3],
            }
        }
        define_codec_struct! {
            Endianness,
            ALIGNMENT,
            pub struct FuncParams {
                // a: bool,
                // b: Inner,
                c: [UInt; 2],
            }
        }
        let value0 = FuncParams {
            // a: false,
            // b: Inner {
            //     x: "abcd".to_string().into_bytes(),
            //     y: [UInt::default(), UInt::default(), UInt::default()],
            // },
            c: [
                UInt::default(),
                [
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                ],
            ],
        };

        // assert_eq!(
        //     <Inner as Encoder<Endianness, ALIGNMENT, Inner>>::HEADER_SIZE,
        //     <i64 as Encoder<Endianness, ALIGNMENT, i64>>::HEADER_SIZE
        //         + <i32 as Encoder<Endianness, ALIGNMENT, i32>>::HEADER_SIZE
        //         + <i16 as Encoder<Endianness, ALIGNMENT, i16>>::HEADER_SIZE
        // );
        let encoded_value = {
            let mut buffer_encoder = BufferEncoder::<Endianness, ALIGNMENT>::new(
                <FuncParams as Encoder<Endianness, ALIGNMENT, FuncParams>>::HEADER_SIZE,
                None,
            );
            value0.encode(&mut buffer_encoder, 0);
            buffer_encoder.finalize()
        };
        let encoded_value_len = encoded_value.len();
        // 0x0000000000000000000000000000000000000000000000000000000000000001 a
        // 0x0000000000000000000000000000000000000000000000000000000000000080 offset of b
        // 0x6100000000000000000000000000000000000000000000000000000000000000 c[0]
        // 0x6200000000000000000000000000000000000000000000000000000000000000 c[1]
        // 0x0000000000000000000000000000000000000000000000000000000000000080 offset of b.x
        // 0x000000000000000000000000000000000000000000000000000000000000000b b.y[0]
        // 0x000000000000000000000000000000000000000000000000000000000000000c b.y[1]
        // 0x000000000000000000000000000000000000000000000000000000000000000d b.y[2]
        // 0x0000000000000000000000000000000000000000000000000000000000000004 b.x (length field)
        // 0x6162636400000000000000000000000000000000000000000000000000000000 b.x (data)
        let expected = "\
        0000000000000000000000000000000000000000000000000000000000000001\
        0000000000000000000000000000000000000000000000000000000000000080\
        6100000000000000000000000000000000000000000000000000000000000000\
        6200000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000080\
        000000000000000000000000000000000000000000000000000000000000000b\
        000000000000000000000000000000000000000000000000000000000000000c\
        000000000000000000000000000000000000000000000000000000000000000d\
        0000000000000000000000000000000000000000000000000000000000000004\
        6162636400000000000000000000000000000000000000000000000000000000\
        ";
        let current = "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
        let fact = hex::encode(&encoded_value);
        assert_eq!(expected, fact);
        let mut buffer_decoder =
            BufferDecoder::<Endianness, ALIGNMENT>::new(encoded_value.as_slice());
        let mut value1 = Default::default();
        FuncParams::decode_body(&mut buffer_decoder, 0, &mut value1);
        assert_eq!(value0, value1);
    }
}
