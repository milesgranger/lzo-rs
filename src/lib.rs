#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_transmute)]
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]

extern crate libc;

use std::ffi::c_void;

pub mod raw;

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
    unsafe {
        let mut wrkmem : [u8; 0] = std::mem::uninitialized();
        let mut out_len = 0;
        let r = raw::lzo1x_decompress(input.as_ptr(), input.len() as u64, output.as_mut_ptr(), &out_len as *const _ as *mut _, wrkmem.as_mut_ptr() as *mut c_void);
        if !r == 0 {
            panic!("Failed to decompress, exit code: {}", r);
        }
        out_len as usize
    }

}

pub fn compress(input: &[u8], output: &mut [u8]) -> usize {
    init();
    unsafe {
        let mut wrkmem : [u8; 64000] = std::mem::uninitialized();

        let mut out_len = 0;
        let v = raw::lzo1x_1_compress(input.as_ptr(), input.len() as u64, output.as_mut_ptr(), &out_len as *const _ as *mut _, wrkmem.as_mut_ptr() as *mut c_void);
        if !v == 0 {
            panic!("Failed to compress, exit code: {}", v);
        }
        out_len as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::{compress, max_compress_len, decompress};

    #[test]
    fn test_roundtrip() {
        let input = b"Oh what a beautiful day, oh what a beaitufl morning!!!".to_vec();

        let mut compressed = vec![0; max_compress_len(input.len())];
        let n_bytes = compress(&input, compressed.as_mut_slice());
        println!("{:?}", String::from_utf8_lossy(&compressed[..n_bytes].to_vec()));
        println!("Output length: {}", n_bytes);

        let mut decompressed: Vec<u8> = vec![0; input.len()];
        let n_bytes = decompress(&compressed[..n_bytes], decompressed.as_mut_slice());
        println!("{:?}", String::from_utf8_lossy(&decompressed[..n_bytes].to_vec()));
        println!("Output length: {}", n_bytes);
    }
}