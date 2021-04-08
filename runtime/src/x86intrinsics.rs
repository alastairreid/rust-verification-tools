// Copyright 2021 The Propverify authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(repr_simd)]

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
#[repr(simd)]
pub struct __m128i(i64, i64);

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
#[repr(simd)]
pub struct u64x2(u64, u64);

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
#[repr(simd)]
pub struct u32x4(u32, u32, u32, u32);

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
#[repr(simd)]
pub struct u16x8(u16, u16, u16, u16, u16, u16, u16, u16);

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
#[repr(simd)]
pub struct u8x16(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8);

#[inline]
#[no_mangle]
pub fn llvm_x86_sse2_pcmpeqb_epi8(a: u8x16, b: u8x16) -> u8x16 {
    let r0  = if a.0  == b.0  { 0xff } else { 0x0 };
    let r1  = if a.1  == b.1  { 0xff } else { 0x0 };
    let r2  = if a.2  == b.2  { 0xff } else { 0x0 };
    let r3  = if a.3  == b.3  { 0xff } else { 0x0 };
    let r4  = if a.4  == b.4  { 0xff } else { 0x0 };
    let r5  = if a.5  == b.5  { 0xff } else { 0x0 };
    let r6  = if a.6  == b.6  { 0xff } else { 0x0 };
    let r7  = if a.7  == b.7  { 0xff } else { 0x0 };
    let r8  = if a.8  == b.8  { 0xff } else { 0x0 };
    let r9  = if a.9  == b.9  { 0xff } else { 0x0 };
    let r10 = if a.10 == b.10 { 0xff } else { 0x0 };
    let r11 = if a.11 == b.11 { 0xff } else { 0x0 };
    let r12 = if a.12 == b.12 { 0xff } else { 0x0 };
    let r13 = if a.13 == b.13 { 0xff } else { 0x0 };
    let r14 = if a.14 == b.14 { 0xff } else { 0x0 };
    let r15 = if a.15 == b.15 { 0xff } else { 0x0 };
    u8x16(r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15)
}

#[inline]
#[no_mangle]
pub fn llvm_x86_sse2_pcmpeqw_epi16(a: u16x8, b: u16x8) -> u16x8 {
    let r0 = if a.0  == b.0 { 0xff } else { 0x0 };
    let r1 = if a.1  == b.1 { 0xff } else { 0x0 };
    let r2 = if a.2  == b.2 { 0xff } else { 0x0 };
    let r3 = if a.3  == b.3 { 0xff } else { 0x0 };
    let r4 = if a.4  == b.4 { 0xff } else { 0x0 };
    let r5 = if a.5  == b.5 { 0xff } else { 0x0 };
    let r6 = if a.6  == b.6 { 0xff } else { 0x0 };
    let r7 = if a.7  == b.7 { 0xff } else { 0x0 };
    u16x8(r0, r1, r2, r3, r4, r5, r6, r7)
}


#[inline]
#[no_mangle]
pub fn llvm_x86_sse2_psrli_w(a: u16x8, imm8: i32) -> u16x8 {
    let imm8: i32 = imm8 & 0xff;
    fn op(x: u16, imm8: i32) -> u16 {
        if imm8 > 15 { 0 } else { x >> imm8 }
    }
    let r0  = op(a.0, imm8);
    let r1  = op(a.1, imm8);
    let r2  = op(a.2, imm8);
    let r3  = op(a.3, imm8);
    let r4  = op(a.4, imm8);
    let r5  = op(a.5, imm8);
    let r6  = op(a.6, imm8);
    let r7  = op(a.7, imm8);
    u16x8(r0, r1, r2, r3, r4, r5, r6, r7)
}


#[inline]
#[no_mangle]
pub fn llvm_x86_sse2_pmovmskb_128(x: u8x16) -> i32 {
    let r0  = ((x.0 >> 7) & 1) as i32;
    let r1  = ((x.1 >> 7) & 1) as i32;
    let r2  = ((x.2 >> 7) & 1) as i32;
    let r3  = ((x.3 >> 7) & 1) as i32;
    let r4  = ((x.4 >> 7) & 1) as i32;
    let r5  = ((x.5 >> 7) & 1) as i32;
    let r6  = ((x.6 >> 7) & 1) as i32;
    let r7  = ((x.7 >> 7) & 1) as i32;
    let r8  = ((x.8 >> 7) & 1) as i32;
    let r9  = ((x.9 >> 7) & 1) as i32;
    let r10 = ((x.10 >> 7) & 1) as i32;
    let r11 = ((x.11 >> 7) & 1) as i32;
    let r12 = ((x.12 >> 7) & 1) as i32;
    let r13 = ((x.13 >> 7) & 1) as i32;
    let r14 = ((x.14 >> 7) & 1) as i32;
    let r15 = ((x.15 >> 7) & 1) as i32;
    let r = (r0  << 0)  | (r1  << 1)  | (r2  << 2)  | (r3  << 3)
          | (r4  << 4)  | (r5  << 5)  | (r6  << 6)  | (r7  << 7)
          | (r8  << 8)  | (r9  << 9)  | (r10 << 10) | (r11 << 11)
          | (r12 << 12) | (r13 << 13) | (r14 << 14) | (r15 << 15);
    r
}



