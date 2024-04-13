use alloc::vec::Vec;

use byteorder::ByteOrder;
use paste::paste;
use phantom_type::PhantomType;

use crate::encoder::{ALIGNMENT_DEFAULT, HEADER_ITEM_SIZE_DEFAULT};
use crate::{align_number, header_item_size};

pub trait WritableBuffer<E: ByteOrder> {
    fn write_i8(&mut self, field_offset: usize, value: i8) -> usize;
    fn write_u8(&mut self, field_offset: usize, value: u8) -> usize;
    fn write_i16(&mut self, field_offset: usize, value: i16) -> usize;
    fn write_u16(&mut self, field_offset: usize, value: u16) -> usize;
    fn write_i32(&mut self, field_offset: usize, value: i32) -> usize;
    fn write_u32(&mut self, field_offset: usize, value: u32) -> usize;
    fn write_i64(&mut self, field_offset: usize, value: i64) -> usize;
    fn write_u64(&mut self, field_offset: usize, value: u64) -> usize;
    fn write_bytes(&mut self, field_offset: usize, bytes: &[u8]) -> usize;
}

macro_rules! impl_byte_writer {
    ($typ:ty, $endianness:ident) => {
        paste! {
            fn [<write_ $typ>](&mut self, field_offset: usize, value: $typ) -> usize {
                $endianness::[<write_ $typ>](&mut self.buffer[field_offset..], value);
                core::mem::size_of::<$typ>()
            }
        }
    };
}

pub struct FixedEncoder<E: ByteOrder, const N: usize> {
    header_length: usize,
    body_length: usize,
    buffer: [u8; N],
    _phantom_data: PhantomType<E>,
}

impl<E: ByteOrder, const N: usize> FixedEncoder<E, N> {
    pub fn new(header_length: usize) -> Self {
        Self {
            header_length,
            body_length: 0,
            buffer: [0; N],
            _phantom_data: Default::default(),
        }
    }

    #[allow(dead_code)]
    pub fn bytes(&self) -> &[u8] {
        &self.buffer[..(self.header_length + self.body_length)]
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.header_length + self.body_length
    }

    #[allow(dead_code)]
    pub fn finalize(self) -> ([u8; N], usize) {
        (self.buffer, self.len())
    }
}

impl<E: ByteOrder, const N: usize> WritableBuffer<E> for FixedEncoder<E, N> {
    fn write_i8(&mut self, field_offset: usize, value: i8) -> usize {
        // if A != ALIGNMENT_DEFAULT {
        self.buffer[field_offset] = value as u8;
        // ByteOrder::write_i8_into(&[value], &mut self.buffer[field_offset..]);
        // } else {
        //     self.buffer[field_offset] = value as u8;
        // }
        1
    }
    fn write_u8(&mut self, field_offset: usize, value: u8) -> usize {
        // if A != ALIGNMENT_DEFAULT {
        self.buffer[field_offset] = value;
        // } else {
        //     self.buffer[field_offset] = value;
        // }
        1
    }

    impl_byte_writer!(u16, E);
    impl_byte_writer!(i16, E);
    impl_byte_writer!(u32, E);
    impl_byte_writer!(i32, E);
    impl_byte_writer!(u64, E);
    impl_byte_writer!(i64, E);

    fn write_bytes(&mut self, field_offset: usize, bytes: &[u8]) -> usize {
        let data_offset = self.len();
        let data_len = bytes.len();
        let header_item_size = HEADER_ITEM_SIZE_DEFAULT;
        let data_len_aligned = data_len;
        <FixedEncoder<E, N> as WritableBuffer<E>>::write_u32(
            self,
            field_offset,
            data_offset as u32,
        );
        <FixedEncoder<E, N> as WritableBuffer<E>>::write_u32(
            self,
            field_offset + header_item_size,
            data_len_aligned as u32,
        );
        self.buffer[data_offset..(data_offset + data_len)].copy_from_slice(bytes);
        self.body_length += data_len;

        return header_item_size * 2;
    }
}

#[derive(Default)]
pub struct BufferEncoder<E, const A: usize> {
    buffer: Vec<u8>,
    _pt1: PhantomType<E>,
}

impl<E: ByteOrder, const A: usize> BufferEncoder<E, A> {
    pub fn new(header_length: usize, data_length: Option<usize>) -> Self {
        let mut buffer = Vec::with_capacity(header_length + data_length.unwrap_or(0));
        buffer.resize(header_length, 0);
        Self {
            buffer,
            _pt1: Default::default(),
        }
    }

    pub fn finalize(self) -> Vec<u8> {
        self.buffer
    }
}

impl<E: ByteOrder, const A: usize> WritableBuffer<E> for BufferEncoder<E, A> {
    fn write_i8(&mut self, field_offset: usize, value: i8) -> usize {
        // TODO use ByteOrder for writing?
        // if A != ALIGNMENT_DEFAULT {
        self.buffer[field_offset] = value as u8;
        // ByteOrder::write_i8_into(&[value], &mut self.buffer[field_offset..]);
        // } else {
        // self.buffer[field_offset] = value as u8;
        // }
        1
    }
    fn write_u8(&mut self, field_offset: usize, value: u8) -> usize {
        // if A != ALIGNMENT_DEFAULT {
        self.buffer[field_offset] = value;
        // } else {
        //     self.buffer[field_offset] = value;
        // }
        1
    }

    impl_byte_writer!(u16, E);
    impl_byte_writer!(i16, E);
    impl_byte_writer!(u32, E);
    impl_byte_writer!(i32, E);
    impl_byte_writer!(u64, E);
    impl_byte_writer!(i64, E);

    fn write_bytes(&mut self, field_offset: usize, bytes: &[u8]) -> usize {
        let data_offset = self.buffer.len();
        let data_len = bytes.len();
        let header_item_size = header_item_size!(A);
        let data_len_aligned = if header_item_size == HEADER_ITEM_SIZE_DEFAULT {
            data_len
        } else {
            align_number!(data_len, A)
        };

        self.write_u32(field_offset, data_offset as u32);
        self.write_u32(field_offset + header_item_size, data_len_aligned as u32);

        self.buffer.extend(bytes);
        if data_len_aligned > data_len {
            for _ in 0..data_len_aligned - data_len {
                self.buffer.push(0);
            }
        }
        header_item_size * 2
    }
}

#[derive(Default)]
pub struct BufferDecoder<'a, E: ByteOrder> {
    buffer: &'a [u8],
    _phantom_data: PhantomType<E>,
}

macro_rules! impl_byte_reader {
    ($typ:ty, $endianness:ident) => {
        paste! {
            pub fn [<read_ $typ>](&self, field_offset: usize) -> $typ {
                $endianness::[<read_ $typ>](&self.buffer[field_offset..])
            }
        }
    };
}

impl<'a, E: ByteOrder> BufferDecoder<'a, E> {
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            buffer: input,
            _phantom_data: Default::default(),
        }
    }

    pub fn read_i8(&mut self, field_offset: usize) -> i8 {
        self.buffer[field_offset] as i8
    }
    pub fn read_u8(&mut self, field_offset: usize) -> u8 {
        self.buffer[field_offset]
    }

    impl_byte_reader!(i16, E);
    impl_byte_reader!(u16, E);
    impl_byte_reader!(i32, E);
    impl_byte_reader!(u32, E);
    impl_byte_reader!(i64, E);
    impl_byte_reader!(u64, E);

    pub fn read_bytes_header(&self, field_offset: usize) -> (usize, usize) {
        let header_item_size = HEADER_ITEM_SIZE_DEFAULT;
        let bytes_offset = self.read_u32(field_offset) as usize;
        let bytes_length = self.read_u32(field_offset + header_item_size) as usize;
        (bytes_offset, bytes_length)
    }

    pub fn read_bytes(&self, field_offset: usize) -> &[u8] {
        let (bytes_offset, bytes_length) = self.read_bytes_header(field_offset);
        &self.buffer[bytes_offset..(bytes_offset + bytes_length)]
    }

    pub fn read_bytes2(&self, field1_offset: usize, field2_offset: usize) -> (&[u8], &[u8]) {
        (
            self.read_bytes(field1_offset),
            self.read_bytes(field2_offset),
        )
    }
}
