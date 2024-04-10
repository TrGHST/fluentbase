use byteorder::ByteOrder;

#[macro_export]
macro_rules! derive_header_size {
    () => (0);
    ($endianness:ident, $alignment:ident, $val:ident: $typ:ty) => {
        <$typ as $crate::Encoder<E, {$alignment}, $typ>>::HEADER_SIZE
    };
    ($endianness:ident, $alignment:ident, $val_x:ident:$typ_x:ty, $($val_y:ident:$typ_y:ty),+ $(,)?) => {
        $crate::derive_header_size!($endianness, $alignment, $val_x:$typ_x) + $crate::derive_header_size!($endianness, $alignment, $($val_y:$typ_y),+)
    };
}
#[macro_export]
macro_rules! derive_encode {
    () => ();
    ($endianness:ident, $alignment:ident, $self:expr, $encoder:expr, $field_offset:expr, $val:ident: $typ:ty) => {
        $self.$val.encode($encoder, $field_offset);
    };
    ($endianness:ident, $alignment:ident, $self:expr, $encoder:expr, $field_offset:expr, $val_x:ident:$typ_x:ty, $($val_y:ident:$typ_y:ty),+ $(,)?) => {
        $crate::derive_encode!($endianness, $alignment, $self, $encoder, $field_offset, $val_x:$typ_x);
        $field_offset += $crate::derive_header_size!($endianness, $alignment, $val_x:$typ_x);
        $crate::derive_encode!($endianness, $alignment, $self, $encoder, $field_offset, $($val_y:$typ_y),+)
    };
}
#[macro_export]
macro_rules! derive_decode {
    () => ();
    ($endianness:ident, $alignment:ident, $self:expr, $decoder:expr, $field_offset:expr, $val:ident: $typ:ty) => {
        <$typ as $crate::Encoder<$endianness, {$alignment}, $typ>>::decode_body($decoder, $field_offset, &mut $self.$val);
    };
    ($endianness:ident, $alignment:ident, $self:expr, $decoder:expr, $field_offset:expr, $val_x:ident:$typ_x:ty, $($val_y:ident:$typ_y:ty),+ $(,)?) => {
        $crate::derive_decode!($endianness, $alignment, $self, $decoder, $field_offset, $val_x:$typ_x);
        $field_offset += $crate::derive_header_size!($endianness, $alignment, $val_x:$typ_x);
        $crate::derive_decode!($endianness, $alignment, $self, $decoder, $field_offset, $($val_y:$typ_y),+);
    };
}
#[macro_export]
macro_rules! derive_types {
    (@typ $field_offset:expr,) => {};
    (@def $field_offset:expr,) => {};
    (@typ $field_offset:expr, $val_head:ident: $typ_head:ty, $($val_next:ident:$typ_next:ty,)* $(,)?) => {
        paste::paste! {
            type [<$val_head:camel>];
        }
        $crate::derive_types!(@typ $field_offset + <$typ_head as $crate::Encoder<::byteorder::LE, {$crate::encoder::ALIGNMENT_32}, $typ_head>>::HEADER_SIZE, $($val_next:$typ_next,)*);
    };
    (@def $field_offset:expr, $val_head:ident: $typ_head:ty, $($val_next:ident:$typ_next:ty,)* $(,)?) => {
        paste::paste! {
            type [<$val_head:camel>] = $crate::FieldEncoder<::byteorder::LE, {$crate::encoder::ALIGNMENT_32}, $typ_head, { $field_offset }>;
        }
        $crate::derive_types!(@def $field_offset + <$typ_head as $crate::Encoder<::byteorder::LE, {$crate::encoder::ALIGNMENT_32}, $typ_head>>::HEADER_SIZE, $($val_next:$typ_next,)*);
    };
}

#[macro_export]
macro_rules! define_codec_struct {
    (pub struct $struct_type:ident { $($element:ident: $ty:ty),* $(,)? }) => {
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
                let mut buffer_decoder = $crate::BufferDecoder::<::byteorder::LE, {$crate::encoder::ALIGNMENT_32}>::new(value.as_slice());
                <$struct_type as $crate::Encoder<::byteorder::LE, {$crate::encoder::ALIGNMENT_32}, $struct_type>>::decode_body(&mut buffer_decoder, 0, &mut result);
                result
            }
        }
        paste::paste! {
            pub trait [<I $struct_type>] {
                $crate::derive_types!(@typ 0, $($element:$ty,)*);
            }
            impl [<I $struct_type>] for $struct_type {
                $crate::derive_types!(@def 0, $($element:$ty,)*);
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use byteorder::{BE, LE};

    use crate::encoder::{ALIGNMENT_32, ALIGNMENT_DEFAULT};
    use crate::{header_item_size, BufferDecoder, BufferEncoder, Encoder};

    define_codec_struct! {
        pub struct SimpleTypeU {
            a: u64,
            b: u32,
            c: u16,
        }
    }

    #[test]
    fn test_simple_type_alignment_default_u_le() {
        type Endianness = LE;
        const ALIGNMENT: usize = ALIGNMENT_DEFAULT;
        let header_item_size = header_item_size!(ALIGNMENT);

        let value0 = SimpleTypeU {
            a: 100,
            b: 20,
            c: 3,
        };
        assert_eq!(
            <SimpleTypeU as Encoder<Endianness, ALIGNMENT, SimpleTypeU>>::HEADER_SIZE,
            8 + 4 + 2
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

        let value0 = SimpleTypeU {
            a: 100,
            b: 20,
            c: 3,
        };
        assert_eq!(
            <SimpleTypeU as Encoder<Endianness, ALIGNMENT, SimpleTypeU>>::HEADER_SIZE,
            32 + 32 + 32
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

    // define_codec_struct! {
    //     pub struct SimpleTypeS {
    //         a: i64,
    //         b: i32,
    //         c: i16,
    //     }
    // }

    // #[test]
    // fn test_simple_type_s() {
    //     let value0 = SimpleTypeS {
    //         a: -100,
    //         b: -20,
    //         c: -3,
    //     };
    //     assert_eq!(
    //         <SimpleTypeS as Encoder<BigEndian, SimpleTypeS>>::HEADER_SIZE,
    //         8 + 4 + 2
    //     );
    //     let encoded_value = {
    //         let mut buffer_encoder = BufferEncoder::<LittleEndian>::new(
    //             <SimpleTypeS as Encoder<LittleEndian, SimpleTypeS>>::HEADER_SIZE,
    //             None,
    //         );
    //         value0.encode(&mut buffer_encoder, 0);
    //         buffer_encoder.finalize()
    //     };
    //     println!("{}", hex::encode(&encoded_value));
    //     let mut buffer_decoder = BufferDecoder::<LittleEndian>::new(encoded_value.as_slice());
    //     let mut value1 = Default::default();
    //     SimpleTypeS::decode_body(&mut buffer_decoder, 0, &mut value1);
    //     assert_eq!(value0, value1);
    // }
    //
    // define_codec_struct! {
    //     pub struct ComplicatedType {
    //         values: Vec<SimpleTypeU>,
    //         maps: HashMap<u32, ComplicatedType>,
    //     }
    // }
    //
    // #[test]
    // fn test_decode_specific_field() {
    //     let value = SimpleTypeU {
    //         a: 100,
    //         b: 20,
    //         c: 3,
    //     };
    //     // check offsets
    //     assert_eq!(<SimpleTypeU as ISimpleTypeU>::A::FIELD_OFFSET, 0);
    //     assert_eq!(<SimpleTypeU as ISimpleTypeU>::B::FIELD_OFFSET, 8);
    //     assert_eq!(<SimpleTypeU as ISimpleTypeU>::C::FIELD_OFFSET, 8 + 4);
    //     // check sizes
    //     assert_eq!(<SimpleTypeU as ISimpleTypeU>::A::FIELD_SIZE, 8);
    //     assert_eq!(<SimpleTypeU as ISimpleTypeU>::B::FIELD_SIZE, 4);
    //     assert_eq!(<SimpleTypeU as ISimpleTypeU>::C::FIELD_SIZE, 2);
    //     // encode entire struct
    //     let encoded_value =
    //         <SimpleTypeU as Encoder<LittleEndian, SimpleTypeU>>::encode_to_vec(&value, 0);
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
