#![allow(unused_variables)]

use ::libc;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
}
pub type lzo_uint = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzo_callback_t {
    pub nalloc: lzo_alloc_func_t,
    pub nfree: lzo_free_func_t,
    pub nprogress: lzo_progress_func_t,
    pub user1: *mut libc::c_void,
    pub user2: lzo_uint,
    pub user3: lzo_uint,
}
pub type lzo_progress_func_t
    =
    Option<unsafe extern "C" fn(_: *mut lzo_callback_t, _: lzo_uint,
                                _: lzo_uint, _: libc::c_int) -> ()>;
pub type lzo_free_func_t
    =
    Option<unsafe extern "C" fn(_: *mut lzo_callback_t, _: *mut libc::c_void)
               -> ()>;
pub type lzo_alloc_func_t
    =
    Option<unsafe extern "C" fn(_: *mut lzo_callback_t, _: lzo_uint,
                                _: lzo_uint) -> *mut libc::c_void>;
pub type lzo_memops_TU8 = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union lzo_config_check_union {
    pub a: [lzo_uint; 2],
    pub b: [libc::c_uchar; 16],
    pub c: [libc::c_ulong; 2],
}
pub type lzo_memops_TU4 = libc::c_uint;
pub type lzo_memops_TU2 = libc::c_ushort;
#[inline(always)]
unsafe extern "C" fn lzo_bitops_ctlz32_func(mut v: libc::c_uint)
 -> libc::c_uint {
    let mut r: libc::c_uint = 0;
    r = v.leading_zeros() as i32 as libc::c_uint;
    return r;
}
#[inline(always)]
unsafe extern "C" fn lzo_bitops_ctlz64_func(mut v: libc::c_ulong)
 -> libc::c_uint {
    let mut r: libc::c_uint = 0;
    r = v.leading_zeros() as i32 as libc::c_uint;
    return r;
}
#[inline(always)]
unsafe extern "C" fn lzo_bitops_cttz32_func(mut v: libc::c_uint)
 -> libc::c_uint {
    let mut r: libc::c_uint = 0;
    r = v.trailing_zeros() as i32 as libc::c_uint;
    return r;
}
#[inline(always)]
unsafe extern "C" fn lzo_bitops_cttz64_func(mut v: libc::c_ulong)
 -> libc::c_uint {
    let mut r: libc::c_uint = 0;
    r = v.trailing_zeros() as i32 as libc::c_uint;
    return r;
}
unsafe extern "C" fn lzo_bitops_unused_funcs() { }
#[no_mangle]
pub unsafe extern "C" fn __lzo_ptr_linear(mut ptr: *const libc::c_void)
 -> libc::c_ulong {
    let mut p: libc::c_ulong = 0;
    p = ptr as libc::c_ulong;
    return p;
}
/* align a char pointer on a boundary that is a multiple of 'size' */
#[no_mangle]
pub unsafe extern "C" fn __lzo_align_gap(mut ptr: *const libc::c_void,
                                         mut size: lzo_uint) -> libc::c_uint {
    let mut p: libc::c_ulong = 0;
    let mut n: libc::c_ulong = 0;
    if size < 2 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_uint
    }
    p = __lzo_ptr_linear(ptr);
    if size & size.wrapping_sub(1 as libc::c_int as libc::c_ulong) !=
           0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_uint
    }
    n = size;
    n =
        (p.wrapping_add(n).wrapping_sub(1 as libc::c_int as libc::c_ulong) &
             !n.wrapping_sub(1 as libc::c_int as
                                 libc::c_ulong)).wrapping_sub(p);
    return n as libc::c_uint;
}
/* If you use the LZO library in a product, I would appreciate that you
 * keep this copyright string in the executable of your product.
 */
static mut lzo_copyright_: [libc::c_char; 5] =
    unsafe {
        *::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"2.10\x00")
    };
static mut lzo_version_string_: [libc::c_char; 5] =
    unsafe {
        *::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"2.10\x00")
    };
static mut lzo_version_date_: [libc::c_char; 12] =
    unsafe {
        *::std::mem::transmute::<&[u8; 12],
                                 &[libc::c_char; 12]>(b"Mar 01 2017\x00")
    };
#[no_mangle]
pub unsafe extern "C" fn lzo_copyright() -> *const libc::c_uchar {
    return lzo_copyright_.as_ptr() as *const libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn lzo_version() -> libc::c_uint {
    return 0x20a0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lzo_version_string() -> *const libc::c_char {
    return lzo_version_string_.as_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn lzo_version_date() -> *const libc::c_char {
    return lzo_version_date_.as_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn _lzo_version_string() -> *const libc::c_char {
    return lzo_version_string_.as_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn _lzo_version_date() -> *const libc::c_char {
    return lzo_version_date_.as_ptr();
}
/* checksum functions */
#[no_mangle]
pub unsafe extern "C" fn lzo_adler32(mut adler: libc::c_uint,
                                     mut buf: *const libc::c_uchar,
                                     mut len: lzo_uint) -> libc::c_uint {
    let mut s1: libc::c_uint = adler & 0xffff as libc::c_int as libc::c_uint;
    let mut s2: libc::c_uint =
        adler >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint;
    let mut k: libc::c_uint = 0;
    if buf.is_null() { return 1 as libc::c_int as libc::c_uint }
    while len > 0 as libc::c_int as libc::c_ulong {
        k =
            if len < 5552 as libc::c_int as libc::c_ulong {
                len as libc::c_uint
            } else { 5552 as libc::c_int as libc::c_uint };
        len =
            (len as libc::c_ulong).wrapping_sub(k as libc::c_ulong) as
                lzo_uint as lzo_uint;
        if k >= 16 as libc::c_int as libc::c_uint {
            loop  {
                s1 =
                    s1.wrapping_add(*buf.offset(0 as libc::c_int as isize) as
                                        libc::c_uint);
                s2 = s2.wrapping_add(s1);
                s1 =
                    s1.wrapping_add(*buf.offset((0 as libc::c_int +
                                                     1 as libc::c_int) as
                                                    isize) as libc::c_uint);
                s2 = s2.wrapping_add(s1);
                s1 =
                    s1.wrapping_add(*buf.offset((0 as libc::c_int +
                                                     2 as libc::c_int) as
                                                    isize) as libc::c_uint);
                s2 = s2.wrapping_add(s1);
                s1 =
                    s1.wrapping_add(*buf.offset((0 as libc::c_int +
                                                     2 as libc::c_int +
                                                     1 as libc::c_int) as
                                                    isize) as libc::c_uint);
                s2 = s2.wrapping_add(s1);
                s1 =
                    s1.wrapping_add(*buf.offset((0 as libc::c_int +
                                                     4 as libc::c_int) as
                                                    isize) as libc::c_uint);
                s2 = s2.wrapping_add(s1);
                s1 =
                    s1.wrapping_add(*buf.offset((0 as libc::c_int +
                                                     4 as libc::c_int +
                                                     1 as libc::c_int) as
                                                    isize) as libc::c_uint);
                s2 = s2.wrapping_add(s1);
                s1 =
                    s1.wrapping_add(*buf.offset((0 as libc::c_int +
                                                     4 as libc::c_int +
                                                     2 as libc::c_int) as
                                                    isize) as libc::c_uint);
                s2 = s2.wrapping_add(s1);
                s1 =
                    s1.wrapping_add(*buf.offset((0 as libc::c_int +
                                                     4 as libc::c_int +
                                                     2 as libc::c_int +
                                                     1 as libc::c_int) as
                                                    isize) as libc::c_uint);
                s2 = s2.wrapping_add(s1);
                s1 =
                    s1.wrapping_add(*buf.offset((0 as libc::c_int +
                                                     8 as libc::c_int) as
                                                    isize) as libc::c_uint);
                s2 = s2.wrapping_add(s1);
                s1 =
                    s1.wrapping_add(*buf.offset((0 as libc::c_int +
                                                     8 as libc::c_int +
                                                     1 as libc::c_int) as
                                                    isize) as libc::c_uint);
                s2 = s2.wrapping_add(s1);
                s1 =
                    s1.wrapping_add(*buf.offset((0 as libc::c_int +
                                                     8 as libc::c_int +
                                                     2 as libc::c_int) as
                                                    isize) as libc::c_uint);
                s2 = s2.wrapping_add(s1);
                s1 =
                    s1.wrapping_add(*buf.offset((0 as libc::c_int +
                                                     8 as libc::c_int +
                                                     2 as libc::c_int +
                                                     1 as libc::c_int) as
                                                    isize) as libc::c_uint);
                s2 = s2.wrapping_add(s1);
                s1 =
                    s1.wrapping_add(*buf.offset((0 as libc::c_int +
                                                     8 as libc::c_int +
                                                     4 as libc::c_int) as
                                                    isize) as libc::c_uint);
                s2 = s2.wrapping_add(s1);
                s1 =
                    s1.wrapping_add(*buf.offset((0 as libc::c_int +
                                                     8 as libc::c_int +
                                                     4 as libc::c_int +
                                                     1 as libc::c_int) as
                                                    isize) as libc::c_uint);
                s2 = s2.wrapping_add(s1);
                s1 =
                    s1.wrapping_add(*buf.offset((0 as libc::c_int +
                                                     8 as libc::c_int +
                                                     4 as libc::c_int +
                                                     2 as libc::c_int) as
                                                    isize) as libc::c_uint);
                s2 = s2.wrapping_add(s1);
                s1 =
                    s1.wrapping_add(*buf.offset((0 as libc::c_int +
                                                     8 as libc::c_int +
                                                     4 as libc::c_int +
                                                     2 as libc::c_int +
                                                     1 as libc::c_int) as
                                                    isize) as libc::c_uint);
                s2 = s2.wrapping_add(s1);
                buf = buf.offset(16 as libc::c_int as isize);
                k = k.wrapping_sub(16 as libc::c_int as libc::c_uint);
                if !(k >= 16 as libc::c_int as libc::c_uint) { break ; }
            }
        }
        if k != 0 as libc::c_int as libc::c_uint {
            loop  {
                let fresh0 = buf;
                buf = buf.offset(1);
                s1 = s1.wrapping_add(*fresh0 as libc::c_uint);
                s2 = s2.wrapping_add(s1);
                k = k.wrapping_sub(1);
                if !(k > 0 as libc::c_int as libc::c_uint) { break ; }
            }
        }
        s1 = s1.wrapping_rem(65521 as libc::c_uint);
        s2 = s2.wrapping_rem(65521 as libc::c_uint)
    }
    return s2 << 16 as libc::c_int | s1;
}
#[no_mangle]
pub unsafe extern "C" fn lzo_memcmp(mut s1: *const libc::c_void,
                                    mut s2: *const libc::c_void,
                                    mut len: lzo_uint) -> libc::c_int {
    return memcmp(s1, s2, len);
}
/* **********************************************************************
// function types
************************************************************************/
/* name mangling */
/* calling convention */
/* DLL export information */
/*empty*/
/*empty*/
/* __cdecl calling convention for public C and assembly functions */
/* function types */
/* Callback interface. Currently only the progress indicator ("nprogress")
 * is used, but this may change in a future release. */
/* malloc & free function types */
/* a progress indicator callback function */
/* custom allocators (set to 0 to disable) */
/* [not used right now] */
/* [not used right now] */
/* a progress indicator callback function (set to 0 to disable) */
/* INFO: the first parameter "self" of the nalloc/nfree/nprogress
     * callbacks points back to this struct, so you are free to store
     * some extra info in the following variables. */
/* **********************************************************************
// error codes and prototypes
************************************************************************/
/* Error codes for the compression/decompression functions. Negative
 * values are errors, positive values will be used for special but
 * normal events.
 */
/* [lzo_alloc_func_t failure] */
/* [not used right now] */
/* [not used right now] */
/* pointer argument is not properly aligned */
/* lzo_init() should be the first function you call.
 * Check the return code !
 *
 * lzo_init() is a macro to allow checking that the library and the
 * compiler's view of various types are consistent.
 */
/* version functions (useful for shared libraries) */
/* string functions */
#[no_mangle]
pub unsafe extern "C" fn lzo_memcpy(mut dest: *mut libc::c_void,
                                    mut src: *const libc::c_void,
                                    mut len: lzo_uint) -> *mut libc::c_void {
    return memcpy(dest, src, len);
}
#[no_mangle]
pub unsafe extern "C" fn lzo_memmove(mut dest: *mut libc::c_void,
                                     mut src: *const libc::c_void,
                                     mut len: lzo_uint) -> *mut libc::c_void {
    return memmove(dest, src, len);
}
#[no_mangle]
pub unsafe extern "C" fn lzo_memset(mut s: *mut libc::c_void,
                                    mut cc: libc::c_int, mut len: lzo_uint)
 -> *mut libc::c_void {
    return memset(s, cc, len);
}
#[inline(never)]
unsafe extern "C" fn u2p(mut ptr: *mut libc::c_void, mut off: lzo_uint)
 -> *mut libc::c_void {
    return (ptr as *mut libc::c_uchar).offset(off as isize) as
               *mut libc::c_void;
}
/* misc. */
#[no_mangle]
pub unsafe extern "C" fn _lzo_config_check() -> libc::c_int {
    let mut u: lzo_config_check_union = lzo_config_check_union{a: [0; 2],};
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut r: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    u.a[1 as libc::c_int as usize] = 0 as libc::c_int as lzo_uint;
    u.a[0 as libc::c_int as usize] = u.a[1 as libc::c_int as usize];
    p =
        u2p(&mut u as *mut lzo_config_check_union as *mut libc::c_void,
            0 as libc::c_int as lzo_uint);
    r &=
        (*(p as *mut libc::c_uchar) as libc::c_int == 0 as libc::c_int) as
            libc::c_int as libc::c_uint;
    u.a[1 as libc::c_int as usize] = 0 as libc::c_int as lzo_uint;
    u.a[0 as libc::c_int as usize] = u.a[1 as libc::c_int as usize];
    u.b[0 as libc::c_int as usize] = 128 as libc::c_int as libc::c_uchar;
    p =
        u2p(&mut u as *mut lzo_config_check_union as *mut libc::c_void,
            0 as libc::c_int as lzo_uint);
    r &=
        (*(p as *mut lzo_uint) == 128 as libc::c_int as libc::c_ulong) as
            libc::c_int as libc::c_uint;
    u.a[1 as libc::c_int as usize] = 0 as libc::c_int as lzo_uint;
    u.a[0 as libc::c_int as usize] = u.a[1 as libc::c_int as usize];
    u.b[0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    u.b[3 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    p =
        u2p(&mut u as *mut lzo_config_check_union as *mut libc::c_void,
            1 as libc::c_int as lzo_uint);
    r &=
        (*(p as *const libc::c_void as *const lzo_memops_TU2) as libc::c_int
             == 0 as libc::c_int) as libc::c_int as libc::c_uint;
    r &=
        (*(p as *const libc::c_void as *const lzo_memops_TU2) as libc::c_int
             == 0 as libc::c_int) as libc::c_int as libc::c_uint;
    u.b[1 as libc::c_int as usize] = 128 as libc::c_int as libc::c_uchar;
    r &=
        (*(p as *const libc::c_void as *const lzo_memops_TU2) as libc::c_int
             == 128 as libc::c_int) as libc::c_int as libc::c_uint;
    u.b[2 as libc::c_int as usize] = 129 as libc::c_int as libc::c_uchar;
    r &=
        (*(p as *const libc::c_void as *const lzo_memops_TU2) as libc::c_uint
             == 0x8180 as libc::c_uint) as libc::c_int as libc::c_uint;
    r &=
        (*(p as *const libc::c_void as *const lzo_memops_TU2) as libc::c_uint
             == 0x8180 as libc::c_uint) as libc::c_int as libc::c_uint;
    u.a[1 as libc::c_int as usize] = 0 as libc::c_int as lzo_uint;
    u.a[0 as libc::c_int as usize] = u.a[1 as libc::c_int as usize];
    u.b[0 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
    u.b[5 as libc::c_int as usize] = 4 as libc::c_int as libc::c_uchar;
    p =
        u2p(&mut u as *mut lzo_config_check_union as *mut libc::c_void,
            1 as libc::c_int as lzo_uint);
    r &=
        (*(p as *const libc::c_void as *const lzo_memops_TU4) ==
             0 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_uint;
    r &=
        (*(p as *const libc::c_void as *const lzo_memops_TU4) ==
             0 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_uint;
    u.b[1 as libc::c_int as usize] = 128 as libc::c_int as libc::c_uchar;
    r &=
        (*(p as *const libc::c_void as *const lzo_memops_TU4) ==
             128 as libc::c_int as libc::c_uint) as libc::c_int as
            libc::c_uint;
    u.b[2 as libc::c_int as usize] = 129 as libc::c_int as libc::c_uchar;
    u.b[3 as libc::c_int as usize] = 130 as libc::c_int as libc::c_uchar;
    u.b[4 as libc::c_int as usize] = 131 as libc::c_int as libc::c_uchar;
    r &=
        (*(p as *const libc::c_void as *const lzo_memops_TU4) ==
             0x83828180 as libc::c_uint) as libc::c_int as libc::c_uint;
    r &=
        (*(p as *const libc::c_void as *const lzo_memops_TU4) ==
             0x83828180 as libc::c_uint) as libc::c_int as libc::c_uint;
    u.c[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ulong;
    u.c[0 as libc::c_int as usize] = u.c[1 as libc::c_int as usize];
    u.b[0 as libc::c_int as usize] = 5 as libc::c_int as libc::c_uchar;
    u.b[9 as libc::c_int as usize] = 6 as libc::c_int as libc::c_uchar;
    p =
        u2p(&mut u as *mut lzo_config_check_union as *mut libc::c_void,
            1 as libc::c_int as lzo_uint);
    u.c[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ulong;
    u.c[0 as libc::c_int as usize] = u.c[1 as libc::c_int as usize];
    r &=
        (*(p as *const libc::c_void as *const lzo_memops_TU8) ==
             0 as libc::c_int as libc::c_ulong) as libc::c_int as
            libc::c_uint;
    r &=
        (*(p as *const libc::c_void as *const lzo_memops_TU8) ==
             0 as libc::c_int as libc::c_ulong) as libc::c_int as
            libc::c_uint;
    u.b[1 as libc::c_int as usize] = 128 as libc::c_int as libc::c_uchar;
    r &=
        (*(p as *const libc::c_void as *const lzo_memops_TU8) ==
             128 as libc::c_int as libc::c_ulong) as libc::c_int as
            libc::c_uint;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut v: libc::c_uint = 0;
    v = 1 as libc::c_int as libc::c_uint;
    while v != 0 as libc::c_int as libc::c_uint &&
              r == 1 as libc::c_int as libc::c_uint {
        r &=
            (v.leading_zeros() as i32 as libc::c_uint ==
                 (31 as libc::c_int as libc::c_uint).wrapping_sub(i)) as
                libc::c_int as libc::c_uint;
        r &=
            (lzo_bitops_ctlz32_func(v) ==
                 (31 as libc::c_int as libc::c_uint).wrapping_sub(i)) as
                libc::c_int as libc::c_uint;
        v <<= 1 as libc::c_int;
        i = i.wrapping_add(1)
    }
    let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut v_0: libc::c_ulong = 0;
    v_0 = 1 as libc::c_int as libc::c_ulong;
    while v_0 != 0 as libc::c_int as libc::c_ulong &&
              r == 1 as libc::c_int as libc::c_uint {
        r &=
            (v_0.leading_zeros() as i32 as libc::c_uint ==
                 (63 as libc::c_int as libc::c_uint).wrapping_sub(i_0)) as
                libc::c_int as libc::c_uint;
        r &=
            (lzo_bitops_ctlz64_func(v_0) ==
                 (63 as libc::c_int as libc::c_uint).wrapping_sub(i_0)) as
                libc::c_int as libc::c_uint;
        v_0 <<= 1 as libc::c_int;
        i_0 = i_0.wrapping_add(1)
    }
    let mut i_1: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut v_1: libc::c_uint = 0;
    v_1 = 1 as libc::c_int as libc::c_uint;
    while v_1 != 0 as libc::c_int as libc::c_uint &&
              r == 1 as libc::c_int as libc::c_uint {
        r &=
            (v_1.trailing_zeros() as i32 as libc::c_uint == i_1) as
                libc::c_int as libc::c_uint;
        r &=
            (lzo_bitops_cttz32_func(v_1) == i_1) as libc::c_int as
                libc::c_uint;
        v_1 <<= 1 as libc::c_int;
        i_1 = i_1.wrapping_add(1)
    }
    let mut i_2: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut v_2: libc::c_ulong = 0;
    v_2 = 1 as libc::c_int as libc::c_ulong;
    while v_2 != 0 as libc::c_int as libc::c_ulong &&
              r == 1 as libc::c_int as libc::c_uint {
        r &=
            (v_2.trailing_zeros() as i32 as libc::c_uint == i_2) as
                libc::c_int as libc::c_uint;
        r &=
            (lzo_bitops_cttz64_func(v_2) == i_2) as libc::c_int as
                libc::c_uint;
        v_2 <<= 1 as libc::c_int;
        i_2 = i_2.wrapping_add(1)
    }
    return if r == 1 as libc::c_int as libc::c_uint {
               0 as libc::c_int
           } else { -(1 as libc::c_int) };
}
#[no_mangle]
pub unsafe extern "C" fn lzo_initialize() -> libc::c_int {
    return __lzo_init_v2(0x20a0 as libc::c_int as libc::c_uint,
                         ::std::mem::size_of::<libc::c_short>() as
                             libc::c_ulong as libc::c_int,
                         ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                             as libc::c_int,
                         ::std::mem::size_of::<libc::c_long>() as
                             libc::c_ulong as libc::c_int,
                         ::std::mem::size_of::<libc::c_uint>() as
                             libc::c_ulong as libc::c_int,
                         ::std::mem::size_of::<lzo_uint>() as libc::c_ulong as
                             libc::c_int,
                         ::std::mem::size_of::<*mut libc::c_uchar>() as
                             libc::c_ulong as libc::c_uint as libc::c_int,
                         ::std::mem::size_of::<*mut libc::c_char>() as
                             libc::c_ulong as libc::c_int,
                         ::std::mem::size_of::<*mut libc::c_void>() as
                             libc::c_ulong as libc::c_int,
                         ::std::mem::size_of::<lzo_callback_t>() as
                             libc::c_ulong as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn __lzo_init_v2(mut v: libc::c_uint,
                                       mut s1: libc::c_int,
                                       mut s2: libc::c_int,
                                       mut s3: libc::c_int,
                                       mut s4: libc::c_int,
                                       mut s5: libc::c_int,
                                       mut s6: libc::c_int,
                                       mut s7: libc::c_int,
                                       mut s8: libc::c_int,
                                       mut s9: libc::c_int) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if v == 0 as libc::c_int as libc::c_uint { return -(1 as libc::c_int) }
    r =
        ((s1 == -(1 as libc::c_int) ||
              s1 ==
                  ::std::mem::size_of::<libc::c_short>() as libc::c_ulong as
                      libc::c_int) &&
             (s2 == -(1 as libc::c_int) ||
                  s2 ==
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          libc::c_int) &&
             (s3 == -(1 as libc::c_int) ||
                  s3 ==
                      ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                          as libc::c_int) &&
             (s4 == -(1 as libc::c_int) ||
                  s4 ==
                      ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong
                          as libc::c_int) &&
             (s5 == -(1 as libc::c_int) ||
                  s5 ==
                      ::std::mem::size_of::<lzo_uint>() as libc::c_ulong as
                          libc::c_int) &&
             (s6 == -(1 as libc::c_int) ||
                  s6 ==
                      ::std::mem::size_of::<*mut libc::c_uchar>() as
                          libc::c_ulong as libc::c_uint as libc::c_int) &&
             (s7 == -(1 as libc::c_int) ||
                  s7 ==
                      ::std::mem::size_of::<*mut libc::c_char>() as
                          libc::c_ulong as libc::c_int) &&
             (s8 == -(1 as libc::c_int) ||
                  s8 ==
                      ::std::mem::size_of::<*mut libc::c_void>() as
                          libc::c_ulong as libc::c_int) &&
             (s9 == -(1 as libc::c_int) ||
                  s9 ==
                      ::std::mem::size_of::<lzo_callback_t>() as libc::c_ulong
                          as libc::c_int)) as libc::c_int;
    if r == 0 { return -(1 as libc::c_int) }
    r = _lzo_config_check();
    if r != 0 as libc::c_int { return r }
    return r;
}
#[inline(never)]
unsafe extern "C" fn lzo1x_1_compress_core(mut in_0: *const libc::c_uchar,
                                           mut in_len: lzo_uint,
                                           mut out: *mut libc::c_uchar,
                                           mut out_len: *mut lzo_uint,
                                           mut ti: lzo_uint,
                                           mut wrkmem: *mut libc::c_void)
 -> lzo_uint {
    let mut ip: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut op: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let in_end: *const libc::c_uchar = in_0.offset(in_len as isize);
    let ip_end: *const libc::c_uchar =
        in_0.offset(in_len as isize).offset(-(20 as libc::c_int as isize));
    let mut ii: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let dict: *mut libc::c_ushort = wrkmem as *mut libc::c_ushort;
    op = out;
    ip = in_0;
    ii = ip;
    ip =
        ip.offset(if ti < 4 as libc::c_int as libc::c_ulong {
                      (4 as libc::c_int as libc::c_ulong).wrapping_sub(ti)
                  } else { 0 as libc::c_int as libc::c_ulong } as isize);
    let mut current_block_97: u64;
    let mut m_pos: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut m_off: lzo_uint = 0;
    let mut m_len: lzo_uint = 0;
    let mut dv: libc::c_uint = 0;
    let mut dindex: lzo_uint = 0;
    'c_3244:
        loop  {
            ip =
                ip.offset((1 as libc::c_int as libc::c_long +
                               (ip.wrapping_offset_from(ii) as libc::c_long >>
                                    5 as libc::c_int)) as isize);
            loop  {
                if (ip >= ip_end) as libc::c_int as libc::c_long != 0 {
                    break 'c_3244 ;
                }
                dv = *(ip as *const libc::c_void as *const lzo_memops_TU4);
                dindex =
                    ((0x1824429d as libc::c_int as
                          libc::c_uint).wrapping_mul(dv) as lzo_uint >>
                         32 as libc::c_int - 14 as libc::c_int &
                         (((1 as libc::c_uint) <<
                               14 as
                                   libc::c_int).wrapping_sub(1 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                              >> 0 as libc::c_int) as libc::c_ulong) <<
                        0 as libc::c_int;
                m_pos =
                    in_0.offset(*dict.offset(dindex as isize) as libc::c_int
                                    as isize);
                *dict.offset(dindex as isize) =
                    ip.wrapping_offset_from(in_0) as libc::c_long as lzo_uint
                        as libc::c_ushort;
                if (dv !=
                        *(m_pos as *const libc::c_void as
                              *const lzo_memops_TU4)) as libc::c_int as
                       libc::c_long != 0 {
                    break ;
                }
                ii = ii.offset(-(ti as isize));
                ti = 0 as libc::c_int as lzo_uint;
                let mut t: lzo_uint =
                    ip.wrapping_offset_from(ii) as libc::c_long as lzo_uint;
                if t != 0 as libc::c_int as libc::c_ulong {
                    if t <= 3 as libc::c_int as libc::c_ulong {
                        *op.offset(-(2 as libc::c_int) as isize) =
                            (*op.offset(-(2 as libc::c_int) as isize) as
                                 libc::c_ulong | t) as libc::c_uchar;
                        ::std::ptr::write_volatile(op as *mut libc::c_void as
                                                       *mut lzo_memops_TU4,
                                                   *(ii as *const libc::c_void
                                                         as
                                                         *const lzo_memops_TU4));
                        op = op.offset(t as isize)
                    } else if t <= 16 as libc::c_int as libc::c_ulong {
                        let fresh1 = op;
                        op = op.offset(1);
                        *fresh1 =
                            t.wrapping_sub(3 as libc::c_int as libc::c_ulong)
                                as libc::c_uchar;
                        ::std::ptr::write_volatile(op as *mut libc::c_void as
                                                       *mut lzo_memops_TU8,
                                                   *(ii as *const libc::c_void
                                                         as
                                                         *const lzo_memops_TU8));
                        ::std::ptr::write_volatile(op.offset(8 as libc::c_int
                                                                 as isize) as
                                                       *mut libc::c_void as
                                                       *mut lzo_memops_TU8,
                                                   *(ii.offset(8 as
                                                                   libc::c_int
                                                                   as isize)
                                                         as
                                                         *const libc::c_void
                                                         as
                                                         *const lzo_memops_TU8));
                        op = op.offset(t as isize)
                    } else {
                        if t <= 18 as libc::c_int as libc::c_ulong {
                            let fresh2 = op;
                            op = op.offset(1);
                            *fresh2 =
                                t.wrapping_sub(3 as libc::c_int as
                                                   libc::c_ulong) as
                                    libc::c_uchar
                        } else {
                            let mut tt: lzo_uint =
                                t.wrapping_sub(18 as libc::c_int as
                                                   libc::c_ulong);
                            let fresh3 = op;
                            op = op.offset(1);
                            *fresh3 = 0 as libc::c_int as libc::c_uchar;
                            while (tt > 255 as libc::c_int as libc::c_ulong)
                                      as libc::c_int as libc::c_long != 0 {
                                tt =
                                    (tt as
                                         libc::c_ulong).wrapping_sub(255 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong)
                                        as lzo_uint as lzo_uint;
                                loop  {
                                    let mut d__1: *mut libc::c_uchar =
                                        op as *mut libc::c_void as
                                            *mut libc::c_uchar;
                                    ::std::ptr::write_volatile(d__1.offset(0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                                               0 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uchar);
                                    if !(0 as libc::c_int != 0) { break ; }
                                }
                                op = op.offset(1)
                            }
                            let fresh4 = op;
                            op = op.offset(1);
                            *fresh4 = tt as libc::c_uchar
                        }
                        loop  {
                            ::std::ptr::write_volatile(op as *mut libc::c_void
                                                           as
                                                           *mut lzo_memops_TU8,
                                                       *(ii as
                                                             *const libc::c_void
                                                             as
                                                             *const lzo_memops_TU8));
                            ::std::ptr::write_volatile(op.offset(8 as
                                                                     libc::c_int
                                                                     as isize)
                                                           as
                                                           *mut libc::c_void
                                                           as
                                                           *mut lzo_memops_TU8,
                                                       *(ii.offset(8 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                                                             as
                                                             *const libc::c_void
                                                             as
                                                             *const lzo_memops_TU8));
                            op = op.offset(16 as libc::c_int as isize);
                            ii = ii.offset(16 as libc::c_int as isize);
                            t =
                                (t as
                                     libc::c_ulong).wrapping_sub(16 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                                    as lzo_uint as lzo_uint;
                            if !(t >= 16 as libc::c_int as libc::c_ulong) {
                                break ;
                            }
                        }
                        if t > 0 as libc::c_int as libc::c_ulong {
                            loop  {
                                let fresh5 = ii;
                                ii = ii.offset(1);
                                let fresh6 = op;
                                op = op.offset(1);
                                *fresh6 = *fresh5;
                                t = t.wrapping_sub(1);
                                if !(t > 0 as libc::c_int as libc::c_ulong) {
                                    break ;
                                }
                            }
                        }
                    }
                }
                m_len = 4 as libc::c_int as lzo_uint;
                let mut v: libc::c_ulong = 0;
                v =
                    *(ip.offset(m_len as isize) as *const libc::c_void as
                          *const lzo_memops_TU8) ^
                        *(m_pos.offset(m_len as isize) as *const libc::c_void
                              as *const lzo_memops_TU8);
                if (v == 0 as libc::c_int as libc::c_ulong) as libc::c_int as
                       libc::c_long != 0 {
                    current_block_97 = 4888910987971495881;
                } else { current_block_97 = 12758904613967585247; }
                loop  {
                    match current_block_97 {
                        12758904613967585247 => {
                            m_len =
                                (m_len as
                                     libc::c_ulong).wrapping_add((v.trailing_zeros()
                                                                      as i32
                                                                      as
                                                                      libc::c_uint).wrapping_div(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_uint)
                                                                     as
                                                                     libc::c_ulong)
                                    as lzo_uint as lzo_uint;
                            break ;
                        }
                        _ => {
                            m_len =
                                (m_len as
                                     libc::c_ulong).wrapping_add(8 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                                    as lzo_uint as lzo_uint;
                            v =
                                *(ip.offset(m_len as isize) as
                                      *const libc::c_void as
                                      *const lzo_memops_TU8) ^
                                    *(m_pos.offset(m_len as isize) as
                                          *const libc::c_void as
                                          *const lzo_memops_TU8);
                            if (ip.offset(m_len as isize) >= ip_end) as
                                   libc::c_int as libc::c_long != 0 {
                                break ;
                            }
                            if v == 0 as libc::c_int as libc::c_ulong {
                                current_block_97 = 4888910987971495881;
                            } else {
                                current_block_97 = 12758904613967585247;
                            }
                        }
                    }
                }
                m_off =
                    ip.wrapping_offset_from(m_pos) as libc::c_long as
                        lzo_uint;
                ip = ip.offset(m_len as isize);
                ii = ip;
                if m_len <= 8 as libc::c_int as libc::c_ulong &&
                       m_off <= 0x800 as libc::c_int as libc::c_ulong {
                    m_off =
                        (m_off as
                             libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                             libc::c_ulong) as
                            lzo_uint as lzo_uint;
                    let fresh7 = op;
                    op = op.offset(1);
                    *fresh7 =
                        (m_len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                             << 5 as libc::c_int |
                             (m_off & 7 as libc::c_int as libc::c_ulong) <<
                                 2 as libc::c_int) as libc::c_uchar;
                    let fresh8 = op;
                    op = op.offset(1);
                    *fresh8 = (m_off >> 3 as libc::c_int) as libc::c_uchar
                } else if m_off <= 0x4000 as libc::c_int as libc::c_ulong {
                    m_off =
                        (m_off as
                             libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                             libc::c_ulong) as
                            lzo_uint as lzo_uint;
                    if m_len <= 33 as libc::c_int as libc::c_ulong {
                        let fresh9 = op;
                        op = op.offset(1);
                        *fresh9 =
                            (32 as libc::c_int as libc::c_ulong |
                                 m_len.wrapping_sub(2 as libc::c_int as
                                                        libc::c_ulong)) as
                                libc::c_uchar
                    } else {
                        m_len =
                            (m_len as
                                 libc::c_ulong).wrapping_sub(33 as libc::c_int
                                                                 as
                                                                 libc::c_ulong)
                                as lzo_uint as lzo_uint;
                        let fresh10 = op;
                        op = op.offset(1);
                        *fresh10 =
                            (32 as libc::c_int | 0 as libc::c_int) as
                                libc::c_uchar;
                        while (m_len > 255 as libc::c_int as libc::c_ulong) as
                                  libc::c_int as libc::c_long != 0 {
                            m_len =
                                (m_len as
                                     libc::c_ulong).wrapping_sub(255 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                                    as lzo_uint as lzo_uint;
                            loop  {
                                let mut d__1_0: *mut libc::c_uchar =
                                    op as *mut libc::c_void as
                                        *mut libc::c_uchar;
                                ::std::ptr::write_volatile(d__1_0.offset(0 as
                                                                             libc::c_int
                                                                             as
                                                                             isize),
                                                           0 as libc::c_int as
                                                               libc::c_uchar);
                                if !(0 as libc::c_int != 0) { break ; }
                            }
                            op = op.offset(1)
                        }
                        let fresh11 = op;
                        op = op.offset(1);
                        *fresh11 = m_len as libc::c_uchar
                    }
                    let fresh12 = op;
                    op = op.offset(1);
                    *fresh12 = (m_off << 2 as libc::c_int) as libc::c_uchar;
                    let fresh13 = op;
                    op = op.offset(1);
                    *fresh13 = (m_off >> 6 as libc::c_int) as libc::c_uchar
                } else {
                    m_off =
                        (m_off as
                             libc::c_ulong).wrapping_sub(0x4000 as libc::c_int
                                                             as libc::c_ulong)
                            as lzo_uint as lzo_uint;
                    if m_len <= 9 as libc::c_int as libc::c_ulong {
                        let fresh14 = op;
                        op = op.offset(1);
                        *fresh14 =
                            (16 as libc::c_int as libc::c_ulong |
                                 m_off >> 11 as libc::c_int &
                                     8 as libc::c_int as libc::c_ulong |
                                 m_len.wrapping_sub(2 as libc::c_int as
                                                        libc::c_ulong)) as
                                libc::c_uchar
                    } else {
                        m_len =
                            (m_len as
                                 libc::c_ulong).wrapping_sub(9 as libc::c_int
                                                                 as
                                                                 libc::c_ulong)
                                as lzo_uint as lzo_uint;
                        let fresh15 = op;
                        op = op.offset(1);
                        *fresh15 =
                            (16 as libc::c_int as libc::c_ulong |
                                 m_off >> 11 as libc::c_int &
                                     8 as libc::c_int as libc::c_ulong) as
                                libc::c_uchar;
                        while (m_len > 255 as libc::c_int as libc::c_ulong) as
                                  libc::c_int as libc::c_long != 0 {
                            m_len =
                                (m_len as
                                     libc::c_ulong).wrapping_sub(255 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                                    as lzo_uint as lzo_uint;
                            loop  {
                                let mut d__1_1: *mut libc::c_uchar =
                                    op as *mut libc::c_void as
                                        *mut libc::c_uchar;
                                ::std::ptr::write_volatile(d__1_1.offset(0 as
                                                                             libc::c_int
                                                                             as
                                                                             isize),
                                                           0 as libc::c_int as
                                                               libc::c_uchar);
                                if !(0 as libc::c_int != 0) { break ; }
                            }
                            op = op.offset(1)
                        }
                        let fresh16 = op;
                        op = op.offset(1);
                        *fresh16 = m_len as libc::c_uchar
                    }
                    let fresh17 = op;
                    op = op.offset(1);
                    *fresh17 = (m_off << 2 as libc::c_int) as libc::c_uchar;
                    let fresh18 = op;
                    op = op.offset(1);
                    *fresh18 = (m_off >> 6 as libc::c_int) as libc::c_uchar
                }
            }
        }
    *out_len = op.wrapping_offset_from(out) as libc::c_long as lzo_uint;
    return in_end.wrapping_offset_from(ii.offset(-(ti as isize))) as
               libc::c_long as lzo_uint;
}
/* minilzo.h -- mini subset of the LZO real-time data compression library

   This file is part of the LZO real-time data compression library.

   Copyright (C) 1996-2017 Markus Franz Xaver Johannes Oberhumer
   All Rights Reserved.

   The LZO library is free software; you can redistribute it and/or
   modify it under the terms of the GNU General Public License as
   published by the Free Software Foundation; either version 2 of
   the License, or (at your option) any later version.

   The LZO library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with the LZO library; see the file COPYING.
   If not, write to the Free Software Foundation, Inc.,
   51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.

   Markus F.X.J. Oberhumer
   <markus@oberhumer.com>
   http://www.oberhumer.com/opensource/lzo/
 */
/*
 * NOTE:
 *   the full LZO package can be found at
 *   http://www.oberhumer.com/opensource/lzo/
 */
/* 2.10 */
/* internal Autoconf configuration file - only used when building miniLZO */
/* **********************************************************************
//
************************************************************************/
/* Memory required for the wrkmem parameter.
 * When the required size is 0, you can also pass a NULL pointer.
 */
/* compression */
#[no_mangle]
pub unsafe extern "C" fn lzo1x_1_compress(mut in_0: *const libc::c_uchar,
                                          mut in_len: lzo_uint,
                                          mut out: *mut libc::c_uchar,
                                          mut out_len: *mut lzo_uint,
                                          mut wrkmem: *mut libc::c_void)
 -> libc::c_int {
    let mut ip: *const libc::c_uchar = in_0;
    let mut op: *mut libc::c_uchar = out;
    let mut l: lzo_uint = in_len;
    let mut t: lzo_uint = 0 as libc::c_int as lzo_uint;
    while l > 20 as libc::c_int as libc::c_ulong {
        let mut ll: lzo_uint = l;
        let mut ll_end: libc::c_ulong = 0;
        ll =
            if ll <= 49152 as libc::c_int as libc::c_ulong {
                ll
            } else { 49152 as libc::c_int as libc::c_ulong };
        ll_end = (ip as libc::c_ulong).wrapping_add(ll);
        if ll_end.wrapping_add(t.wrapping_add(ll) >> 5 as libc::c_int) <=
               ll_end ||
               ll_end.wrapping_add(t.wrapping_add(ll) >> 5 as libc::c_int) as
                   *const libc::c_uchar <= ip.offset(ll as isize) {
            break ;
        }
        lzo_memset(wrkmem, 0 as libc::c_int,
                   ((1 as libc::c_int as lzo_uint) <<
                        14 as
                            libc::c_int).wrapping_mul(::std::mem::size_of::<libc::c_ushort>()
                                                          as libc::c_ulong));
        t = lzo1x_1_compress_core(ip, ll, op, out_len, t, wrkmem);
        ip = ip.offset(ll as isize);
        op = op.offset(*out_len as isize);
        l = (l as libc::c_ulong).wrapping_sub(ll) as lzo_uint as lzo_uint
    }
    t = (t as libc::c_ulong).wrapping_add(l) as lzo_uint as lzo_uint;
    if t > 0 as libc::c_int as libc::c_ulong {
        let mut ii: *const libc::c_uchar =
            in_0.offset(in_len as isize).offset(-(t as isize));
        if op == out && t <= 238 as libc::c_int as libc::c_ulong {
            let fresh19 = op;
            op = op.offset(1);
            *fresh19 =
                (17 as libc::c_int as libc::c_ulong).wrapping_add(t) as
                    libc::c_uchar
        } else if t <= 3 as libc::c_int as libc::c_ulong {
            *op.offset(-(2 as libc::c_int) as isize) =
                (*op.offset(-(2 as libc::c_int) as isize) as libc::c_ulong |
                     t) as libc::c_uchar
        } else if t <= 18 as libc::c_int as libc::c_ulong {
            let fresh20 = op;
            op = op.offset(1);
            *fresh20 =
                t.wrapping_sub(3 as libc::c_int as libc::c_ulong) as
                    libc::c_uchar
        } else {
            let mut tt: lzo_uint =
                t.wrapping_sub(18 as libc::c_int as libc::c_ulong);
            let fresh21 = op;
            op = op.offset(1);
            *fresh21 = 0 as libc::c_int as libc::c_uchar;
            while tt > 255 as libc::c_int as libc::c_ulong {
                tt =
                    (tt as
                         libc::c_ulong).wrapping_sub(255 as libc::c_int as
                                                         libc::c_ulong) as
                        lzo_uint as lzo_uint;
                loop  {
                    let mut d__1: *mut libc::c_uchar =
                        op as *mut libc::c_void as *mut libc::c_uchar;
                    ::std::ptr::write_volatile(d__1.offset(0 as libc::c_int as
                                                               isize),
                                               0 as libc::c_int as
                                                   libc::c_uchar);
                    if !(0 as libc::c_int != 0) { break ; }
                }
                op = op.offset(1)
            }
            let fresh22 = op;
            op = op.offset(1);
            *fresh22 = tt as libc::c_uchar
        }
        loop  {
            let mut d__n: *mut libc::c_uchar =
                op as *mut libc::c_void as *mut libc::c_uchar;
            let mut s__n: *const libc::c_uchar =
                ii as *const libc::c_void as *const libc::c_uchar;
            let mut n__n: lzo_uint = t;
            while n__n >= 8 as libc::c_int as libc::c_ulong {
                ::std::ptr::write_volatile(d__n as *mut libc::c_void as
                                               *mut lzo_memops_TU8,
                                           *(s__n as *const libc::c_void as
                                                 *const lzo_memops_TU8));
                d__n = d__n.offset(8 as libc::c_int as isize);
                s__n = s__n.offset(8 as libc::c_int as isize);
                n__n =
                    (n__n as
                         libc::c_ulong).wrapping_sub(8 as libc::c_int as
                                                         libc::c_ulong) as
                        lzo_uint as lzo_uint
            }
            if n__n >= 4 as libc::c_int as libc::c_ulong {
                ::std::ptr::write_volatile(d__n as *mut libc::c_void as
                                               *mut lzo_memops_TU4,
                                           *(s__n as *const libc::c_void as
                                                 *const lzo_memops_TU4));
                d__n = d__n.offset(4 as libc::c_int as isize);
                s__n = s__n.offset(4 as libc::c_int as isize);
                n__n =
                    (n__n as
                         libc::c_ulong).wrapping_sub(4 as libc::c_int as
                                                         libc::c_ulong) as
                        lzo_uint as lzo_uint
            }
            if n__n > 0 as libc::c_int as libc::c_ulong {
                loop  {
                    let fresh23 = s__n;
                    s__n = s__n.offset(1);
                    let fresh24 = d__n;
                    d__n = d__n.offset(1);
                    *fresh24 = *fresh23;
                    n__n = n__n.wrapping_sub(1);
                    if !(n__n > 0 as libc::c_int as libc::c_ulong) { break ; }
                }
            }
            if !(0 as libc::c_int != 0) { break ; }
        }
        op = op.offset(t as isize)
    }
    let fresh25 = op;
    op = op.offset(1);
    *fresh25 = (16 as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
    let fresh26 = op;
    op = op.offset(1);
    *fresh26 = 0 as libc::c_int as libc::c_uchar;
    let fresh27 = op;
    op = op.offset(1);
    *fresh27 = 0 as libc::c_int as libc::c_uchar;
    *out_len = op.wrapping_offset_from(out) as libc::c_long as lzo_uint;
    return 0 as libc::c_int;
}
/* decompression */
#[no_mangle]
pub unsafe extern "C" fn lzo1x_decompress(mut in_0: *const libc::c_uchar,
                                          mut in_len: lzo_uint,
                                          mut out: *mut libc::c_uchar,
                                          mut out_len: *mut lzo_uint,
                                          mut wrkmem: *mut libc::c_void)
 -> libc::c_int {
    let mut current_block: u64;
    let mut op: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ip: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut t: lzo_uint = 0;
    let mut m_pos: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let ip_end: *const libc::c_uchar = in_0.offset(in_len as isize);
    *out_len = 0 as libc::c_int as lzo_uint;
    op = out;
    ip = in_0;
    if *ip as libc::c_int > 17 as libc::c_int {
        let fresh28 = ip;
        ip = ip.offset(1);
        t = (*fresh28 as libc::c_int - 17 as libc::c_int) as lzo_uint;
        if t < 4 as libc::c_int as libc::c_ulong {
            current_block = 16974974966130203269;
        } else {
            loop  {
                let fresh29 = ip;
                ip = ip.offset(1);
                let fresh30 = op;
                op = op.offset(1);
                *fresh30 = *fresh29;
                t = t.wrapping_sub(1);
                if !(t > 0 as libc::c_int as libc::c_ulong) { break ; }
            }
            current_block = 780924439199517896;
        }
    } else { current_block = 4956146061682418353; }
    loop  {
        match current_block {
            16974974966130203269 => {
                let fresh62 = ip;
                ip = ip.offset(1);
                let fresh63 = op;
                op = op.offset(1);
                *fresh63 = *fresh62;
                if t > 1 as libc::c_int as libc::c_ulong {
                    let fresh64 = ip;
                    ip = ip.offset(1);
                    let fresh65 = op;
                    op = op.offset(1);
                    *fresh65 = *fresh64;
                    if t > 2 as libc::c_int as libc::c_ulong {
                        let fresh66 = ip;
                        ip = ip.offset(1);
                        let fresh67 = op;
                        op = op.offset(1);
                        *fresh67 = *fresh66
                    }
                }
                let fresh68 = ip;
                ip = ip.offset(1);
                t = *fresh68 as lzo_uint;
                current_block = 3090853732581768918;
            }
            4956146061682418353 => {
                let fresh31 = ip;
                ip = ip.offset(1);
                t = *fresh31 as lzo_uint;
                if t >= 16 as libc::c_int as libc::c_ulong {
                    current_block = 3090853732581768918;
                } else {
                    if t == 0 as libc::c_int as libc::c_ulong {
                        while *ip as libc::c_int == 0 as libc::c_int {
                            t =
                                (t as
                                     libc::c_ulong).wrapping_add(255 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                                    as lzo_uint as lzo_uint;
                            ip = ip.offset(1)
                        }
                        let fresh32 = ip;
                        ip = ip.offset(1);
                        t =
                            (t as
                                 libc::c_ulong).wrapping_add((15 as
                                                                  libc::c_int
                                                                  +
                                                                  *fresh32 as
                                                                      libc::c_int)
                                                                 as
                                                                 libc::c_ulong)
                                as lzo_uint as lzo_uint
                    }
                    t =
                        (t as
                             libc::c_ulong).wrapping_add(3 as libc::c_int as
                                                             libc::c_ulong) as
                            lzo_uint as lzo_uint;
                    if t >= 8 as libc::c_int as libc::c_ulong {
                        loop  {
                            ::std::ptr::write_volatile(op as *mut libc::c_void
                                                           as
                                                           *mut lzo_memops_TU8,
                                                       *(ip as
                                                             *const libc::c_void
                                                             as
                                                             *const lzo_memops_TU8));
                            op = op.offset(8 as libc::c_int as isize);
                            ip = ip.offset(8 as libc::c_int as isize);
                            t =
                                (t as
                                     libc::c_ulong).wrapping_sub(8 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                                    as lzo_uint as lzo_uint;
                            if !(t >= 8 as libc::c_int as libc::c_ulong) {
                                break ;
                            }
                        }
                    }
                    if t >= 4 as libc::c_int as libc::c_ulong {
                        ::std::ptr::write_volatile(op as *mut libc::c_void as
                                                       *mut lzo_memops_TU4,
                                                   *(ip as *const libc::c_void
                                                         as
                                                         *const lzo_memops_TU4));
                        op = op.offset(4 as libc::c_int as isize);
                        ip = ip.offset(4 as libc::c_int as isize);
                        t =
                            (t as
                                 libc::c_ulong).wrapping_sub(4 as libc::c_int
                                                                 as
                                                                 libc::c_ulong)
                                as lzo_uint as lzo_uint
                    }
                    if t > 0 as libc::c_int as libc::c_ulong {
                        let fresh33 = ip;
                        ip = ip.offset(1);
                        let fresh34 = op;
                        op = op.offset(1);
                        *fresh34 = *fresh33;
                        if t > 1 as libc::c_int as libc::c_ulong {
                            let fresh35 = ip;
                            ip = ip.offset(1);
                            let fresh36 = op;
                            op = op.offset(1);
                            *fresh36 = *fresh35;
                            if t > 2 as libc::c_int as libc::c_ulong {
                                let fresh37 = ip;
                                ip = ip.offset(1);
                                let fresh38 = op;
                                op = op.offset(1);
                                *fresh38 = *fresh37
                            }
                        }
                    }
                    current_block = 780924439199517896;
                    continue ;
                }
            }
            _ => {
                let fresh39 = ip;
                ip = ip.offset(1);
                t = *fresh39 as lzo_uint;
                if t >= 16 as libc::c_int as libc::c_ulong {
                    current_block = 3090853732581768918;
                } else {
                    m_pos =
                        op.offset(-((1 as libc::c_int + 0x800 as libc::c_int)
                                        as isize));
                    m_pos = m_pos.offset(-((t >> 2 as libc::c_int) as isize));
                    let fresh40 = ip;
                    ip = ip.offset(1);
                    m_pos =
                        m_pos.offset(-(((*fresh40 as libc::c_int) <<
                                            2 as libc::c_int) as isize));
                    let fresh41 = m_pos;
                    m_pos = m_pos.offset(1);
                    let fresh42 = op;
                    op = op.offset(1);
                    *fresh42 = *fresh41;
                    let fresh43 = m_pos;
                    m_pos = m_pos.offset(1);
                    let fresh44 = op;
                    op = op.offset(1);
                    *fresh44 = *fresh43;
                    let fresh45 = op;
                    op = op.offset(1);
                    *fresh45 = *m_pos;
                    current_block = 5444525074679853294;
                }
            }
        }
        match current_block {
            3090853732581768918 => {
                if t >= 64 as libc::c_int as libc::c_ulong {
                    m_pos = op.offset(-(1 as libc::c_int as isize));
                    m_pos =
                        m_pos.offset(-((t >> 2 as libc::c_int &
                                            7 as libc::c_int as libc::c_ulong)
                                           as isize));
                    let fresh46 = ip;
                    ip = ip.offset(1);
                    m_pos =
                        m_pos.offset(-(((*fresh46 as libc::c_int) <<
                                            3 as libc::c_int) as isize));
                    t =
                        (t >>
                             5 as
                                 libc::c_int).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong);
                    current_block = 11354191643585990046;
                } else {
                    if t >= 32 as libc::c_int as libc::c_ulong {
                        t &= 31 as libc::c_int as libc::c_ulong;
                        if t == 0 as libc::c_int as libc::c_ulong {
                            while *ip as libc::c_int == 0 as libc::c_int {
                                t =
                                    (t as
                                         libc::c_ulong).wrapping_add(255 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong)
                                        as lzo_uint as lzo_uint;
                                ip = ip.offset(1)
                            }
                            let fresh47 = ip;
                            ip = ip.offset(1);
                            t =
                                (t as
                                     libc::c_ulong).wrapping_add((31 as
                                                                      libc::c_int
                                                                      +
                                                                      *fresh47
                                                                          as
                                                                          libc::c_int)
                                                                     as
                                                                     libc::c_ulong)
                                    as lzo_uint as lzo_uint
                        }
                        m_pos = op.offset(-(1 as libc::c_int as isize));
                        m_pos =
                            m_pos.offset(-((*(ip as *const libc::c_void as
                                                  *const lzo_memops_TU2) as
                                                libc::c_int >>
                                                2 as libc::c_int) as isize));
                        ip = ip.offset(2 as libc::c_int as isize);
                        current_block = 16593409533420678784;
                    } else if t >= 16 as libc::c_int as libc::c_ulong {
                        m_pos = op;
                        m_pos =
                            m_pos.offset(-(((t &
                                                 8 as libc::c_int as
                                                     libc::c_ulong) <<
                                                11 as libc::c_int) as isize));
                        t &= 7 as libc::c_int as libc::c_ulong;
                        if t == 0 as libc::c_int as libc::c_ulong {
                            while *ip as libc::c_int == 0 as libc::c_int {
                                t =
                                    (t as
                                         libc::c_ulong).wrapping_add(255 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong)
                                        as lzo_uint as lzo_uint;
                                ip = ip.offset(1)
                            }
                            let fresh48 = ip;
                            ip = ip.offset(1);
                            t =
                                (t as
                                     libc::c_ulong).wrapping_add((7 as
                                                                      libc::c_int
                                                                      +
                                                                      *fresh48
                                                                          as
                                                                          libc::c_int)
                                                                     as
                                                                     libc::c_ulong)
                                    as lzo_uint as lzo_uint
                        }
                        m_pos =
                            m_pos.offset(-((*(ip as *const libc::c_void as
                                                  *const lzo_memops_TU2) as
                                                libc::c_int >>
                                                2 as libc::c_int) as isize));
                        ip = ip.offset(2 as libc::c_int as isize);
                        if m_pos == op { break ; }
                        m_pos =
                            m_pos.offset(-(0x4000 as libc::c_int as isize));
                        current_block = 16593409533420678784;
                    } else {
                        m_pos = op.offset(-(1 as libc::c_int as isize));
                        m_pos =
                            m_pos.offset(-((t >> 2 as libc::c_int) as isize));
                        let fresh49 = ip;
                        ip = ip.offset(1);
                        m_pos =
                            m_pos.offset(-(((*fresh49 as libc::c_int) <<
                                                2 as libc::c_int) as isize));
                        let fresh50 = m_pos;
                        m_pos = m_pos.offset(1);
                        let fresh51 = op;
                        op = op.offset(1);
                        *fresh51 = *fresh50;
                        let fresh52 = op;
                        op = op.offset(1);
                        *fresh52 = *m_pos;
                        current_block = 5444525074679853294;
                    }
                    match current_block {
                        5444525074679853294 => { }
                        _ => {
                            if op.wrapping_offset_from(m_pos) as libc::c_long
                                   >= 8 as libc::c_int as libc::c_long {
                                t =
                                    (t as
                                         libc::c_ulong).wrapping_add((3 as
                                                                          libc::c_int
                                                                          -
                                                                          1 as
                                                                              libc::c_int)
                                                                         as
                                                                         libc::c_ulong)
                                        as lzo_uint as lzo_uint;
                                if t >= 8 as libc::c_int as libc::c_ulong {
                                    loop  {
                                        ::std::ptr::write_volatile(op as
                                                                       *mut libc::c_void
                                                                       as
                                                                       *mut lzo_memops_TU8,
                                                                   *(m_pos as
                                                                         *const libc::c_void
                                                                         as
                                                                         *const lzo_memops_TU8));
                                        op =
                                            op.offset(8 as libc::c_int as
                                                          isize);
                                        m_pos =
                                            m_pos.offset(8 as libc::c_int as
                                                             isize);
                                        t =
                                            (t as
                                                 libc::c_ulong).wrapping_sub(8
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong)
                                                as lzo_uint as lzo_uint;
                                        if !(t >=
                                                 8 as libc::c_int as
                                                     libc::c_ulong) {
                                            break ;
                                        }
                                    }
                                }
                                if t >= 4 as libc::c_int as libc::c_ulong {
                                    ::std::ptr::write_volatile(op as
                                                                   *mut libc::c_void
                                                                   as
                                                                   *mut lzo_memops_TU4,
                                                               *(m_pos as
                                                                     *const libc::c_void
                                                                     as
                                                                     *const lzo_memops_TU4));
                                    op = op.offset(4 as libc::c_int as isize);
                                    m_pos =
                                        m_pos.offset(4 as libc::c_int as
                                                         isize);
                                    t =
                                        (t as
                                             libc::c_ulong).wrapping_sub(4 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong)
                                            as lzo_uint as lzo_uint
                                }
                                if t > 0 as libc::c_int as libc::c_ulong {
                                    let fresh53 = op;
                                    op = op.offset(1);
                                    *fresh53 =
                                        *m_pos.offset(0 as libc::c_int as
                                                          isize);
                                    if t > 1 as libc::c_int as libc::c_ulong {
                                        let fresh54 = op;
                                        op = op.offset(1);
                                        *fresh54 =
                                            *m_pos.offset(1 as libc::c_int as
                                                              isize);
                                        if t >
                                               2 as libc::c_int as
                                                   libc::c_ulong {
                                            let fresh55 = op;
                                            op = op.offset(1);
                                            *fresh55 =
                                                *m_pos.offset(2 as libc::c_int
                                                                  as isize)
                                        }
                                    }
                                }
                                current_block = 5444525074679853294;
                            } else { current_block = 11354191643585990046; }
                        }
                    }
                }
                match current_block {
                    5444525074679853294 => { }
                    _ => {
                        let fresh56 = m_pos;
                        m_pos = m_pos.offset(1);
                        let fresh57 = op;
                        op = op.offset(1);
                        *fresh57 = *fresh56;
                        let fresh58 = m_pos;
                        m_pos = m_pos.offset(1);
                        let fresh59 = op;
                        op = op.offset(1);
                        *fresh59 = *fresh58;
                        loop  {
                            let fresh60 = m_pos;
                            m_pos = m_pos.offset(1);
                            let fresh61 = op;
                            op = op.offset(1);
                            *fresh61 = *fresh60;
                            t = t.wrapping_sub(1);
                            if !(t > 0 as libc::c_int as libc::c_ulong) {
                                break ;
                            }
                        }
                    }
                }
            }
            _ => { }
        }
        t =
            (*ip.offset(-(2 as libc::c_int) as isize) as libc::c_int &
                 3 as libc::c_int) as lzo_uint;
        if t == 0 as libc::c_int as libc::c_ulong {
            current_block = 4956146061682418353;
        } else { current_block = 16974974966130203269; }
    }
    *out_len = op.wrapping_offset_from(out) as libc::c_long as lzo_uint;
    return if ip == ip_end {
               0 as libc::c_int
           } else if ip < ip_end {
               -(8 as libc::c_int)
           } else { -(4 as libc::c_int) };
}
/* safe decompression with overrun testing */
#[no_mangle]
pub unsafe extern "C" fn lzo1x_decompress_safe(mut in_0: *const libc::c_uchar,
                                               mut in_len: lzo_uint,
                                               mut out: *mut libc::c_uchar,
                                               mut out_len: *mut lzo_uint,
                                               mut wrkmem: *mut libc::c_void)
 -> libc::c_int {
    let mut current_block: u64;
    let mut op: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ip: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut t: lzo_uint = 0;
    let mut m_pos: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let ip_end: *const libc::c_uchar = in_0.offset(in_len as isize);
    let op_end: *mut libc::c_uchar = out.offset(*out_len as isize);
    *out_len = 0 as libc::c_int as lzo_uint;
    op = out;
    ip = in_0;
    if !((ip_end.wrapping_offset_from(ip) as libc::c_long as lzo_uint) <
             1 as libc::c_int as lzo_uint) {
        if *ip as libc::c_int > 17 as libc::c_int {
            let fresh69 = ip;
            ip = ip.offset(1);
            t = (*fresh69 as libc::c_int - 17 as libc::c_int) as lzo_uint;
            if t < 4 as libc::c_int as libc::c_ulong {
                current_block = 10601179871800211547;
            } else if (op_end.wrapping_offset_from(op) as libc::c_long as
                           lzo_uint) < t {
                current_block = 262178057513751245;
            } else if (ip_end.wrapping_offset_from(ip) as libc::c_long as
                           lzo_uint) <
                          t.wrapping_add(3 as libc::c_int as libc::c_ulong) {
                current_block = 10472394685181188800;
            } else {
                loop  {
                    let fresh70 = ip;
                    ip = ip.offset(1);
                    let fresh71 = op;
                    op = op.offset(1);
                    *fresh71 = *fresh70;
                    t = t.wrapping_sub(1);
                    if !(t > 0 as libc::c_int as libc::c_ulong) { break ; }
                }
                current_block = 3938046190763135880;
            }
        } else { current_block = 5689001924483802034; }
        match current_block {
            10472394685181188800 => { }
            _ => {
                's_92:
                    loop  {
                        match current_block {
                            262178057513751245 => {
                                *out_len =
                                    op.wrapping_offset_from(out) as
                                        libc::c_long as lzo_uint;
                                return -(5 as libc::c_int)
                            }
                            3938046190763135880 => {
                                let fresh80 = ip;
                                ip = ip.offset(1);
                                t = *fresh80 as lzo_uint;
                                if t >= 16 as libc::c_int as libc::c_ulong {
                                    current_block = 12490644442199436808;
                                } else {
                                    m_pos =
                                        op.offset(-((1 as libc::c_int +
                                                         0x800 as libc::c_int)
                                                        as isize));
                                    m_pos =
                                        m_pos.offset(-((t >> 2 as libc::c_int)
                                                           as isize));
                                    let fresh81 = ip;
                                    ip = ip.offset(1);
                                    m_pos =
                                        m_pos.offset(-(((*fresh81 as
                                                             libc::c_int) <<
                                                            2 as libc::c_int)
                                                           as isize));
                                    if (m_pos as libc::c_ulong) <
                                           out as libc::c_ulong ||
                                           m_pos as libc::c_ulong >=
                                               op as libc::c_ulong {
                                        current_block = 7071516361779662619;
                                        break ;
                                    }
                                    if (op_end.wrapping_offset_from(op) as
                                            libc::c_long as lzo_uint) <
                                           3 as libc::c_int as lzo_uint {
                                        current_block = 262178057513751245;
                                        continue ;
                                    }
                                    let fresh82 = m_pos;
                                    m_pos = m_pos.offset(1);
                                    let fresh83 = op;
                                    op = op.offset(1);
                                    *fresh83 = *fresh82;
                                    let fresh84 = m_pos;
                                    m_pos = m_pos.offset(1);
                                    let fresh85 = op;
                                    op = op.offset(1);
                                    *fresh85 = *fresh84;
                                    let fresh86 = op;
                                    op = op.offset(1);
                                    *fresh86 = *m_pos;
                                    current_block = 997266168469697404;
                                }
                            }
                            5689001924483802034 => {
                                if (ip_end.wrapping_offset_from(ip) as
                                        libc::c_long as lzo_uint) <
                                       3 as libc::c_int as lzo_uint {
                                    current_block = 10472394685181188800;
                                    break ;
                                }
                                let fresh72 = ip;
                                ip = ip.offset(1);
                                t = *fresh72 as lzo_uint;
                                if t >= 16 as libc::c_int as libc::c_ulong {
                                    current_block = 12490644442199436808;
                                } else {
                                    if t == 0 as libc::c_int as libc::c_ulong
                                       {
                                        while *ip as libc::c_int ==
                                                  0 as libc::c_int {
                                            t =
                                                (t as
                                                     libc::c_ulong).wrapping_add(255
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong)
                                                    as lzo_uint as lzo_uint;
                                            ip = ip.offset(1);
                                            if t >
                                                   (0 as libc::c_int as
                                                        lzo_uint).wrapping_sub(511
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_ulong)
                                               {
                                                current_block =
                                                    10472394685181188800;
                                                break 's_92 ;
                                            }
                                            if (ip_end.wrapping_offset_from(ip)
                                                    as libc::c_long as
                                                    lzo_uint) <
                                                   1 as libc::c_int as
                                                       lzo_uint {
                                                current_block =
                                                    10472394685181188800;
                                                break 's_92 ;
                                            }
                                        }
                                        let fresh73 = ip;
                                        ip = ip.offset(1);
                                        t =
                                            (t as
                                                 libc::c_ulong).wrapping_add((15
                                                                                  as
                                                                                  libc::c_int
                                                                                  +
                                                                                  *fresh73
                                                                                      as
                                                                                      libc::c_int)
                                                                                 as
                                                                                 libc::c_ulong)
                                                as lzo_uint as lzo_uint
                                    }
                                    if (op_end.wrapping_offset_from(op) as
                                            libc::c_long as lzo_uint) <
                                           t.wrapping_add(3 as libc::c_int as
                                                              libc::c_ulong) {
                                        current_block = 262178057513751245;
                                        continue ;
                                    }
                                    if (ip_end.wrapping_offset_from(ip) as
                                            libc::c_long as lzo_uint) <
                                           t.wrapping_add(6 as libc::c_int as
                                                              libc::c_ulong) {
                                        current_block = 10472394685181188800;
                                        break ;
                                    }
                                    t =
                                        (t as
                                             libc::c_ulong).wrapping_add(3 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong)
                                            as lzo_uint as lzo_uint;
                                    if t >= 8 as libc::c_int as libc::c_ulong
                                       {
                                        loop  {
                                            ::std::ptr::write_volatile(op as
                                                                           *mut libc::c_void
                                                                           as
                                                                           *mut lzo_memops_TU8,
                                                                       *(ip as
                                                                             *const libc::c_void
                                                                             as
                                                                             *const lzo_memops_TU8));
                                            op =
                                                op.offset(8 as libc::c_int as
                                                              isize);
                                            ip =
                                                ip.offset(8 as libc::c_int as
                                                              isize);
                                            t =
                                                (t as
                                                     libc::c_ulong).wrapping_sub(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong)
                                                    as lzo_uint as lzo_uint;
                                            if !(t >=
                                                     8 as libc::c_int as
                                                         libc::c_ulong) {
                                                break ;
                                            }
                                        }
                                    }
                                    if t >= 4 as libc::c_int as libc::c_ulong
                                       {
                                        ::std::ptr::write_volatile(op as
                                                                       *mut libc::c_void
                                                                       as
                                                                       *mut lzo_memops_TU4,
                                                                   *(ip as
                                                                         *const libc::c_void
                                                                         as
                                                                         *const lzo_memops_TU4));
                                        op =
                                            op.offset(4 as libc::c_int as
                                                          isize);
                                        ip =
                                            ip.offset(4 as libc::c_int as
                                                          isize);
                                        t =
                                            (t as
                                                 libc::c_ulong).wrapping_sub(4
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong)
                                                as lzo_uint as lzo_uint
                                    }
                                    if t > 0 as libc::c_int as libc::c_ulong {
                                        let fresh74 = ip;
                                        ip = ip.offset(1);
                                        let fresh75 = op;
                                        op = op.offset(1);
                                        *fresh75 = *fresh74;
                                        if t >
                                               1 as libc::c_int as
                                                   libc::c_ulong {
                                            let fresh76 = ip;
                                            ip = ip.offset(1);
                                            let fresh77 = op;
                                            op = op.offset(1);
                                            *fresh77 = *fresh76;
                                            if t >
                                                   2 as libc::c_int as
                                                       libc::c_ulong {
                                                let fresh78 = ip;
                                                ip = ip.offset(1);
                                                let fresh79 = op;
                                                op = op.offset(1);
                                                *fresh79 = *fresh78
                                            }
                                        }
                                    }
                                    current_block = 3938046190763135880;
                                    continue ;
                                }
                            }
                            _ => {
                                if (op_end.wrapping_offset_from(op) as
                                        libc::c_long as lzo_uint) < t {
                                    current_block = 262178057513751245;
                                    continue ;
                                }
                                if (ip_end.wrapping_offset_from(ip) as
                                        libc::c_long as lzo_uint) <
                                       t.wrapping_add(3 as libc::c_int as
                                                          libc::c_ulong) {
                                    current_block = 10472394685181188800;
                                    break ;
                                }
                                let fresh103 = ip;
                                ip = ip.offset(1);
                                let fresh104 = op;
                                op = op.offset(1);
                                *fresh104 = *fresh103;
                                if t > 1 as libc::c_int as libc::c_ulong {
                                    let fresh105 = ip;
                                    ip = ip.offset(1);
                                    let fresh106 = op;
                                    op = op.offset(1);
                                    *fresh106 = *fresh105;
                                    if t > 2 as libc::c_int as libc::c_ulong {
                                        let fresh107 = ip;
                                        ip = ip.offset(1);
                                        let fresh108 = op;
                                        op = op.offset(1);
                                        *fresh108 = *fresh107
                                    }
                                }
                                let fresh109 = ip;
                                ip = ip.offset(1);
                                t = *fresh109 as lzo_uint;
                                current_block = 12490644442199436808;
                            }
                        }
                        match current_block {
                            12490644442199436808 => {
                                if t >= 64 as libc::c_int as libc::c_ulong {
                                    m_pos =
                                        op.offset(-(1 as libc::c_int as
                                                        isize));
                                    m_pos =
                                        m_pos.offset(-((t >> 2 as libc::c_int
                                                            &
                                                            7 as libc::c_int
                                                                as
                                                                libc::c_ulong)
                                                           as isize));
                                    let fresh87 = ip;
                                    ip = ip.offset(1);
                                    m_pos =
                                        m_pos.offset(-(((*fresh87 as
                                                             libc::c_int) <<
                                                            3 as libc::c_int)
                                                           as isize));
                                    t =
                                        (t >>
                                             5 as
                                                 libc::c_int).wrapping_sub(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_ulong);
                                    if (m_pos as libc::c_ulong) <
                                           out as libc::c_ulong ||
                                           m_pos as libc::c_ulong >=
                                               op as libc::c_ulong {
                                        current_block = 7071516361779662619;
                                        break ;
                                    }
                                    if (op_end.wrapping_offset_from(op) as
                                            libc::c_long as lzo_uint) <
                                           t.wrapping_add(3 as libc::c_int as
                                                              libc::c_ulong).wrapping_sub(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_ulong)
                                       {
                                        current_block = 262178057513751245;
                                        continue ;
                                    }
                                    current_block = 16504611031939317935;
                                } else {
                                    if t >= 32 as libc::c_int as libc::c_ulong
                                       {
                                        t &=
                                            31 as libc::c_int as
                                                libc::c_ulong;
                                        if t ==
                                               0 as libc::c_int as
                                                   libc::c_ulong {
                                            while *ip as libc::c_int ==
                                                      0 as libc::c_int {
                                                t =
                                                    (t as
                                                         libc::c_ulong).wrapping_add(255
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong)
                                                        as lzo_uint as
                                                        lzo_uint;
                                                ip = ip.offset(1);
                                                if t >
                                                       (0 as libc::c_int as
                                                            lzo_uint).wrapping_sub(511
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_ulong)
                                                   {
                                                    current_block =
                                                        262178057513751245;
                                                    continue 's_92 ;
                                                }
                                                if (ip_end.wrapping_offset_from(ip)
                                                        as libc::c_long as
                                                        lzo_uint) <
                                                       1 as libc::c_int as
                                                           lzo_uint {
                                                    current_block =
                                                        10472394685181188800;
                                                    break 's_92 ;
                                                }
                                            }
                                            let fresh88 = ip;
                                            ip = ip.offset(1);
                                            t =
                                                (t as
                                                     libc::c_ulong).wrapping_add((31
                                                                                      as
                                                                                      libc::c_int
                                                                                      +
                                                                                      *fresh88
                                                                                          as
                                                                                          libc::c_int)
                                                                                     as
                                                                                     libc::c_ulong)
                                                    as lzo_uint as lzo_uint;
                                            if (ip_end.wrapping_offset_from(ip)
                                                    as libc::c_long as
                                                    lzo_uint) <
                                                   2 as libc::c_int as
                                                       lzo_uint {
                                                current_block =
                                                    10472394685181188800;
                                                break ;
                                            }
                                        }
                                        m_pos =
                                            op.offset(-(1 as libc::c_int as
                                                            isize));
                                        m_pos =
                                            m_pos.offset(-((*(ip as
                                                                  *const libc::c_void
                                                                  as
                                                                  *const lzo_memops_TU2)
                                                                as libc::c_int
                                                                >>
                                                                2 as
                                                                    libc::c_int)
                                                               as isize));
                                        ip =
                                            ip.offset(2 as libc::c_int as
                                                          isize);
                                        current_block = 14913924298693586572;
                                    } else if t >=
                                                  16 as libc::c_int as
                                                      libc::c_ulong {
                                        m_pos = op;
                                        m_pos =
                                            m_pos.offset(-(((t &
                                                                 8 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                                                                <<
                                                                11 as
                                                                    libc::c_int)
                                                               as isize));
                                        t &=
                                            7 as libc::c_int as libc::c_ulong;
                                        if t ==
                                               0 as libc::c_int as
                                                   libc::c_ulong {
                                            while *ip as libc::c_int ==
                                                      0 as libc::c_int {
                                                t =
                                                    (t as
                                                         libc::c_ulong).wrapping_add(255
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong)
                                                        as lzo_uint as
                                                        lzo_uint;
                                                ip = ip.offset(1);
                                                if t >
                                                       (0 as libc::c_int as
                                                            lzo_uint).wrapping_sub(511
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_ulong)
                                                   {
                                                    current_block =
                                                        262178057513751245;
                                                    continue 's_92 ;
                                                }
                                                if (ip_end.wrapping_offset_from(ip)
                                                        as libc::c_long as
                                                        lzo_uint) <
                                                       1 as libc::c_int as
                                                           lzo_uint {
                                                    current_block =
                                                        10472394685181188800;
                                                    break 's_92 ;
                                                }
                                            }
                                            let fresh89 = ip;
                                            ip = ip.offset(1);
                                            t =
                                                (t as
                                                     libc::c_ulong).wrapping_add((7
                                                                                      as
                                                                                      libc::c_int
                                                                                      +
                                                                                      *fresh89
                                                                                          as
                                                                                          libc::c_int)
                                                                                     as
                                                                                     libc::c_ulong)
                                                    as lzo_uint as lzo_uint;
                                            if (ip_end.wrapping_offset_from(ip)
                                                    as libc::c_long as
                                                    lzo_uint) <
                                                   2 as libc::c_int as
                                                       lzo_uint {
                                                current_block =
                                                    10472394685181188800;
                                                break ;
                                            }
                                        }
                                        m_pos =
                                            m_pos.offset(-((*(ip as
                                                                  *const libc::c_void
                                                                  as
                                                                  *const lzo_memops_TU2)
                                                                as libc::c_int
                                                                >>
                                                                2 as
                                                                    libc::c_int)
                                                               as isize));
                                        ip =
                                            ip.offset(2 as libc::c_int as
                                                          isize);
                                        if m_pos == op {
                                            *out_len =
                                                op.wrapping_offset_from(out)
                                                    as libc::c_long as
                                                    lzo_uint;
                                            return if ip == ip_end {
                                                       0 as libc::c_int
                                                   } else if ip < ip_end {
                                                       -(8 as libc::c_int)
                                                   } else {
                                                       -(4 as libc::c_int)
                                                   }
                                        } else {
                                            m_pos =
                                                m_pos.offset(-(0x4000 as
                                                                   libc::c_int
                                                                   as isize))
                                        }
                                        current_block = 14913924298693586572;
                                    } else {
                                        m_pos =
                                            op.offset(-(1 as libc::c_int as
                                                            isize));
                                        m_pos =
                                            m_pos.offset(-((t >>
                                                                2 as
                                                                    libc::c_int)
                                                               as isize));
                                        let fresh90 = ip;
                                        ip = ip.offset(1);
                                        m_pos =
                                            m_pos.offset(-(((*fresh90 as
                                                                 libc::c_int)
                                                                <<
                                                                2 as
                                                                    libc::c_int)
                                                               as isize));
                                        if (m_pos as libc::c_ulong) <
                                               out as libc::c_ulong ||
                                               m_pos as libc::c_ulong >=
                                                   op as libc::c_ulong {
                                            current_block =
                                                7071516361779662619;
                                            break ;
                                        }
                                        if (op_end.wrapping_offset_from(op) as
                                                libc::c_long as lzo_uint) <
                                               2 as libc::c_int as lzo_uint {
                                            current_block =
                                                262178057513751245;
                                            continue ;
                                        }
                                        let fresh91 = m_pos;
                                        m_pos = m_pos.offset(1);
                                        let fresh92 = op;
                                        op = op.offset(1);
                                        *fresh92 = *fresh91;
                                        let fresh93 = op;
                                        op = op.offset(1);
                                        *fresh93 = *m_pos;
                                        current_block = 997266168469697404;
                                    }
                                    match current_block {
                                        997266168469697404 => { }
                                        _ => {
                                            if (m_pos as libc::c_ulong) <
                                                   out as libc::c_ulong ||
                                                   m_pos as libc::c_ulong >=
                                                       op as libc::c_ulong {
                                                current_block =
                                                    7071516361779662619;
                                                break ;
                                            }
                                            if (op_end.wrapping_offset_from(op)
                                                    as libc::c_long as
                                                    lzo_uint) <
                                                   t.wrapping_add(3 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong).wrapping_sub(1
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_ulong)
                                               {
                                                current_block =
                                                    262178057513751245;
                                                continue ;
                                            }
                                            if op.wrapping_offset_from(m_pos)
                                                   as libc::c_long >=
                                                   8 as libc::c_int as
                                                       libc::c_long {
                                                t =
                                                    (t as
                                                         libc::c_ulong).wrapping_add((3
                                                                                          as
                                                                                          libc::c_int
                                                                                          -
                                                                                          1
                                                                                              as
                                                                                              libc::c_int)
                                                                                         as
                                                                                         libc::c_ulong)
                                                        as lzo_uint as
                                                        lzo_uint;
                                                if t >=
                                                       8 as libc::c_int as
                                                           libc::c_ulong {
                                                    loop  {
                                                        ::std::ptr::write_volatile(op
                                                                                       as
                                                                                       *mut libc::c_void
                                                                                       as
                                                                                       *mut lzo_memops_TU8,
                                                                                   *(m_pos
                                                                                         as
                                                                                         *const libc::c_void
                                                                                         as
                                                                                         *const lzo_memops_TU8));
                                                        op =
                                                            op.offset(8 as
                                                                          libc::c_int
                                                                          as
                                                                          isize);
                                                        m_pos =
                                                            m_pos.offset(8 as
                                                                             libc::c_int
                                                                             as
                                                                             isize);
                                                        t =
                                                            (t as
                                                                 libc::c_ulong).wrapping_sub(8
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong)
                                                                as lzo_uint as
                                                                lzo_uint;
                                                        if !(t >=
                                                                 8 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                                                           {
                                                            break ;
                                                        }
                                                    }
                                                }
                                                if t >=
                                                       4 as libc::c_int as
                                                           libc::c_ulong {
                                                    ::std::ptr::write_volatile(op
                                                                                   as
                                                                                   *mut libc::c_void
                                                                                   as
                                                                                   *mut lzo_memops_TU4,
                                                                               *(m_pos
                                                                                     as
                                                                                     *const libc::c_void
                                                                                     as
                                                                                     *const lzo_memops_TU4));
                                                    op =
                                                        op.offset(4 as
                                                                      libc::c_int
                                                                      as
                                                                      isize);
                                                    m_pos =
                                                        m_pos.offset(4 as
                                                                         libc::c_int
                                                                         as
                                                                         isize);
                                                    t =
                                                        (t as
                                                             libc::c_ulong).wrapping_sub(4
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong)
                                                            as lzo_uint as
                                                            lzo_uint
                                                }
                                                if t >
                                                       0 as libc::c_int as
                                                           libc::c_ulong {
                                                    let fresh94 = op;
                                                    op = op.offset(1);
                                                    *fresh94 =
                                                        *m_pos.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize);
                                                    if t >
                                                           1 as libc::c_int as
                                                               libc::c_ulong {
                                                        let fresh95 = op;
                                                        op = op.offset(1);
                                                        *fresh95 =
                                                            *m_pos.offset(1 as
                                                                              libc::c_int
                                                                              as
                                                                              isize);
                                                        if t >
                                                               2 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong
                                                           {
                                                            let fresh96 = op;
                                                            op = op.offset(1);
                                                            *fresh96 =
                                                                *m_pos.offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)
                                                        }
                                                    }
                                                }
                                                current_block =
                                                    997266168469697404;
                                            } else {
                                                current_block =
                                                    16504611031939317935;
                                            }
                                        }
                                    }
                                }
                                match current_block {
                                    997266168469697404 => { }
                                    _ => {
                                        let fresh97 = m_pos;
                                        m_pos = m_pos.offset(1);
                                        let fresh98 = op;
                                        op = op.offset(1);
                                        *fresh98 = *fresh97;
                                        let fresh99 = m_pos;
                                        m_pos = m_pos.offset(1);
                                        let fresh100 = op;
                                        op = op.offset(1);
                                        *fresh100 = *fresh99;
                                        loop  {
                                            let fresh101 = m_pos;
                                            m_pos = m_pos.offset(1);
                                            let fresh102 = op;
                                            op = op.offset(1);
                                            *fresh102 = *fresh101;
                                            t = t.wrapping_sub(1);
                                            if !(t >
                                                     0 as libc::c_int as
                                                         libc::c_ulong) {
                                                break ;
                                            }
                                        }
                                    }
                                }
                            }
                            _ => { }
                        }
                        t =
                            (*ip.offset(-(2 as libc::c_int) as isize) as
                                 libc::c_int & 3 as libc::c_int) as lzo_uint;
                        if t == 0 as libc::c_int as libc::c_ulong {
                            current_block = 5689001924483802034;
                        } else { current_block = 10601179871800211547; }
                    }
                match current_block {
                    10472394685181188800 => { }
                    _ => {
                        *out_len =
                            op.wrapping_offset_from(out) as libc::c_long as
                                lzo_uint;
                        return -(6 as libc::c_int)
                    }
                }
            }
        }
    }
    *out_len = op.wrapping_offset_from(out) as libc::c_long as lzo_uint;
    return -(4 as libc::c_int);
}
unsafe fn main_0() -> libc::c_int { return 0 as libc::c_int; }
#[main]
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
/* **** End of minilzo.c *****/
