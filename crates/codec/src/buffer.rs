use alloc::vec::Vec;

use byteorder::ByteOrder;
use paste::paste;
use phantom_type::PhantomType;

pub trait WritableBuffer<E: ByteOrder> {
    fn len(&self) -> usize;
    fn write_i8(&mut self, field_offset: usize, value: i8) -> usize;
    fn write_u8(&mut self, field_offset: usize, value: u8) -> usize;
    fn write_i16(&mut self, field_offset: usize, value: i16) -> usize;
    fn write_u16(&mut self, field_offset: usize, value: u16) -> usize;
    fn write_i32(&mut self, field_offset: usize, value: i32) -> usize;
    fn write_u32(&mut self, field_offset: usize, value: u32) -> usize;
    fn write_i64(&mut self, field_offset: usize, value: i64) -> usize;
    fn write_u64(&mut self, field_offset: usize, value: u64) -> usize;
    fn write_bytes(&mut self, field_offset: usize, bytes: &[u8]) -> usize;
    fn fill_bytes(&mut self, offset: usize, count: usize, value: u8) -> usize;
}

macro_rules! impl_byte_writer {
    ($typ:ty, $endianness:ident) => {
        paste! {
            fn [<write_ $typ>](&mut self, offset: usize, value: $typ) -> usize {
                $endianness::[<write_ $typ>](&mut self.buffer[offset..], value);
                $crate::size_of!($typ)
            }
        }
    };
}

#[derive(Default)]
pub struct DynamicBuffer<E> {
    buffer: Vec<u8>,
    _pt1: PhantomType<E>,
}

impl<E: ByteOrder> DynamicBuffer<E> {
    pub fn new(header_length: usize, data_length: Option<usize>) -> Self {
        let mut buffer = Vec::with_capacity(header_length + data_length.unwrap_or(0));
        buffer.resize(header_length, 0);
        Self {
            buffer,
            ..Default::default()
        }
    }

    pub fn finalize(self) -> Vec<u8> {
        self.buffer
    }
}

impl<E: ByteOrder> WritableBuffer<E> for DynamicBuffer<E> {
    fn len(&self) -> usize {
        return self.buffer.len();
    }
    fn write_i8(&mut self, offset: usize, value: i8) -> usize {
        self.buffer[offset] = value as u8;
        1
    }
    fn write_u8(&mut self, offset: usize, value: u8) -> usize {
        self.buffer[offset] = value;
        1
    }

    impl_byte_writer!(u16, E);
    impl_byte_writer!(i16, E);
    impl_byte_writer!(u32, E);
    impl_byte_writer!(i32, E);
    impl_byte_writer!(u64, E);
    impl_byte_writer!(i64, E);

    fn write_bytes(&mut self, offset: usize, data: &[u8]) -> usize {
        let data_len = data.len();

        if self.buffer.len() < offset + data_len {
            self.buffer.resize(offset + data_len, 0);
        }
        self.buffer[offset..offset + data_len].copy_from_slice(data);
        data_len
    }

    fn fill_bytes(&mut self, offset: usize, count: usize, value: u8) -> usize {
        if count == 0 {
            return count;
        }
        if self.buffer.len() < offset + count {
            self.buffer.resize(offset + count, 0);
        }
        self.buffer[offset..offset + count].fill(value);
        count
    }
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

#[derive(Default)]
pub struct ReadableBuffer<'a, E> {
    buffer: &'a [u8],
    _pt1: PhantomType<E>,
}
impl<'a, E: ByteOrder> ReadableBuffer<'a, E> {
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            buffer: input,
            ..Default::default()
        }
    }

    pub fn read_i8(&self, field_offset: usize) -> i8 {
        self.buffer[field_offset] as i8
    }
    pub fn read_u8(&self, field_offset: usize) -> u8 {
        self.buffer[field_offset]
    }

    impl_byte_reader!(i16, E);
    impl_byte_reader!(u16, E);
    impl_byte_reader!(i32, E);
    impl_byte_reader!(u32, E);
    impl_byte_reader!(i64, E);
    impl_byte_reader!(u64, E);

    pub fn read_bytes(&self, offset: usize, len: usize) -> &[u8] {
        &self.buffer[offset..offset + len]
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn len_offset(&self, offset: usize) -> usize {
        self.buffer[offset..].len()
    }
}
