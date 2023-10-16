extern crate ethereum_types;
extern crate tiny_keccak;

pub use ethereum_types::H256;
use std::io;
use tiny_keccak::{Hasher, Keccak};

/// Get the KECCAK (i.e. Keccak) hash of the empty bytes string.
pub const KECCAK_EMPTY: H256 = H256([
    0xc5, 0xd2, 0x46, 0x01, 0x86, 0xf7, 0x23, 0x3c, 0x92, 0x7e, 0x7d, 0xb2, 0xdc, 0xc7, 0x03, 0xc0,
    0xe5, 0x00, 0xb6, 0x53, 0xca, 0x82, 0x27, 0x3b, 0x7b, 0xfa, 0xd8, 0x04, 0x5d, 0x85, 0xa4, 0x70,
]);

/// The KECCAK of the RLP encoding of empty data.
pub const KECCAK_NULL_RLP: H256 = H256([
    0x56, 0xe8, 0x1f, 0x17, 0x1b, 0xcc, 0x55, 0xa6, 0xff, 0x83, 0x45, 0xe6, 0x92, 0xc0, 0xf8, 0x6e,
    0x5b, 0x48, 0xe0, 0x1b, 0x99, 0x6c, 0xad, 0xc0, 0x01, 0x62, 0x2f, 0xb5, 0xe3, 0x63, 0xb4, 0x21,
]);

/// The KECCAK of the RLP encoding of empty list.
pub const KECCAK_EMPTY_LIST_RLP: H256 = H256([
    0x1d, 0xcc, 0x4d, 0xe8, 0xde, 0xc7, 0x5d, 0x7a, 0xab, 0x85, 0xb5, 0x67, 0xb6, 0xcc, 0xd4, 0x1a,
    0xd3, 0x12, 0x45, 0x1b, 0x94, 0x8a, 0x74, 0x13, 0xf0, 0xa1, 0x42, 0xfd, 0x40, 0xd4, 0x93, 0x47,
]);

extern "C" {
    /// Hashes input. Returns -1 if either out or input does not exist. Otherwise returns 0.
    pub fn keccak_256(out: *mut u8, outlen: usize, input: *const u8, inputlen: usize) -> i32;
    /// Hashes input. Returns -1 if either out or input does not exist. Otherwise returns 0.
    pub fn keccak_512(out: *mut u8, outlen: usize, input: *const u8, inputlen: usize) -> i32;
}

pub fn keccak<T: AsRef<[u8]>>(s: T) -> H256 {
    let mut result = [0u8; 32];
    write_keccak(s, &mut result);
    H256(result)
}

pub fn write_keccak<T: AsRef<[u8]>>(s: T, dest: &mut [u8]) {
    let input = s.as_ref();
    unsafe {
        // we can safely ignore keccak_256 output, cause we know that both input
        // and dest are properly allocated
        keccak_256(dest.as_mut_ptr(), dest.len(), input.as_ptr(), input.len());
    }
}

pub fn keccak_buffer(r: &mut dyn io::BufRead) -> Result<H256, io::Error> {
    let mut output = [0u8; 32];
    let mut input = [0u8; 1024];
    let mut keccak = Keccak::v256();

    // read file
    loop {
        let some = r.read(&mut input)?;
        if some == 0 {
            break;
        }
        keccak.update(&input[0..some]);
    }

    keccak.finalize(&mut output);
    Ok(output.into())
}
