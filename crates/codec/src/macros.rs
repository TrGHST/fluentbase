#[macro_export]
macro_rules! derive_header_size {
    () => (0);
    ($endianness:ty, $alignment:ident, $val:ident: $typ:ty) => {
        <$typ as $crate::Encoder<E, $alignment, $typ>>::HEADER_SIZE
    };
    ($endianness:ty, $alignment:ident, $val_x:ident:$typ_x:ty, $($val_y:ident:$typ_y:ty),+ $(,)?) => {
        $crate::derive_header_size!($endianness, $alignment, $val_x:$typ_x) + $crate::derive_header_size!($endianness, $alignment, $($val_y:$typ_y),+)
    };
}
#[macro_export]
macro_rules! derive_encode {
    () => ();
    ($endianness:ty, $alignment:ident, $self:expr, $encoder:expr, $field_offset:expr, $val:ident: $typ:ty) => {
        $self.$val.encode($encoder, $field_offset);
    };
    ($endianness:ty, $alignment:ident, $self:expr, $encoder:expr, $field_offset:expr, $val_x:ident:$typ_x:ty, $($val_y:ident:$typ_y:ty),+ $(,)?) => {
        $crate::derive_encode!($endianness, $alignment, $self, $encoder, $field_offset, $val_x:$typ_x);
        $field_offset += $crate::derive_header_size!($endianness, $alignment, $val_x:$typ_x);
        $crate::derive_encode!($endianness, $alignment, $self, $encoder, $field_offset, $($val_y:$typ_y),+)
    };
}
#[macro_export]
macro_rules! derive_decode {
    () => ();
    ($endianness:ty, $alignment:ident, $self:expr, $decoder:expr, $field_offset:expr, $val:ident: $typ:ty) => {
        <$typ as $crate::Encoder<$endianness, $alignment, $typ>>::decode_body($decoder, $field_offset, &mut $self.$val);
    };
    ($endianness:ty, $alignment:ident, $self:expr, $decoder:expr, $field_offset:expr, $val_x:ident:$typ_x:ty, $($val_y:ident:$typ_y:ty),+ $(,)?) => {
        $crate::derive_decode!($endianness, $alignment, $self, $decoder, $field_offset, $val_x:$typ_x);
        $field_offset += $crate::derive_header_size!($endianness, $alignment, $val_x:$typ_x);
        $crate::derive_decode!($endianness, $alignment, $self, $decoder, $field_offset, $($val_y:$typ_y),+);
    };
}
#[macro_export]
macro_rules! derive_types {
    (@typ $endianness:ty, $alignment:ty, $field_offset:expr,) => {};
    (@def $endianness:ty, $alignment:ty, $field_offset:expr,) => {};
    (@typ $endianness:ty, $alignment:ty, $field_offset:expr, $val_head:ident: $typ_head:ty, $($val_next:ident:$typ_next:ty,)* $(,)?) => {
        paste::paste! {
            type [<$val_head:camel>];
        }
        $crate::derive_types!(@typ $endianness, $alignment, $field_offset + <$typ_head as $crate::Encoder<$endianness, $alignment, $typ_head>>::HEADER_SIZE, $($val_next:$typ_next,)*);
    };
    (@def $endianness:ty, $alignment:ty, $field_offset:expr, $val_head:ident: $typ_head:ty, $($val_next:ident:$typ_next:ty,)* $(,)?) => {
        paste::paste! {
            type [<$val_head:camel>] = $crate::FieldEncoder<$endianness, $alignment, $typ_head, { $field_offset }>;
        }
        $crate::derive_types!(@def $endianness, $alignment, $field_offset + <$typ_head as $crate::Encoder<$endianness, $alignment, $typ_head>>::HEADER_SIZE, $($val_next:$typ_next,)*);
    };
}

#[macro_export]
macro_rules! define_codec_struct {
    ($endianness:ty, $alignment:ty, pub struct $struct_type:ident { $($element:ident: $ty:ty),* $(,)? }) => {
        #[derive(Debug, Default, PartialEq, Clone)]
        pub struct $struct_type {
            $(pub $element: $ty),*
        }
        impl<E: ::byteorder::ByteOrder, const A: usize> $crate::Encoder<E, A, $struct_type> for $struct_type {
            const HEADER_SIZE: usize = $crate::derive_header_size!(E, A, $($element:$ty),*);
            fn encode<W: $crate::WritableBuffer<E, A>>(&self, encoder: &mut W, mut field_offset: usize) {
                $crate::derive_encode!(E, A, self, encoder, field_offset, $($element:$ty),*);
            }
            fn decode_header(decoder: &mut $crate::BufferDecoder<E, A>, mut field_offset: usize, result: &mut $struct_type) -> (usize, usize) {
                $crate::derive_decode!(E, A, result, decoder, field_offset, $($element:$ty),*);
                (0, 0)
            }
        }
        impl From<Vec<u8>> for $struct_type {
            fn from(value: Vec<u8>) -> Self {
                let mut result = Self::default();
                let mut buffer_decoder = $crate::BufferDecoder::<$endianness, $alignment>::new(value.as_slice());
                <$struct_type as $crate::Encoder<$endianness, $alignment, $struct_type>>::decode_body(&mut buffer_decoder, 0, &mut result);
                result
            }
        }
        paste::paste! {
            pub trait [<I $struct_type>] {
                $crate::derive_types!(@typ $endianness, $alignment, 0, $($element:$ty,)*);
            }
            impl [<I $struct_type>] for $struct_type {
                $crate::derive_types!(@def $endianness, $alignment, 0, $($element:$ty,)*);
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use byteorder::{BE, LE};

    use crate::encoder::{ALIGNMENT_32, ALIGNMENT_DEFAULT};
    use crate::{
        header_item_size, BufferDecoder, BufferEncoder, Encoder, FieldEncoder, WritableBuffer,
    };

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
        let expected = "\
        0000000000000064000000000000000000000000000000000000000000000000\
        0000001400000000000000000000000000000000000000000000000000000000\
        0003000000000000000000000000000000000000000000000000000000000000\
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

    // #[test]
    // fn test_complicated_type() {
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
    //         <ComplicatedType as Encoder<LittleEndian, ComplicatedType>>::HEADER_SIZE,
    //         <Vec<SimpleTypeU> as Encoder<LittleEndian, Vec<SimpleTypeU>>>::HEADER_SIZE
    //             + <HashMap::<u32, SimpleTypeU> as Encoder<
    //                 LittleEndian,
    //                 HashMap::<u32, SimpleTypeU>,
    //             >>::HEADER_SIZE
    //     );
    //     let encoded_value = {
    //         let mut buffer_encoder = BufferEncoder::<LittleEndian>::new(
    //             <ComplicatedType as Encoder<LittleEndian, ComplicatedType>>::HEADER_SIZE,
    //             None,
    //         );
    //         value0.encode(&mut buffer_encoder, 0);
    //         buffer_encoder.finalize()
    //     };
    //     println!("{}", hex::encode(&encoded_value));
    //     let mut buffer_decoder = BufferDecoder::<LittleEndian>::new(encoded_value.as_slice());
    //     let mut value1 = Default::default();
    //     ComplicatedType::decode_body(&mut buffer_decoder, 0, &mut value1);
    //     assert_eq!(value0, value1);
    // }
}
