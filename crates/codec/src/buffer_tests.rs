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
        let mut buffer = FixedEncoder::<Endianness, 1024>::new(
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
    let expected = "1eabdcba1c00000005000000efbeadde21000000050000007f00000000010203040506070809";
    let res = hex::encode(&buffer);
    assert_eq!(expected, res);
    let decoder = BufferDecoder::<Endianness>::new(buffer.as_slice());
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
        let mut buffer = FixedEncoder::<Endianness, 1024>::new(
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
    let expected = "badcab1e0000001c00000005deadbeef00000021000000050000007f00010203040506070809";
    let res = hex::encode(&buffer);
    assert_eq!(expected, res);
    let decoder = BufferDecoder::<Endianness>::new(buffer.as_slice());
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
        let mut buffer = FixedEncoder::<Endianness, 1024>::new(
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
    let decoder = BufferDecoder::<Endianness>::new(buffer.as_slice());
    let mut field_offset = 0;
    assert_eq!(0xbadcab1e, decoder.read_u32(field_offset));
    field_offset += header_item_size;
    assert_eq!(
        vec![
            0, 1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0
        ],
        decoder.read_bytes(field_offset).to_vec()
    );
    field_offset += header_item_size * 2;
    assert_eq!(0xdeadbeef, decoder.read_u32(field_offset));
    field_offset += header_item_size;
    assert_eq!(
        vec![
            5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0
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
    let decoder = BufferDecoder::<Endianness>::new(buffer.as_slice());
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
    let decoder = BufferDecoder::<Endianness>::new(buffer.as_slice());
    let mut field_offset = 0;
    assert_eq!(0xbadcab1e, decoder.read_u32(field_offset));
    field_offset += header_item_size;
    assert_eq!(
        vec![
            0, 1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0
        ],
        decoder.read_bytes(field_offset).to_vec(),
    );
    field_offset += header_item_size * 2;
    assert_eq!(0xdeadbeef, decoder.read_u32(field_offset));
    field_offset += header_item_size;
    assert_eq!(
        vec![
            5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0
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
    let decoder = BufferDecoder::<Endianness>::new(buffer.as_slice());
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
        let mut buffer = FixedEncoder::<Endianness, 1024>::new(4 + 2 + 8);
        let mut offset = 0;
        offset += buffer.write_u32(offset, test.a);
        offset += buffer.write_u16(offset, test.b);
        buffer.write_u64(offset, test.c);
        buffer.bytes().to_vec()
    };
    println!("{}", hex::encode(&buffer));
    let decoder = BufferDecoder::<Endianness>::new(&buffer);
    assert_eq!(decoder.read_u32(0), 100);
    assert_eq!(decoder.read_u16(4), 20);
    assert_eq!(decoder.read_u64(6), 3);
}
