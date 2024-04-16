use alloc::vec::Vec;

use byteorder::ByteOrder;
use paste::paste;
use phantom_type::PhantomType;

pub trait WritableBuffer<E: ByteOrder> {
    fn len(&self) -> usize;
    fn write_bool(&mut self, field_offset: usize, value: bool) -> usize;
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
pub struct WritableBufferImpl<E> {
    buffer: Vec<u8>,
    _pt1: PhantomType<E>,
}

impl<E: ByteOrder> WritableBufferImpl<E> {
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

impl<E: ByteOrder> WritableBuffer<E> for WritableBufferImpl<E> {
    fn len(&self) -> usize {
        return self.buffer.len();
    }
    fn write_bool(&mut self, offset: usize, value: bool) -> usize {
        self.buffer[offset] = value as u8;
        1
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
            fn [<read_ $typ>](&self, field_offset: usize) -> $typ {
                $endianness::[<read_ $typ>](&self.buffer[field_offset..])
            }
        }
    };
}

pub trait ReadableBuffer<E: ByteOrder> {
    fn read_bool(&self, field_offset: usize) -> bool;
    fn read_i8(&self, field_offset: usize) -> i8;
    fn read_u8(&self, field_offset: usize) -> u8;
    fn read_i16(&self, field_offset: usize) -> i16;
    fn read_u16(&self, field_offset: usize) -> u16;
    fn read_i32(&self, field_offset: usize) -> i32;
    fn read_u32(&self, field_offset: usize) -> u32;
    fn read_i64(&self, field_offset: usize) -> i64;
    fn read_u64(&self, field_offset: usize) -> u64;

    fn read_bytes(&self, offset: usize, len: usize) -> &[u8];

    fn len(&self) -> usize;

    fn len_offset(&self, offset: usize) -> usize;
}

#[derive(Default)]
pub struct ReadableBufferImpl<'a, E> {
    buffer: &'a [u8],
    _pt1: PhantomType<E>,
}

impl<'a, E: ByteOrder> ReadableBufferImpl<'a, E> {
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            buffer: input,
            ..Default::default()
        }
    }
}
impl<'a, E: ByteOrder> ReadableBuffer<E> for ReadableBufferImpl<'a, E> {
    fn read_bool(&self, field_offset: usize) -> bool {
        self.buffer[field_offset] > 0
    }
    fn read_i8(&self, field_offset: usize) -> i8 {
        self.buffer[field_offset] as i8
    }
    fn read_u8(&self, field_offset: usize) -> u8 {
        self.buffer[field_offset]
    }

    impl_byte_reader!(i16, E);
    impl_byte_reader!(u16, E);
    impl_byte_reader!(i32, E);
    impl_byte_reader!(u32, E);
    impl_byte_reader!(i64, E);
    impl_byte_reader!(u64, E);

    fn read_bytes(&self, offset: usize, len: usize) -> &[u8] {
        &self.buffer[offset..offset + len]
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }

    fn len_offset(&self, offset: usize) -> usize {
        self.buffer[offset..].len()
    }
}
