// Copyright 2021 The Rust verification tools authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(repr_simd)]
#![allow(improper_ctypes_definitions)]
#![allow(unused)]

/// Utilities that capture the common structures in SIMD operations
/// using 2nd order functions
mod vector {

    /// Implemented by types that support 4-element vectors
    /// Provides methods to construct, destruct and convert vectors to their
    /// native representation.
    pub trait Vector4
        where
            Self: Copy,
            Self::Vec: Copy,
    {
        /// Portable type used to represent vectors
        type Vec;

        /// Machine-dependent type used to represent vectors
        type Machine;

        /// Construct a vector from individual elements
        fn new(x0: Self, x1: Self, x2: Self, x3: Self) -> Self::Vec;

        /// Extract elements from a vector
        fn get0(x: Self::Vec) -> Self;
        fn get1(x: Self::Vec) -> Self;
        fn get2(x: Self::Vec) -> Self;
        fn get3(x: Self::Vec) -> Self;

        /// Convert the machine-dependent type to the portable representation
        fn to_vec(Self::Machine) -> Self::Vec;

        /// Convert the portable representation to the machine-dependent type
        fn from_vec(Self::Vec) -> Self::Machine;
    }

    /// Implemented by types that support 8-element vectors
    /// Provides methods to construct, destruct and convert vectors to their
    /// native representation.
    pub trait Vector8
        where
            Self: Copy,
            Self::Vec: Copy,
    {
        /// Portable type used to represent vectors
        type Vec;

        /// Machine-dependent type used to represent vectors
        type Machine;

        /// Construct a vector from individual elements
        fn new(x0: Self, x1: Self, x2: Self, x3: Self, x4: Self, x5: Self, x6: Self, x7: Self) -> Self::Vec;

        /// Extract elements from a vector
        fn get0(x: Self::Vec) -> Self;
        fn get1(x: Self::Vec) -> Self;
        fn get2(x: Self::Vec) -> Self;
        fn get3(x: Self::Vec) -> Self;
        fn get4(x: Self::Vec) -> Self;
        fn get5(x: Self::Vec) -> Self;
        fn get6(x: Self::Vec) -> Self;
        fn get7(x: Self::Vec) -> Self;

        /// Convert the machine-dependent type to the portable representation
        fn to_vec(Self::Machine) -> Self::Vec;

        /// Convert the portable representation to the machine-dependent type
        fn from_vec(Self::Vec) -> Self::Machine;
    }

    /// Implemented by types that support 16-element vectors
    /// Provides methods to construct, destruct and convert vectors to their
    /// native representation.
    pub trait Vector16
        where
            Self: Copy,
            Self::Vec: Copy,
    {
        /// Portable type used to represent vectors
        type Vec;

        /// Machine-dependent type used to represent vectors
        type Machine;

        /// Construct a vector from individual elements
        fn new(x0: Self, x1: Self, x2: Self, x3: Self, x4: Self, x5: Self, x6: Self, x7: Self,
               x8: Self, x9: Self, x10: Self, x11: Self, x12: Self, x13: Self, x14: Self, x15: Self) -> Self::Vec;

        /// Extract elements from a vector
        fn get0(x: Self::Vec) -> Self;
        fn get1(x: Self::Vec) -> Self;
        fn get2(x: Self::Vec) -> Self;
        fn get3(x: Self::Vec) -> Self;
        fn get4(x: Self::Vec) -> Self;
        fn get5(x: Self::Vec) -> Self;
        fn get6(x: Self::Vec) -> Self;
        fn get7(x: Self::Vec) -> Self;
        fn get8(x: Self::Vec) -> Self;
        fn get9(x: Self::Vec) -> Self;
        fn get10(x: Self::Vec) -> Self;
        fn get11(x: Self::Vec) -> Self;
        fn get12(x: Self::Vec) -> Self;
        fn get13(x: Self::Vec) -> Self;
        fn get14(x: Self::Vec) -> Self;
        fn get15(x: Self::Vec) -> Self;

        /// Convert the machine-dependent type to the portable representation
        fn to_vec(Self::Machine) -> Self::Vec;

        /// Convert the portable representation to the machine-dependent type
        fn from_vec(Self::Vec) -> Self::Machine;
    }

    #[derive(Copy, Clone, Debug)]
    #[allow(non_camel_case_types)]
    #[repr(simd)]
    pub struct __m128i(i64, i64);

    #[derive(Copy, Clone, Debug)]
    #[allow(non_camel_case_types)]
    #[repr(simd)]
    pub struct __m256i(i64, i64, i64, i64);

    /// Define From implementations between portable and machine types
    macro_rules! conversions {
        ($pty: ty, $mty: ty) => {
            impl From<$mty> for $pty {
                fn from(x: $mty) -> Self {
                    union U {
                        intel: $mty,
                        sane: $pty,
                    }
                    let u = U{intel: x};
                    unsafe { u.sane }
                }
            }

            impl From<$pty> for $mty {
                fn from(x: $pty) -> Self {
                    union U {
                        intel: $mty,
                        sane: $pty,
                    }
                    let u = U{sane: x};
                    unsafe { u.intel }
                }
            }
        }
    }

    /// Define Vector4 implementation for vectors of $ety and constructor $c
    macro_rules! vector4 {
        ($ety: ty, $pty: ident, $mty: ty) => {
            #[derive(Copy, Clone, Debug)]
            #[allow(non_camel_case_types)]
            #[repr(simd)]
            pub struct $pty(pub $ety, pub $ety, pub $ety, pub $ety);

            conversions!($pty, $mty);

            impl Vector4 for $ety {
                type Machine = $mty;
                type Vec = $pty;

                fn new(x0: Self, x1: Self, x2: Self, x3: Self) -> Self::Vec {
                    $pty(x0, x1, x2, x3)
                }

                fn get0(x: Self::Vec) -> Self { x.0 }
                fn get1(x: Self::Vec) -> Self { x.1 }
                fn get2(x: Self::Vec) -> Self { x.2 }
                fn get3(x: Self::Vec) -> Self { x.3 }

                fn to_vec(x: Self::Machine) -> Self::Vec {
                    x.into()
                }
                fn from_vec(x: Self::Vec) -> Self::Machine {
                    x.into()
                }
            }
        }
    }

    /// Define Vector8 implementation for vectors of $ety and constructor $c
    macro_rules! vector8 {
        ($ety: ty, $pty: ident, $mty: ty) => {
            #[derive(Copy, Clone, Debug)]
            #[allow(non_camel_case_types)]
            #[repr(simd)]
            pub struct $pty(pub $ety, pub $ety, pub $ety, pub $ety, pub $ety, pub $ety, pub $ety, pub $ety);

            conversions!($pty, $mty);

            impl Vector8 for $ety {
                type Machine = $mty;
                type Vec = $pty;

                fn new(x0: Self, x1: Self, x2: Self, x3: Self, x4: Self, x5: Self, x6: Self, x7: Self) -> Self::Vec {
                    $pty(x0, x1, x2, x3, x4, x5, x6, x7)
                }

                fn get0(x: Self::Vec) -> Self { x.0 }
                fn get1(x: Self::Vec) -> Self { x.1 }
                fn get2(x: Self::Vec) -> Self { x.2 }
                fn get3(x: Self::Vec) -> Self { x.3 }
                fn get4(x: Self::Vec) -> Self { x.4 }
                fn get5(x: Self::Vec) -> Self { x.5 }
                fn get6(x: Self::Vec) -> Self { x.6 }
                fn get7(x: Self::Vec) -> Self { x.7 }

                fn to_vec(x: Self::Machine) -> Self::Vec {
                    x.into()
                }
                fn from_vec(x: Self::Vec) -> Self::Machine {
                    x.into()
                }
            }
        }
    }

    /// Define Vector8 implementation for vectors of $ety and constructor $c
    macro_rules! vector16 {
        ($ety: ty, $pty: ident, $mty: ty) => {
            #[derive(Copy, Clone, Debug)]
            #[allow(non_camel_case_types)]
            #[repr(simd)]
            pub struct $pty(
                pub $ety, pub $ety, pub $ety, pub $ety, pub $ety, pub $ety, pub $ety, pub $ety,
                pub $ety, pub $ety, pub $ety, pub $ety, pub $ety, pub $ety, pub $ety, pub $ety);

            conversions!($pty, $mty);

            impl Vector16 for $ety {
                type Machine = $mty;
                type Vec = $pty;

                fn new(x0: Self, x1: Self, x2: Self, x3: Self, x4: Self, x5: Self, x6: Self, x7: Self,
                       x8: Self, x9: Self, x10: Self, x11: Self, x12: Self, x13: Self, x14: Self, x15: Self) -> Self::Vec {
                    $pty(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15)
                }

                fn get0(x: Self::Vec) -> Self { x.0 }
                fn get1(x: Self::Vec) -> Self { x.1 }
                fn get2(x: Self::Vec) -> Self { x.2 }
                fn get3(x: Self::Vec) -> Self { x.3 }
                fn get4(x: Self::Vec) -> Self { x.4 }
                fn get5(x: Self::Vec) -> Self { x.5 }
                fn get6(x: Self::Vec) -> Self { x.6 }
                fn get7(x: Self::Vec) -> Self { x.7 }
                fn get8(x: Self::Vec) -> Self { x.8 }
                fn get9(x: Self::Vec) -> Self { x.9 }
                fn get10(x: Self::Vec) -> Self { x.10 }
                fn get11(x: Self::Vec) -> Self { x.11 }
                fn get12(x: Self::Vec) -> Self { x.12 }
                fn get13(x: Self::Vec) -> Self { x.13 }
                fn get14(x: Self::Vec) -> Self { x.14 }
                fn get15(x: Self::Vec) -> Self { x.15 }

                fn to_vec(x: Self::Machine) -> Self::Vec {
                    x.into()
                }
                fn from_vec(x: Self::Vec) -> Self::Machine {
                    x.into()
                }
            }
        }
    }

    vector4!(u32, u32x4, __m128i);
    vector4!(u64, u64x4, __m256i);

    vector8!(u16, u16x8, __m128i);
    vector8!(u32, u32x8, __m256i);

    vector16!(u8,  u8x16, __m128i);
    vector16!(u16, u16x16, __m256i);

    // lift a binary operation over a vector and a scalar (replicating the scalar)
    pub fn lift8_vs_v<F, A, B, R>(f: F, a: A::Machine, b: B) -> R::Machine
        where
            F: Fn(A, B) -> R,
            A: Vector8,
            B: Copy,
            R: Vector8,
    {
        let a = A::to_vec(a);
        let r0  = f(A::get0(a), b);
        let r1  = f(A::get1(a), b);
        let r2  = f(A::get2(a), b);
        let r3  = f(A::get3(a), b);
        let r4  = f(A::get4(a), b);
        let r5  = f(A::get5(a), b);
        let r6  = f(A::get6(a), b);
        let r7  = f(A::get7(a), b);
        let r = R::new(r0, r1, r2, r3, r4, r5, r6, r7);
        R::from_vec(r)
    }

    // lift a binary operation over two vectors
    pub fn lift8_vv_v<F, A, B, R>(f: F, a: A::Machine, b: B::Machine) -> R::Machine
        where
            F: Fn(A, B) -> R,
            A: Vector8,
            B: Vector8,
            R: Vector8,
    {
        let a = A::to_vec(a);
        let b = B::to_vec(b);
        let r0  = f(A::get0(a),  B::get0(b));
        let r1  = f(A::get1(a),  B::get1(b));
        let r2  = f(A::get2(a),  B::get2(b));
        let r3  = f(A::get3(a),  B::get3(b));
        let r4  = f(A::get4(a),  B::get4(b));
        let r5  = f(A::get5(a),  B::get5(b));
        let r6  = f(A::get6(a),  B::get6(b));
        let r7  = f(A::get7(a),  B::get7(b));
        let r = R::new(r0, r1, r2, r3, r4, r5, r6, r7);
        R::from_vec(r)
    }

    // lift a binary operation over two vectors
    pub fn lift16_vv_v<F, A, B, R>(f: F, a: A::Machine, b: B::Machine) -> R::Machine
        where
            F: Fn(A, B) -> R,
            A: Vector16,
            B: Vector16,
            R: Vector16,
    {
        let a = A::to_vec(a);
        let b = B::to_vec(b);
        let r0  = f(A::get0(a),  B::get0(b));
        let r1  = f(A::get1(a),  B::get1(b));
        let r2  = f(A::get2(a),  B::get2(b));
        let r3  = f(A::get3(a),  B::get3(b));
        let r4  = f(A::get4(a),  B::get4(b));
        let r5  = f(A::get5(a),  B::get5(b));
        let r6  = f(A::get6(a),  B::get6(b));
        let r7  = f(A::get7(a),  B::get7(b));
        let r8  = f(A::get8(a),  B::get8(b));
        let r9  = f(A::get9(a),  B::get9(b));
        let r10 = f(A::get10(a), B::get10(b));
        let r11 = f(A::get11(a), B::get11(b));
        let r12 = f(A::get12(a), B::get12(b));
        let r13 = f(A::get13(a), B::get13(b));
        let r14 = f(A::get14(a), B::get14(b));
        let r15 = f(A::get15(a), B::get15(b));
        let r = R::new(r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15);
        R::from_vec(r)
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

    // // map f over vectors then reduce with g
    // pub fn lift8_vv_s<F, G, A, B, R: Copy>(f: F, g: G, a: simd8<A>, b: simd8<B>) -> R
    //     where F: Fn(A, B) -> R,
    //           G: Fn(R, R) -> R
    // {
    //     let r0  = f(a.0,  b.0);
    //     let r1  = f(a.1,  b.1);
    //     let r2  = f(a.2,  b.2);
    //     let r3  = f(a.3,  b.3);
    //     let r4  = f(a.4,  b.4);
    //     let r5  = f(a.5,  b.5);
    //     let r6  = f(a.6,  b.6);
    //     let r7  = f(a.7,  b.7);
    //     reduce8(g, r0, r1, r2, r3, r4, r5, r6, r7)
    // }

    // // map f over vectors then reduce with g
    // pub fn lift16_vv_s<F, G, A, B, R: Copy>(f: F, g: G, a: simd16<A>, b: simd16<B>) -> R
    //     where F: Fn(A, B) -> R,
    //           G: Fn(R, R) -> R
    // {
    //     let r0  = f(a.0,  b.0);
    //     let r1  = f(a.1,  b.1);
    //     let r2  = f(a.2,  b.2);
    //     let r3  = f(a.3,  b.3);
    //     let r4  = f(a.4,  b.4);
    //     let r5  = f(a.5,  b.5);
    //     let r6  = f(a.6,  b.6);
    //     let r7  = f(a.7,  b.7);
    //     let r8  = f(a.8,  b.8);
    //     let r9  = f(a.9,  b.9);
    //     let r10 = f(a.10, b.10);
    //     let r11 = f(a.11, b.11);
    //     let r12 = f(a.12, b.12);
    //     let r13 = f(a.13, b.13);
    //     let r14 = f(a.14, b.14);
    //     let r15 = f(a.15, b.15);
    //     reduce16(g, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15)
    // }
}

mod scalar {
    // todo: there may be some room for sharing code between the different int sizes/signs?

    pub fn cmpeq_u8(x: u8, y: u8) -> u8 {
        if x == y { 0xff } else { 0x0 }
    }

    pub fn cmpeq_u16(x: u16, y: u16) -> u16 {
        if x == y { 0xffff } else { 0x0 }
    }

    /// Logical shift right by 8-bit immediate (0 if shift distance too large)
    pub fn srl_immed_u16_u8(x: u16, imm8: u8) -> u16 {
        if imm8 > 15 { 0 } else { x >> imm8 }
    }

    /// Sign of a u8, expressed as an i32
    /// (todo: not sure the type of this is ideal)
    pub fn sign_u8_i32(x: u8) -> i32 {
        ((x >> 7) & 1) as i32
    }
}

use vector::*;

#[inline]
#[no_mangle]
extern "C" fn llvm_x86_sse2_pcmpeqb_epi8(a: __m128i, b: __m128i) -> __m128i {
    lift16_vv_v(scalar::cmpeq_u8, a, b)
}

#[inline]
#[no_mangle]
extern "C" fn llvm_x86_sse2_pcmpeqw_epi16(a: __m128i, b: __m128i) -> __m128i {
    lift8_vv_v(scalar::cmpeq_u16, a, b)
}

#[inline]
#[no_mangle]
extern "C" fn llvm_x86_sse2_psrli_w(a: __m128i, imm8: i32) -> __m128i {
    lift8_vs_v(scalar::srl_immed_u16_u8, a, imm8 as u8)
}

#[inline]
#[no_mangle]
extern "C" fn llvm_x86_sse2_pmovmskb_128(a: __m128i) -> i32 {
    fn f(x: u8) -> i32 {
        ((x >> 7) & 1) as i32
    }
    let a = <u8 as Vector16>::to_vec(a);
    let r0  = f(<u8 as Vector16>::get0(a));
    let r1  = f(<u8 as Vector16>::get1(a));
    let r2  = f(<u8 as Vector16>::get2(a));
    let r3  = f(<u8 as Vector16>::get3(a));
    let r4  = f(<u8 as Vector16>::get4(a));
    let r5  = f(<u8 as Vector16>::get5(a));
    let r6  = f(<u8 as Vector16>::get6(a));
    let r7  = f(<u8 as Vector16>::get7(a));
    let r8  = f(<u8 as Vector16>::get8(a));
    let r9  = f(<u8 as Vector16>::get9(a));
    let r10 = f(<u8 as Vector16>::get10(a));
    let r11 = f(<u8 as Vector16>::get11(a));
    let r12 = f(<u8 as Vector16>::get12(a));
    let r13 = f(<u8 as Vector16>::get13(a));
    let r14 = f(<u8 as Vector16>::get14(a));
    let r15 = f(<u8 as Vector16>::get15(a));
    let r = (r0  << 0)  | (r1  << 1)  | (r2  << 2)  | (r3  << 3)
          | (r4  << 4)  | (r5  << 5)  | (r6  << 6)  | (r7  << 7)
          | (r8  << 8)  | (r9  << 9)  | (r10 << 10) | (r11 << 11)
          | (r12 << 12) | (r13 << 13) | (r14 << 14) | (r15 << 15);
    r
}
