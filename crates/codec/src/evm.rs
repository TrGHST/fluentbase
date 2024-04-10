use alloy_primitives::{Address, Bytes, FixedBytes, Uint};
use byteorder::{ByteOrder, BE};

use crate::encoder::ALIGNMENT_DEFAULT;
use crate::{buffer::WritableBuffer, BufferDecoder, Encoder};

impl<E: ByteOrder, const A: usize> Encoder<E, A, Bytes> for Bytes {
    const HEADER_SIZE: usize = core::mem::size_of::<u32>() * 2;

    fn encode<W: WritableBuffer<E, A>>(&self, encoder: &mut W, field_offset: usize) {
        encoder.write_bytes(field_offset, &self.0);
    }

    fn decode_header(
        decoder: &mut BufferDecoder<E, A>,
        field_offset: usize,
        _result: &mut Bytes,
    ) -> (usize, usize) {
        decoder.read_bytes_header(field_offset)
    }

    fn decode_body(decoder: &mut BufferDecoder<E, A>, field_offset: usize, result: &mut Bytes) {
        let bytes = decoder.read_bytes(field_offset);
        *result = Bytes::copy_from_slice(bytes);
    }
}

impl<E: ByteOrder, const N: usize, const A: usize> Encoder<E, A, FixedBytes<N>> for FixedBytes<N> {
    const HEADER_SIZE: usize = N; // TODO correct?
    fn encode<W: WritableBuffer<E, A>>(&self, encoder: &mut W, field_offset: usize) {
        self.0.encode(encoder, field_offset)
    }
    fn decode_header(
        decoder: &mut BufferDecoder<E, A>,
        field_offset: usize,
        result: &mut FixedBytes<N>,
    ) -> (usize, usize) {
        <[u8; N]>::decode_body(decoder, field_offset, &mut result.0);
        (0, 0)
    }
}

macro_rules! impl_evm_fixed {
    ($typ:ty) => {
        impl<E: ByteOrder, const A: usize> Encoder<E, A, $typ> for $typ {
            const HEADER_SIZE: usize = if A == ALIGNMENT_DEFAULT {
                <$typ>::len_bytes()
            } else {
                A // what if: A != ALIGNMENT_DEFAULT && A < <$typ>::len_bytes()
            };
            fn encode<W: WritableBuffer<E, A>>(&self, encoder: &mut W, field_offset: usize) {
                self.0.encode(encoder, field_offset)
            }
            fn decode_header(
                decoder: &mut BufferDecoder<E, A>,
                field_offset: usize,
                result: &mut $typ,
            ) -> (usize, usize) {
                <FixedBytes<{ <Self as Encoder<BE, { ALIGNMENT_DEFAULT }, Self>>::HEADER_SIZE }> as Encoder<
                    E,
                    A,
                    FixedBytes<{ <Self as Encoder<BE, { ALIGNMENT_DEFAULT }, Self>>::HEADER_SIZE }>,
                >>::decode_header(decoder, field_offset, &mut result.0);
                (0, 0)
            }
        }
    };
}

impl_evm_fixed!(Address);

impl<E: ByteOrder, const A: usize, const BITS: usize, const LIMBS: usize>
    Encoder<E, A, Uint<BITS, LIMBS>> for Uint<BITS, LIMBS>
{
    const HEADER_SIZE: usize = Self::BYTES;
    fn encode<W: WritableBuffer<E, A>>(&self, encoder: &mut W, field_offset: usize) {
        self.as_limbs().encode(encoder, field_offset)
    }
    fn decode_header(
        decoder: &mut BufferDecoder<E, A>,
        field_offset: usize,
        result: &mut Uint<BITS, LIMBS>,
    ) -> (usize, usize) {
        unsafe {
            <[u64; LIMBS]>::decode_header(decoder, field_offset, result.as_limbs_mut());
        }
        (0, 0)
    }
}
