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
    ($endianness:ty, $alignment:ident, $self:ident, $encoder:expr, $field_offset:expr, $val:ident: $typ:ty) => {
        <$typ as Encoder<$endianness, $alignment, $typ >>::encode(&$self.$val, $encoder, $field_offset);
    };
    ($endianness:ty, $alignment:ident, $self:ident, $encoder:expr, $field_offset:expr, $val_x:ident:$typ_x:ty, $($val_y:ident:$typ_y:ty),+ $(,)?) => {
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
    (@typ $endianness:ty, $alignment:ident, $field_offset:expr,) => {};
    (@def $endianness:ty, $alignment:ident, $field_offset:expr,) => {};
    (@typ $endianness:ty, $alignment:ident, $field_offset:expr, $val_head:ident: $typ_head:ty, $($val_next:ident:$typ_next:ty,)* $(,)?) => {
        paste::paste! {
            type [<$val_head:camel>];
        }
        $crate::derive_types!(@typ $endianness, $alignment, $field_offset + <$typ_head as $crate::Encoder<$endianness, $alignment, $typ_head>>::HEADER_SIZE, $($val_next:$typ_next,)*);
    };
    (@def $endianness:ty, $alignment:ident, $field_offset:expr, $val_head:ident: $typ_head:ty, $($val_next:ident:$typ_next:ty,)* $(,)?) => {
        paste::paste! {
            type [<$val_head:camel>] = $crate::FieldEncoder<$endianness, $alignment, $typ_head, { $field_offset }>;
        }
        $crate::derive_types!(@def $endianness, $alignment, $field_offset + <$typ_head as $crate::Encoder<$endianness, $alignment, $typ_head>>::HEADER_SIZE, $($val_next:$typ_next,)*);
    };
}

#[macro_export]
macro_rules! define_codec_struct {
    ($endianness:ty, $alignment:ident, pub struct $struct_type:tt { $($element:ident: $ty:ty),* $(,)? }) => {
        #[derive(Debug, Default, PartialEq, Clone)]
        pub struct $struct_type {
            $(pub $element: $ty),*
        }
        impl<E: ::byteorder::ByteOrder, const A: usize> $crate::Encoder<E, A, $struct_type> for $struct_type {
            const HEADER_SIZE: usize = $crate::derive_header_size!(E, A, $($element:$ty),*);
            fn encode<W: $crate::WritableBuffer<E>>(&self, encoder: &mut W, mut field_offset: usize) {
                $crate::derive_encode!(E, A, self, encoder, field_offset, $($element:$ty),*);
            }
            fn decode_header(decoder: &mut $crate::BufferDecoder<E>, mut field_offset: usize, result: &mut $struct_type) -> (usize, usize) {
                $crate::derive_decode!(E, A, result, decoder, field_offset, $($element:$ty),*);
                (0, 0)
            }
        }
        impl From<Vec<u8>> for $struct_type {
            fn from(value: Vec<u8>) -> Self {
                let mut result = Self::default();
                let mut buffer_decoder = $crate::BufferDecoder::<$endianness>::new(value.as_slice());
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
