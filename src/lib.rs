#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

extern crate libc;
use std::ffi::c_void;
use std::io;
use std::convert::TryInto;

pub mod raw;

const BLOCK_SIZE: usize = 64000;
const MAX_BLOCK_COMPRESS_SIZE: usize = BLOCK_SIZE + (BLOCK_SIZE / 16) + 64 + 3;

pub fn init() {
    let r = unsafe { raw::lzo_initialize() };
    if !r == 0 {
        panic!("Failed initialize LZO!");
    }
}

pub fn max_compress_len(input_len: usize) -> usize {
    // ref: docs/LZO.FAQ
    input_len + (input_len / 16) + 64 + 3
}

pub fn decompress(input: &[u8], output: &mut [u8]) -> usize {
    init();

    // Determine if there is a header
    let input_buf = if [0xf0, 0xf1].contains(&input[0]) {
        &input[5..]
    } else {
        &input[..]
    };

    let (n_bytes_written, _n_bytes_consumed) = unsafe {
        let mut wrkmem: [u8; 0] = std::mem::MaybeUninit::uninit().assume_init();
        let mut out_len: u32 = 0;
        let (r, n_consumed_bytes) = raw::lzo1x_decompress_safe(
            input_buf.as_ptr(),
            input_buf.len() as u64,
            output.as_mut_ptr(),
            &out_len as *const _ as *mut _,
            wrkmem.as_mut_ptr() as *mut c_void,
        );
        if r != 0 {
            panic!("Failed to decompress, exit code: {}", r);
        }
        (out_len, n_consumed_bytes)
    };
    n_bytes_written as usize
}

pub fn compress(input: &[u8], output: &mut [u8], header: bool) -> usize {
    init();
    unsafe {
        let mut wrkmem: [u8; 64000] = std::mem::MaybeUninit::uninit().assume_init();

        let mut out_len = 0;
        let mut out = if header { &mut output[5..] } else { &mut output[..] };
        let v = raw::lzo1x_1_compress(
            input.as_ptr(),
            input.len() as u64,
            out.as_mut_ptr(),
            &out_len as *const _ as *mut _,
            wrkmem.as_mut_ptr() as *mut c_void,
        );
        if v != 0 {
            panic!("Failed to compress, exit code: {}", v);
        }
        if header {
            output[0] = 0xf0;
            output[1..5].copy_from_slice(&(input.len() as u32).to_be_bytes());
            out_len += 5;
        }
        out_len as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::{compress, decompress, max_compress_len};

    fn gen_data() -> Vec<u8> {
        (0..10000)
            .map(|_| b"Oh what a beautiful day, oh what a beaitufl morning!!!".to_vec())
            .flat_map(|v| v)
            .collect::<Vec<u8>>()
    }

    #[test]
    fn roundtrip() {
        let input = b"bytes".to_vec();

        let mut compressed = vec![0; max_compress_len(input.len())];
        let n_bytes = compress(&input, compressed.as_mut_slice(), true);
        println!("{:?}", &compressed[..n_bytes]);

        let mut decompressed: Vec<u8> = vec![0; input.len()];
        let n_bytes = decompress(&compressed[..n_bytes], decompressed.as_mut_slice());

        assert_eq!(&decompressed[..n_bytes], input.as_slice());
    }
}
