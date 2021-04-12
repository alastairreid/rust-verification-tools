// Copyright 2021 The Rust verification tools authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(repr_simd)]

// This type is identical to the type declared in comp_arch.
// It is much less convenient to use in implementations so we
// actually use the templated definitions simdN<T> below and
// can transmute arguments/results as needed.
#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
#[repr(simd)]
pub struct __m128i(i64, i64);

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
#[repr(simd)]
pub struct simd2<T>(T, T);

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
#[repr(simd)]
pub struct simd4<T>(T, T, T, T);

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
#[repr(simd)]
pub struct simd8<T>(T, T, T, T, T, T, T, T);

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
#[repr(simd)]
pub struct simd16<T>(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T);

fn map8_vs_v<F>(op: F, a: simd8<u16>, b: u16) -> simd8<u16>
    where F: Fn(u16, u16) -> u16
{
    let r0  = op(a.0,  b);
    let r1  = op(a.1,  b);
    let r2  = op(a.2,  b);
    let r3  = op(a.3,  b);
    let r4  = op(a.4,  b);
    let r5  = op(a.5,  b);
    let r6  = op(a.6,  b);
    let r7  = op(a.7,  b);
    simd8(r0, r1, r2, r3, r4, r5, r6, r7)
}

fn map8_vv_v<F>(op: F, a: simd8<u16>, b: simd8<u16>) -> simd8<u16>
    where F: Fn(u16, u16) -> u16
{
    let r0  = op(a.0,  b.0);
    let r1  = op(a.1,  b.1);
    let r2  = op(a.2,  b.2);
    let r3  = op(a.3,  b.3);
    let r4  = op(a.4,  b.4);
    let r5  = op(a.5,  b.5);
    let r6  = op(a.6,  b.6);
    let r7  = op(a.7,  b.7);
    simd8(r0, r1, r2, r3, r4, r5, r6, r7)
}

fn map16_vv_v<F>(op: F, a: simd16<u8>, b: simd16<u8>) -> simd16<u8>
    where F: Fn(u8, u8) -> u8
{
    let r0  = op(a.0,  b.0);
    let r1  = op(a.1,  b.1);
    let r2  = op(a.2,  b.2);
    let r3  = op(a.3,  b.3);
    let r4  = op(a.4,  b.4);
    let r5  = op(a.5,  b.5);
    let r6  = op(a.6,  b.6);
    let r7  = op(a.7,  b.7);
    let r8  = op(a.8,  b.8);
    let r9  = op(a.9,  b.9);
    let r10 = op(a.10, b.10);
    let r11 = op(a.11, b.11);
    let r12 = op(a.12, b.12);
    let r13 = op(a.13, b.13);
    let r14 = op(a.14, b.14);
    let r15 = op(a.15, b.15);
    simd16(r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15)
}

#[inline]
#[no_mangle]
extern "C" fn llvm_x86_sse2_pcmpeqb_epi8(a: simd16<u8>, b: simd16<u8>) -> simd16<u8> {
    fn op(x: u8, y: u8) -> u8 {
        if x == y { 0xff } else { 0x0 }
    }
    map16_vv_v(op, a, b)
}

#[inline]
#[no_mangle]
extern "C" fn llvm_x86_sse2_pcmpeqw_epi16(a: simd8<u16>, b: simd8<u16>) -> simd8<u16> {
    fn op(x: u16, y: u16) -> u16 {
        if x == y { 0xffff } else { 0x0 }
    }
    map8_vv_v(op, a, b)
}

#[inline]
#[no_mangle]
extern "C" fn llvm_x86_sse2_psrli_w(a: simd8<u16>, imm8: i32) -> simd8<u16> {
    let imm8 = imm8 as u8;
    fn op(x: u16, imm8: u8) -> u16 {
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
    simd8(r0, r1, r2, r3, r4, r5, r6, r7)
}

#[inline]
#[no_mangle]
extern "C" fn llvm_x86_sse2_pmovmskb_128(a: simd16<u8>) -> i32 {
    fn op(x: u8) -> i32 {
        ((x >> 7) & 1) as i32
    }
    let r0  = op(a.0);
    let r1  = op(a.1);
    let r2  = op(a.2);
    let r3  = op(a.3);
    let r4  = op(a.4);
    let r5  = op(a.5);
    let r6  = op(a.6);
    let r7  = op(a.7);
    let r8  = op(a.8);
    let r9  = op(a.9);
    let r10 = op(a.10);
    let r11 = op(a.11);
    let r12 = op(a.12);
    let r13 = op(a.13);
    let r14 = op(a.14);
    let r15 = op(a.15);
    let r = (r0  << 0)  | (r1  << 1)  | (r2  << 2)  | (r3  << 3)
          | (r4  << 4)  | (r5  << 5)  | (r6  << 6)  | (r7  << 7)
          | (r8  << 8)  | (r9  << 9)  | (r10 << 10) | (r11 << 11)
          | (r12 << 12) | (r13 << 13) | (r14 << 14) | (r15 << 15);
    r
}
