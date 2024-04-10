use alloc::vec::Vec;

use byteorder::ByteOrder;
use paste::paste;
use phantom_type::PhantomType;

use crate::encoder::{ALIGNMENT_DEFAULT, HEADER_ITEM_SIZE_DEFAULT};
use crate::header_item_size;

pub trait WritableBuffer<E: ByteOrder, const A: usize> {
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
    ($typ:ty, $endianness:ident, $alignment:expr) => {
        paste! {
            fn [<write_ $typ>](&mut self, field_offset: usize, value: $typ) -> usize {
                if $alignment == ALIGNMENT_DEFAULT {
                    $endianness::[<write_ $typ>](&mut self.buffer[field_offset..], value);
                    core::mem::size_of::<$typ>()
                } else {
                    let header_item_size = header_item_size!(A);
                    let type_size = core::mem::size_of::<$typ>();
                    let field_start_offset = field_offset + A - type_size;
                    $endianness::[<write_ $typ>](&mut self.buffer[field_offset..], value);
                    self.buffer[field_offset+type_size..field_offset+A].fill(0);
                    A
                }
            }
        }
    };
}

pub struct FixedEncoder<E: ByteOrder, const N: usize, const A: usize> {
    header_length: usize,
    body_length: usize,
    buffer: [u8; N],
    _phantom_data: PhantomType<E>,
}

impl<E: ByteOrder, const N: usize, const A: usize> FixedEncoder<E, N, A> {
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

impl<E: ByteOrder, const N: usize, const A: usize> WritableBuffer<E, A> for FixedEncoder<E, N, A> {
    fn write_i8(&mut self, field_offset: usize, value: i8) -> usize {
        if A != ALIGNMENT_DEFAULT {
            let mut v = [0; A];
            v.last_mut().map(|v| *v = value as u8);
            self.write_bytes(field_offset, &v);
        } else {
            self.buffer[field_offset] = value as u8;
        }
        1
    }
    fn write_u8(&mut self, field_offset: usize, value: u8) -> usize {
        if A != ALIGNMENT_DEFAULT {
            let mut v = [0u8; A];
            v.last_mut().map(|v| *v = value);
            self.write_bytes(field_offset, &v);
        } else {
            self.buffer[field_offset] = value;
        }
        1
    }

    impl_byte_writer!(u16, E, A);
    impl_byte_writer!(i16, E, A);
    impl_byte_writer!(u32, E, A);
    impl_byte_writer!(i32, E, A);
    impl_byte_writer!(u64, E, A);
    impl_byte_writer!(i64, E, A);

    fn write_bytes(&mut self, field_offset: usize, bytes: &[u8]) -> usize {
        let data_offset = self.len();
        let data_len = bytes.len();
        let header_item_size = header_item_size!(A);
        let data_len_aligned = if header_item_size == HEADER_ITEM_SIZE_DEFAULT {
            data_len
        } else {
            (data_len + A - 1) / A * A
        };
        <FixedEncoder<E, N, A> as WritableBuffer<E, A>>::write_u32(
            self,
            field_offset,
            data_offset as u32,
        );
        <FixedEncoder<E, N, A> as WritableBuffer<E, A>>::write_u32(
            self,
            field_offset + header_item_size,
            data_len_aligned as u32,
        );
        self.buffer[data_offset..(data_offset + data_len)].copy_from_slice(bytes);
        if header_item_size == HEADER_ITEM_SIZE_DEFAULT {
            self.body_length += data_len;
        } else {
            let data_len_aligned = (data_len + A - 1) / A * A;
            if data_len != data_len_aligned {
                self.buffer[data_offset + data_len..data_offset + data_len_aligned].fill(0)
            }
            self.body_length += data_len_aligned;
        }

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

impl<E: ByteOrder, const A: usize> WritableBuffer<E, A> for BufferEncoder<E, A> {
    fn write_i8(&mut self, field_offset: usize, value: i8) -> usize {
        if A != ALIGNMENT_DEFAULT {
            let mut v = [0; A];
            v.last_mut().map(|v| *v = value as u8);
            self.write_bytes(field_offset, &v);
        } else {
            self.buffer[field_offset] = value as u8;
        }
        1
    }
    fn write_u8(&mut self, field_offset: usize, value: u8) -> usize {
        if A != ALIGNMENT_DEFAULT {
            let mut v = [0; A];
            v.last_mut().map(|v| *v = value);
            self.write_bytes(field_offset, &v);
        } else {
            self.buffer[field_offset] = value;
        }
        1
    }

    impl_byte_writer!(u16, E, A);
    impl_byte_writer!(i16, E, A);
    impl_byte_writer!(u32, E, A);
    impl_byte_writer!(i32, E, A);
    impl_byte_writer!(u64, E, A);
    impl_byte_writer!(i64, E, A);

    fn write_bytes(&mut self, field_offset: usize, bytes: &[u8]) -> usize {
        let data_offset = self.buffer.len();
        let data_len = bytes.len();
        let header_item_size = header_item_size!(A);
        let data_len_aligned = if header_item_size == HEADER_ITEM_SIZE_DEFAULT {
            data_len
        } else {
            (data_len + A - 1) / A * A
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
pub struct BufferDecoder<'a, E: ByteOrder, const A: usize> {
    buffer: &'a [u8],
    _phantom_data: PhantomType<E>,
}

macro_rules! impl_byte_reader {
    ($typ:ty, $endianness:ident, $alignment:expr) => {
        paste! {
            pub fn [<read_ $typ>](&self, field_offset: usize) -> $typ {
                $endianness::[<read_ $typ>](&self.buffer[field_offset..])
            }
        }
    };
}

impl<'a, E: ByteOrder, const A: usize> BufferDecoder<'a, E, A> {
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

    impl_byte_reader!(i16, E, A);
    impl_byte_reader!(u16, E, A);
    impl_byte_reader!(i32, E, A);
    impl_byte_reader!(u32, E, A);
    impl_byte_reader!(i64, E, A);
    impl_byte_reader!(u64, E, A);

    pub fn read_bytes_header(&self, field_offset: usize) -> (usize, usize) {
        let header_item_size = header_item_size!(A);
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

#[cfg(test)]
mod test {
    use byteorder::{ByteOrder, BE, LE};

    use crate::buffer::{BufferDecoder, BufferEncoder, FixedEncoder, WritableBuffer};
    use crate::encoder::ALIGNMENT_DEFAULT;
    use crate::{header_item_size, Encoder};

    const ALIGNMENT_32: usize = 32;

    #[test]
    fn test_fixed_array_alignment_default_le() {
        type Endianness = LE;
        const ALIGNMENT: usize = ALIGNMENT_DEFAULT;
        let header_item_size = header_item_size!(ALIGNMENT);
        let buffer = {
            let mut field_offset = 0;
            let mut buffer = FixedEncoder::<Endianness, 1024, ALIGNMENT>::new(
                header_item_size
                    + header_item_size * 2 // dynamic
                    + header_item_size
                    + header_item_size * 2 // dynamic
                    + header_item_size,
            );
            buffer.write_u32(field_offset, 0xbadcab1e);
            field_offset += header_item_size;
            buffer.write_bytes(field_offset, &[0, 1, 2, 3, 4]);
            field_offset += header_item_size * 2;
            buffer.write_u32(field_offset, 0xdeadbeef);
            field_offset += header_item_size;
            buffer.write_bytes(field_offset, &[5, 6, 7, 8, 9]);
            field_offset += header_item_size * 2;
            buffer.write_u32(field_offset, 0x7f);
            buffer.bytes().to_vec()
        };
        let expected =
            "1eabdcba1c00000005000000efbeadde21000000050000007f00000000010203040506070809";
        let res = hex::encode(&buffer);
        assert_eq!(expected, res);
        let decoder = BufferDecoder::<Endianness, ALIGNMENT>::new(buffer.as_slice());
        assert_eq!(decoder.read_u32(0), 0xbadcab1e);
        assert_eq!(decoder.read_bytes(4).to_vec(), vec![0, 1, 2, 3, 4]);
        assert_eq!(decoder.read_u32(12), 0xdeadbeef);
        assert_eq!(decoder.read_bytes(16).to_vec(), vec![5, 6, 7, 8, 9]);
        assert_eq!(decoder.read_u32(24), 0x7f);
    }

    #[test]
    fn test_fixed_array_alignment_default_be() {
        type Endianness = BE;
        const ALIGNMENT: usize = ALIGNMENT_DEFAULT;
        let header_item_size = header_item_size!(ALIGNMENT);
        let buffer = {
            let mut field_offset = 0;
            let mut buffer = FixedEncoder::<Endianness, 1024, ALIGNMENT>::new(
                header_item_size
                    + header_item_size * 2 // dynamic
                    + header_item_size
                    + header_item_size * 2 // dynamic
                    + header_item_size,
            );
            buffer.write_u32(field_offset, 0xbadcab1e);
            field_offset += header_item_size;
            buffer.write_bytes(field_offset, &[0, 1, 2, 3, 4]);
            field_offset += header_item_size * 2;
            buffer.write_u32(field_offset, 0xdeadbeef);
            field_offset += header_item_size;
            buffer.write_bytes(field_offset, &[5, 6, 7, 8, 9]);
            field_offset += header_item_size * 2;
            buffer.write_u32(field_offset, 0x7f);
            buffer.bytes().to_vec()
        };
        let expected =
            "badcab1e0000001c00000005deadbeef00000021000000050000007f00010203040506070809";
        let res = hex::encode(&buffer);
        assert_eq!(expected, res);
        let decoder = BufferDecoder::<Endianness, ALIGNMENT>::new(buffer.as_slice());
        let mut field_offset = 0;
        assert_eq!(decoder.read_u32(field_offset), 0xbadcab1e);
        field_offset += header_item_size;
        assert_eq!(
            vec![0, 1, 2, 3, 4],
            decoder.read_bytes(field_offset).to_vec(),
        );
        field_offset += header_item_size * 2;
        assert_eq!(0xdeadbeef, decoder.read_u32(field_offset),);
        field_offset += header_item_size;
        assert_eq!(
            vec![5, 6, 7, 8, 9],
            decoder.read_bytes(field_offset).to_vec(),
        );
        field_offset += header_item_size * 2;
        assert_eq!(0x7f, decoder.read_u32(field_offset),);
    }

    #[test]
    fn test_sign_extend_with_endianness() {
        type Endianness = LE;
        type VType = i32;
        const A: usize = 32;
        let v: VType = -12345;
        let type_size = core::mem::size_of::<VType>();
        let mut buf = vec![0; A];
        Endianness::write_i32(&mut buf[A - type_size..], v);
        let expected = "00000000000000000000000000000000000000000000000000000000c7cfffff";
        let fact = hex::encode(&buf);
        assert_eq!(expected, fact);
    }

    #[test]
    fn test_fixed_array_alignment_32_be() {
        type Endianness = BE;
        const ALIGNMENT: usize = ALIGNMENT_32;
        let header_item_size = header_item_size!(ALIGNMENT);
        let buffer = {
            let mut field_offset = 0;
            let mut buffer = FixedEncoder::<Endianness, 1024, ALIGNMENT>::new(
                header_item_size
                    + header_item_size * 2 // dynamic
                    + header_item_size
                    + header_item_size * 2 // dynamic
                    + header_item_size,
            );
            buffer.write_u32(field_offset, 0xbadcab1e);
            field_offset += header_item_size;
            buffer.write_bytes(field_offset, &[0, 1, 2, 3, 4]);
            field_offset += header_item_size * 2;
            buffer.write_u32(field_offset, 0xdeadbeef);
            field_offset += header_item_size;
            buffer.write_bytes(field_offset, &[5, 6, 7, 8, 9]);
            field_offset += header_item_size * 2;
            buffer.write_u32(field_offset, 0x7f);
            buffer.bytes().to_vec()
        };
        let expected = "\
            badcab1e00000000000000000000000000000000000000000000000000000000\
            000000e000000000000000000000000000000000000000000000000000000000\
            0000002000000000000000000000000000000000000000000000000000000000\
            deadbeef00000000000000000000000000000000000000000000000000000000\
            0000010000000000000000000000000000000000000000000000000000000000\
            0000002000000000000000000000000000000000000000000000000000000000\
            0000007f00000000000000000000000000000000000000000000000000000000\
            0001020304000000000000000000000000000000000000000000000000000000\
            0506070809000000000000000000000000000000000000000000000000000000";
        let fact = hex::encode(&buffer);
        assert_eq!(expected, fact);
        let decoder = BufferDecoder::<Endianness, ALIGNMENT>::new(buffer.as_slice());
        let mut field_offset = 0;
        assert_eq!(0xbadcab1e, decoder.read_u32(field_offset));
        field_offset += header_item_size;
        assert_eq!(
            vec![
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ],
            decoder.read_bytes(field_offset).to_vec()
        );
        field_offset += header_item_size * 2;
        assert_eq!(0xdeadbeef, decoder.read_u32(field_offset));
        field_offset += header_item_size;
        assert_eq!(
            vec![
                5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ],
            decoder.read_bytes(field_offset).to_vec()
        );
        field_offset += header_item_size * 2;
        assert_eq!(0x7f, decoder.read_u32(field_offset));
    }

    #[test]
    fn test_bytes_array_alignment_default_le() {
        type Endianness = LE;
        const ALIGNMENT: usize = ALIGNMENT_DEFAULT;
        let header_item_size = header_item_size!(ALIGNMENT);
        let buffer = {
            let mut buffer = BufferEncoder::<Endianness, ALIGNMENT>::new(
                header_item_size
                    + header_item_size * 2
                    + header_item_size
                    + header_item_size * 2
                    + header_item_size,
                None,
            );
            let mut field_offset = 0;
            buffer.write_u32(field_offset, 0xbadcab1e);
            field_offset += header_item_size;
            buffer.write_bytes(field_offset, &[0, 1, 2, 3, 4]);
            field_offset += header_item_size * 2;
            buffer.write_u32(field_offset, 0xdeadbeef);
            field_offset += header_item_size;
            buffer.write_bytes(field_offset, &[5, 6, 7, 8, 9]);
            field_offset += header_item_size * 2;
            buffer.write_u32(field_offset, 0x7f);
            buffer.finalize()
        };
        println!("{}", hex::encode(&buffer));
        let decoder = BufferDecoder::<Endianness, ALIGNMENT>::new(buffer.as_slice());
        assert_eq!(decoder.read_u32(0), 0xbadcab1e);
        assert_eq!(decoder.read_bytes(4).to_vec(), vec![0, 1, 2, 3, 4]);
        assert_eq!(decoder.read_u32(12), 0xdeadbeef);
        assert_eq!(decoder.read_bytes(16).to_vec(), vec![5, 6, 7, 8, 9]);
        assert_eq!(decoder.read_u32(24), 0x7f);
    }

    #[test]
    fn test_bytes_array_alignment_32_be() {
        type Endianness = BE;
        const ALIGNMENT: usize = ALIGNMENT_32;
        let header_item_size = header_item_size!(ALIGNMENT);
        let buffer = {
            let header_length = header_item_size
                + header_item_size * 2
                + header_item_size
                + header_item_size * 2
                + header_item_size;
            let mut buffer = BufferEncoder::<Endianness, ALIGNMENT>::new(header_length, None);
            let mut field_offset = 0;
            let v = 0xbadcab1e;
            let header_size = <u32 as Encoder<Endianness, ALIGNMENT, u32>>::HEADER_SIZE;
            assert_eq!(ALIGNMENT, header_size);
            buffer.write_u32(field_offset, v);
            field_offset += header_item_size;
            buffer.write_bytes(field_offset, &[0, 1, 2, 3, 4]);
            field_offset += header_item_size * 2;
            buffer.write_u32(field_offset, 0xdeadbeef);
            field_offset += header_item_size;
            buffer.write_bytes(field_offset, &[5, 6, 7, 8, 9]);
            field_offset += header_item_size * 2;
            buffer.write_u32(field_offset, 0x7f);
            buffer.finalize()
        };
        let expected = "\
        badcab1e00000000000000000000000000000000000000000000000000000000\
        000000e000000000000000000000000000000000000000000000000000000000\
        0000002000000000000000000000000000000000000000000000000000000000\
        deadbeef00000000000000000000000000000000000000000000000000000000\
        0000010000000000000000000000000000000000000000000000000000000000\
        0000002000000000000000000000000000000000000000000000000000000000\
        0000007f00000000000000000000000000000000000000000000000000000000\
        0001020304000000000000000000000000000000000000000000000000000000\
        0506070809000000000000000000000000000000000000000000000000000000";
        let fact = hex::encode(&buffer);
        assert_eq!(expected, fact);
        let decoder = BufferDecoder::<Endianness, ALIGNMENT>::new(buffer.as_slice());
        let mut field_offset = 0;
        assert_eq!(0xbadcab1e, decoder.read_u32(field_offset));
        field_offset += header_item_size;
        assert_eq!(
            vec![
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ],
            decoder.read_bytes(field_offset).to_vec(),
        );
        field_offset += header_item_size * 2;
        assert_eq!(0xdeadbeef, decoder.read_u32(field_offset));
        field_offset += header_item_size;
        assert_eq!(
            vec![
                5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ],
            decoder.read_bytes(field_offset).to_vec(),
        );
        field_offset += header_item_size * 2;
        assert_eq!(0x7f, decoder.read_u32(field_offset));
    }

    #[test]
    fn test_simple_encoding_alignment_default_le() {
        type Endianness = LE;
        const ALIGNMENT: usize = ALIGNMENT_DEFAULT;
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
            let mut buffer = BufferEncoder::<Endianness, ALIGNMENT>::new(4 + 2 + 8, None);
            let mut offset = 0;
            offset += buffer.write_u32(offset, test.a);
            offset += buffer.write_u16(offset, test.b);
            buffer.write_u64(offset, test.c);
            buffer.finalize()
        };
        println!("{}", hex::encode(&buffer));
        let decoder = BufferDecoder::<Endianness, ALIGNMENT>::new(buffer.as_slice());
        assert_eq!(decoder.read_u32(0), 100);
        assert_eq!(decoder.read_u16(4), 20);
        assert_eq!(decoder.read_u64(6), 3);
    }

    #[test]
    fn test_fixed_encoding_alignment_default_le() {
        type Endianness = LE;
        const ALIGNMENT: usize = ALIGNMENT_DEFAULT;
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
            let mut buffer = FixedEncoder::<Endianness, 1024, ALIGNMENT>::new(4 + 2 + 8);
            let mut offset = 0;
            offset += buffer.write_u32(offset, test.a);
            offset += buffer.write_u16(offset, test.b);
            buffer.write_u64(offset, test.c);
            buffer.bytes().to_vec()
        };
        println!("{}", hex::encode(&buffer));
        let decoder = BufferDecoder::<Endianness, ALIGNMENT>::new(&buffer);
        assert_eq!(decoder.read_u32(0), 100);
        assert_eq!(decoder.read_u16(4), 20);
        assert_eq!(decoder.read_u64(6), 3);
    }
}
