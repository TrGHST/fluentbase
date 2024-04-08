use alloc::vec::Vec;
use byteorder::ByteOrder;
use paste::paste;
use std::marker::PhantomData;

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

pub struct FixedEncoder<E, const N: usize> {
    header_length: usize,
    body_length: usize,
    buffer: [u8; N],
    _phantom_data: PhantomData<E>,
}

impl<E, const N: usize> FixedEncoder<E, N> {
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
        self.buffer[field_offset] = value as u8;
        1
    }
    fn write_u8(&mut self, field_offset: usize, value: u8) -> usize {
        self.buffer[field_offset] = value;
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
        let data_length = bytes.len();
        // write header with data offset and length
        <FixedEncoder<E, N> as WritableBuffer<E>>::write_u32(
            self,
            field_offset + 0,
            data_offset as u32,
        );
        <FixedEncoder<E, N> as WritableBuffer<E>>::write_u32(
            self,
            field_offset + 4,
            data_length as u32,
        );
        // write bytes to the end of the buffer
        self.buffer[data_offset..(data_offset + data_length)].copy_from_slice(bytes);
        self.body_length += bytes.len();
        8
    }
}

#[derive(Default)]
pub struct BufferEncoder<E> {
    buffer: Vec<u8>,
    _phantom_data: PhantomData<E>,
}

impl<E: ByteOrder> BufferEncoder<E> {
    pub fn new(header_length: usize, data_length: Option<usize>) -> Self {
        let mut buffer = Vec::with_capacity(header_length + data_length.unwrap_or(0));
        buffer.resize(header_length, 0);
        Self {
            buffer,
            _phantom_data: Default::default(),
        }
    }

    pub fn finalize(self) -> Vec<u8> {
        self.buffer
    }
}

impl<E: ByteOrder> WritableBuffer<E> for BufferEncoder<E> {
    fn write_i8(&mut self, field_offset: usize, value: i8) -> usize {
        self.buffer[field_offset] = value as u8;
        1
    }
    fn write_u8(&mut self, field_offset: usize, value: u8) -> usize {
        self.buffer[field_offset] = value;
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
        let data_length = bytes.len();
        // write header with data offset and length
        self.write_u32(field_offset + 0, data_offset as u32);
        self.write_u32(field_offset + 4, data_length as u32);
        // write bytes to the end of the buffer
        self.buffer.extend(bytes);
        8
    }
}

#[derive(Default)]
pub struct BufferDecoder<'a, E: ByteOrder> {
    buffer: &'a [u8],
    _phantom_data: PhantomData<E>,
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
        let bytes_offset = self.read_u32(field_offset + 0) as usize;
        let bytes_length = self.read_u32(field_offset + 4) as usize;
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

#[cfg(test)]
mod test {
    use crate::buffer::{BufferDecoder, BufferEncoder, FixedEncoder, WritableBuffer};
    use byteorder::LittleEndian;

    #[test]
    fn test_simple_encoding() {
        struct Test {
            a: u32,
            b: u16,
            c: u64,
        }
        let test = Test {
            a: 100,
            b: 20,
            c: 3,
        };
        let buffer = {
            let mut buffer = BufferEncoder::<LittleEndian>::new(4 + 2 + 8, None);
            let mut offset = 0;
            offset += buffer.write_u32(offset, test.a);
            offset += buffer.write_u16(offset, test.b);
            buffer.write_u64(offset, test.c);
            buffer.finalize()
        };
        println!("{}", hex::encode(&buffer));
        let decoder = BufferDecoder::<LittleEndian>::new(buffer.as_slice());
        assert_eq!(decoder.read_u32(0), 100);
        assert_eq!(decoder.read_u16(4), 20);
        assert_eq!(decoder.read_u64(6), 3);
    }

    #[test]
    fn test_fixed_encoding() {
        struct Test {
            a: u32,
            b: u16,
            c: u64,
        }
        let test = Test {
            a: 100,
            b: 20,
            c: 3,
        };
        let buffer = {
            let mut buffer = FixedEncoder::<LittleEndian, 1024>::new(4 + 2 + 8);
            let mut offset = 0;
            offset += buffer.write_u32(offset, test.a);
            offset += buffer.write_u16(offset, test.b);
            buffer.write_u64(offset, test.c);
            buffer.bytes().to_vec()
        };
        println!("{}", hex::encode(&buffer));
        let decoder = BufferDecoder::<LittleEndian>::new(&buffer);
        assert_eq!(decoder.read_u32(0), 100);
        assert_eq!(decoder.read_u16(4), 20);
        assert_eq!(decoder.read_u64(6), 3);
    }

    #[test]
    fn test_fixed_array() {
        let buffer = {
            let mut buffer = FixedEncoder::<LittleEndian, 1024>::new(4 + 8 + 4 + 8 + 4);
            buffer.write_u32(0, 0xbadcab1e);
            buffer.write_bytes(4, &[0, 1, 2, 3, 4]);
            buffer.write_u32(12, 0xdeadbeef);
            buffer.write_bytes(16, &[5, 6, 7, 8, 9]);
            buffer.write_u32(24, 0x7f);
            buffer.bytes().to_vec()
        };
        println!("{}", hex::encode(&buffer));
        let decoder = BufferDecoder::<LittleEndian>::new(buffer.as_slice());
        assert_eq!(decoder.read_u32(0), 0xbadcab1e);
        assert_eq!(decoder.read_bytes(4).to_vec(), vec![0, 1, 2, 3, 4]);
        assert_eq!(decoder.read_u32(12), 0xdeadbeef);
        assert_eq!(decoder.read_bytes(16).to_vec(), vec![5, 6, 7, 8, 9]);
        assert_eq!(decoder.read_u32(24), 0x7f);
    }

    #[test]
    fn test_bytes_array() {
        let buffer = {
            let mut buffer = BufferEncoder::<LittleEndian>::new(4 + 8 + 4 + 8 + 4, None);
            buffer.write_u32(0, 0xbadcab1e);
            buffer.write_bytes(4, &[0, 1, 2, 3, 4]);
            buffer.write_u32(12, 0xdeadbeef);
            buffer.write_bytes(16, &[5, 6, 7, 8, 9]);
            buffer.write_u32(24, 0x7f);
            buffer.finalize()
        };
        println!("{}", hex::encode(&buffer));
        let decoder = BufferDecoder::<LittleEndian>::new(buffer.as_slice());
        assert_eq!(decoder.read_u32(0), 0xbadcab1e);
        assert_eq!(decoder.read_bytes(4).to_vec(), vec![0, 1, 2, 3, 4]);
        assert_eq!(decoder.read_u32(12), 0xdeadbeef);
        assert_eq!(decoder.read_bytes(16).to_vec(), vec![5, 6, 7, 8, 9]);
        assert_eq!(decoder.read_u32(24), 0x7f);
    }
}
