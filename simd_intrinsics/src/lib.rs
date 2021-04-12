// Copyright 2021 The Rust verification tools authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(repr_simd)]
#![allow(unused)]

/// Utilities that capture the common structures in SIMD operations
/// using 2nd order functions
mod utils {

    #[derive(Copy, Clone, Debug)]
    #[allow(non_camel_case_types)]
    #[repr(simd)]
    pub struct simd2<T>(pub T, pub T);

    #[derive(Copy, Clone, Debug)]
    #[allow(non_camel_case_types)]
    #[repr(simd)]
    pub struct simd4<T>(pub T, pub T, pub T, pub T);

    #[derive(Copy, Clone, Debug)]
    #[allow(non_camel_case_types)]
    #[repr(simd)]
    pub struct simd8<T>(pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub T);

    #[derive(Copy, Clone, Debug)]
    #[allow(non_camel_case_types)]
    #[repr(simd)]
    pub struct simd16<T>(pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub T, pub T);

    // lift a binary operation over a vector and a scalar (replicating the scalar)
    pub fn lift8_vs_v<F, A, B: Copy, R>(f: F, a: simd8<A>, b: B) -> simd8<R>
        where F: Fn(A, B) -> R
    {
        let r0  = f(a.0,  b);
        let r1  = f(a.1,  b);
        let r2  = f(a.2,  b);
        let r3  = f(a.3,  b);
        let r4  = f(a.4,  b);
        let r5  = f(a.5,  b);
        let r6  = f(a.6,  b);
        let r7  = f(a.7,  b);
        simd8(r0, r1, r2, r3, r4, r5, r6, r7)
    }

    // lift a binary operation over two vectors
    pub fn lift8_vv_v<F, A, B, R>(f: F, a: simd8<A>, b: simd8<B>) -> simd8<R>
        where F: Fn(A, B) -> R
    {
        let r0  = f(a.0,  b.0);
        let r1  = f(a.1,  b.1);
        let r2  = f(a.2,  b.2);
        let r3  = f(a.3,  b.3);
        let r4  = f(a.4,  b.4);
        let r5  = f(a.5,  b.5);
        let r6  = f(a.6,  b.6);
        let r7  = f(a.7,  b.7);
        simd8(r0, r1, r2, r3, r4, r5, r6, r7)
    }

    // lift a binary operation over two vectors
    pub fn lift16_vv_v<F, A, B, R>(f: F, a: simd16<A>, b: simd16<B>) -> simd16<R>
        where F: Fn(A, B) -> R
    {
        let r0  = f(a.0,  b.0);
        let r1  = f(a.1,  b.1);
        let r2  = f(a.2,  b.2);
        let r3  = f(a.3,  b.3);
        let r4  = f(a.4,  b.4);
        let r5  = f(a.5,  b.5);
        let r6  = f(a.6,  b.6);
        let r7  = f(a.7,  b.7);
        let r8  = f(a.8,  b.8);
        let r9  = f(a.9,  b.9);
        let r10 = f(a.10, b.10);
        let r11 = f(a.11, b.11);
        let r12 = f(a.12, b.12);
        let r13 = f(a.13, b.13);
        let r14 = f(a.14, b.14);
        let r15 = f(a.15, b.15);
        simd16(r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15)
    }

    pub fn reduce2<F, A: Copy>(f: F, a0: A, a1: A) -> A
        where F: Fn(A, A) -> A
    {
        f(a0, a1)
    }

    pub fn reduce4<F, A: Copy>(f: F, a0: A, a1: A, a2: A, a3: A) -> A
        where F: Fn(A, A) -> A
    {
        f(reduce2(&f, a0, a1), reduce2(&f, a2, a3))
    }

    pub fn reduce8<F, A: Copy>(f: F, a0: A, a1: A, a2: A, a3: A, a4: A, a5: A, a6: A, a7: A) -> A
        where F: Fn(A, A) -> A
    {
        f(reduce4(&f, a0, a1, a2, a3), reduce4(&f, a4, a5, a6, a7))
    }

    pub fn reduce16<F, A: Copy>(f: F, a0: A, a1: A, a2: A, a3: A, a4: A, a5: A, a6: A, a7: A, a8: A, a9: A, a10: A, a11: A, a12: A, a13: A, a14: A, a15: A) -> A
        where F: Fn(A, A) -> A
    {
        f(reduce8(&f, a0, a1, a2, a3, a4, a5, a6, a7), reduce8(&f, a8, a9, a10, a11, a12, a13, a14, a15))
    }


    // todo: these reductions may need to use the element number or the size of the reduction
    // (2,4,8,16) as well as the values.

    // map f over vectors then reduce with g
    pub fn lift8_vv_s<F, G, A, B, R: Copy>(f: F, g: G, a: simd8<A>, b: simd8<B>) -> R
        where F: Fn(A, B) -> R,
              G: Fn(R, R) -> R
    {
        let r0  = f(a.0,  b.0);
        let r1  = f(a.1,  b.1);
        let r2  = f(a.2,  b.2);
        let r3  = f(a.3,  b.3);
        let r4  = f(a.4,  b.4);
        let r5  = f(a.5,  b.5);
        let r6  = f(a.6,  b.6);
        let r7  = f(a.7,  b.7);
        reduce8(g, r0, r1, r2, r3, r4, r5, r6, r7)
    }

    // map f over vectors then reduce with g
    pub fn lift16_vv_s<F, G, A, B, R: Copy>(f: F, g: G, a: simd16<A>, b: simd16<B>) -> R
        where F: Fn(A, B) -> R,
              G: Fn(R, R) -> R
    {
        let r0  = f(a.0,  b.0);
        let r1  = f(a.1,  b.1);
        let r2  = f(a.2,  b.2);
        let r3  = f(a.3,  b.3);
        let r4  = f(a.4,  b.4);
        let r5  = f(a.5,  b.5);
        let r6  = f(a.6,  b.6);
        let r7  = f(a.7,  b.7);
        let r8  = f(a.8,  b.8);
        let r9  = f(a.9,  b.9);
        let r10 = f(a.10, b.10);
        let r11 = f(a.11, b.11);
        let r12 = f(a.12, b.12);
        let r13 = f(a.13, b.13);
        let r14 = f(a.14, b.14);
        let r15 = f(a.15, b.15);
        reduce16(g, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15)
    }
}


use utils::*;

#[inline]
#[no_mangle]
extern "C" fn llvm_x86_sse2_pcmpeqb_epi8(a: simd16<u8>, b: simd16<u8>) -> simd16<u8> {
    fn op(x: u8, y: u8) -> u8 {
        if x == y { 0xff } else { 0x0 }
    }
    lift16_vv_v(op, a, b)
}

#[inline]
#[no_mangle]
extern "C" fn llvm_x86_sse2_pcmpeqw_epi16(a: simd8<u16>, b: simd8<u16>) -> simd8<u16> {
    fn op(x: u16, y: u16) -> u16 {
        if x == y { 0xffff } else { 0x0 }
    }
    lift8_vv_v(op, a, b)
}

#[inline]
#[no_mangle]
extern "C" fn llvm_x86_sse2_psrli_w(a: simd8<u16>, imm8: i32) -> simd8<u16> {
    let imm8 = imm8 as u8;
    fn op(x: u16, imm8: u8) -> u16 {
        if imm8 > 15 { 0 } else { x >> imm8 }
    }
    lift8_vs_v(op, a, imm8)
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

/// The above operations use structured types that reflect the underlying
/// operations but they do not match the types of the official Intel
/// intrinsics.
/// This module implements the official operations.
///
/// Since these versions are designed for use with MIR, we don't
/// faff around with the extern "C" annotations used to generate usable LLVM
mod with_intel_official_types {
    #[derive(Copy, Clone, Debug)]
    #[allow(non_camel_case_types)]
    #[repr(simd)]
    pub struct __m128i(i64, i64);

    fn llvm_x86_sse2_pcmpeqb_epi8(a: __m128i, b: __m128i) -> __m128i {
        let a = unsafe { std::mem::transmute::<__m128i, super::simd16<u8>>(a) };
        let b = unsafe { std::mem::transmute::<__m128i, super::simd16<u8>>(b) };
        let r = super::llvm_x86_sse2_pcmpeqb_epi8(a, b);
        unsafe { std::mem::transmute::<super::simd16<u8>, __m128i>(r) }
    }

    fn llvm_x86_sse2_pcmpeqb_epi16(a: __m128i, b: __m128i) -> __m128i {
        let a = unsafe { std::mem::transmute::<__m128i, super::simd8<u16>>(a) };
        let b = unsafe { std::mem::transmute::<__m128i, super::simd8<u16>>(b) };
        let r = super::llvm_x86_sse2_pcmpeqw_epi16(a, b);
        unsafe { std::mem::transmute::<super::simd8<u16>, __m128i>(r) }
    }

    fn llvm_x86_sse2_psrli_w(a: __m128i, imm8: i32) -> __m128i {
        let a = unsafe { std::mem::transmute::<__m128i, super::simd8<u16>>(a) };
        let r = super::llvm_x86_sse2_psrli_w(a, imm8);
        unsafe { std::mem::transmute::<super::simd8<u16>, __m128i>(r) }
    }

    fn llvm_x86_sse2_pmovmskb_128(a: __m128i) -> i32 {
        let a = unsafe { std::mem::transmute::<__m128i, super::simd16<u8>>(a) };
        super::llvm_x86_sse2_pmovmskb_128(a)
    }
}
