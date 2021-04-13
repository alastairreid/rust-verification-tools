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

    pub use std::arch::x86_64::{__m128i, __m256i};

    /// Implemented by types that support 4-element vectors
    /// Provides methods to construct, destruct and convert vectors to their
    /// native representation.
    pub trait Vector2
    where
        Self: Copy,
        Self::Vec: Copy,
    {
        /// Portable type used to represent vectors
        type Vec;

        /// Machine-dependent type used to represent vectors
        type Machine;

        /// Construct a vector from individual elements
        fn new(x0: Self, x1: Self) -> Self::Vec;

        /// Extract elements from a vector
        fn get0(x: Self::Vec) -> Self;
        fn get1(x: Self::Vec) -> Self;

        /// Convert the machine-dependent type to the portable representation
        fn to_vec(Self::Machine) -> Self::Vec;

        /// Convert the portable representation to the machine-dependent type
        fn from_vec(Self::Vec) -> Self::Machine;
    }

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
        fn new(
            x0: Self,
            x1: Self,
            x2: Self,
            x3: Self,
            x4: Self,
            x5: Self,
            x6: Self,
            x7: Self,
        ) -> Self::Vec;

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
        fn new(
            x0: Self,
            x1: Self,
            x2: Self,
            x3: Self,
            x4: Self,
            x5: Self,
            x6: Self,
            x7: Self,
            x8: Self,
            x9: Self,
            x10: Self,
            x11: Self,
            x12: Self,
            x13: Self,
            x14: Self,
            x15: Self,
        ) -> Self::Vec;

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

    /// Implemented by types that support 32-element vectors
    /// Provides methods to construct, destruct and convert vectors to their
    /// native representation.
    pub trait Vector32
    where
        Self: Copy,
        Self::Vec: Copy,
    {
        /// Portable type used to represent vectors
        type Vec;

        /// Machine-dependent type used to represent vectors
        type Machine;

        /// Construct a vector from individual elements
        fn new(
            x0: Self,
            x1: Self,
            x2: Self,
            x3: Self,
            x4: Self,
            x5: Self,
            x6: Self,
            x7: Self,
            x8: Self,
            x9: Self,
            x10: Self,
            x11: Self,
            x12: Self,
            x13: Self,
            x14: Self,
            x15: Self,
            x16: Self,
            x17: Self,
            x18: Self,
            x19: Self,
            x20: Self,
            x21: Self,
            x22: Self,
            x23: Self,
            x24: Self,
            x25: Self,
            x26: Self,
            x27: Self,
            x28: Self,
            x29: Self,
            x30: Self,
            x31: Self,
        ) -> Self::Vec;

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
        fn get16(x: Self::Vec) -> Self;
        fn get17(x: Self::Vec) -> Self;
        fn get18(x: Self::Vec) -> Self;
        fn get19(x: Self::Vec) -> Self;
        fn get20(x: Self::Vec) -> Self;
        fn get21(x: Self::Vec) -> Self;
        fn get22(x: Self::Vec) -> Self;
        fn get23(x: Self::Vec) -> Self;
        fn get24(x: Self::Vec) -> Self;
        fn get25(x: Self::Vec) -> Self;
        fn get26(x: Self::Vec) -> Self;
        fn get27(x: Self::Vec) -> Self;
        fn get28(x: Self::Vec) -> Self;
        fn get29(x: Self::Vec) -> Self;
        fn get30(x: Self::Vec) -> Self;
        fn get31(x: Self::Vec) -> Self;

        /// Convert the machine-dependent type to the portable representation
        fn to_vec(Self::Machine) -> Self::Vec;

        /// Convert the portable representation to the machine-dependent type
        fn from_vec(Self::Vec) -> Self::Machine;
    }

    /// Define From implementations between portable and machine types
    macro_rules! conversions {
        ($pty: ty, $mty: ty) => {
            impl From<$mty> for $pty {
                fn from(x: $mty) -> Self {
                    union U {
                        intel: $mty,
                        sane: $pty,
                    }
                    let u = U { intel: x };
                    unsafe { u.sane }
                }
            }

            impl From<$pty> for $mty {
                fn from(x: $pty) -> Self {
                    union U {
                        intel: $mty,
                        sane: $pty,
                    }
                    let u = U { sane: x };
                    unsafe { u.intel }
                }
            }
        };
    }

    /// Define Vector4 implementation for vectors of $ety and constructor $c
    macro_rules! vector2 {
        ($ety: ty, $pty: ident, $mty: ty) => {
            #[derive(Copy, Clone, Debug)]
            #[allow(non_camel_case_types)]
            #[repr(simd)]
            pub struct $pty(pub $ety, pub $ety);

            conversions!($pty, $mty);

            impl Vector2 for $ety {
                type Machine = $mty;
                type Vec = $pty;

                fn new(x0: Self, x1: Self) -> Self::Vec {
                    $pty(x0, x1)
                }

                fn get0(x: Self::Vec) -> Self {
                    x.0
                }
                fn get1(x: Self::Vec) -> Self {
                    x.1
                }

                fn to_vec(x: Self::Machine) -> Self::Vec {
                    x.into()
                }
                fn from_vec(x: Self::Vec) -> Self::Machine {
                    x.into()
                }
            }
        };
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

                fn get0(x: Self::Vec) -> Self {
                    x.0
                }
                fn get1(x: Self::Vec) -> Self {
                    x.1
                }
                fn get2(x: Self::Vec) -> Self {
                    x.2
                }
                fn get3(x: Self::Vec) -> Self {
                    x.3
                }

                fn to_vec(x: Self::Machine) -> Self::Vec {
                    x.into()
                }
                fn from_vec(x: Self::Vec) -> Self::Machine {
                    x.into()
                }
            }
        };
    }

    /// Define Vector8 implementation for vectors of $ety and constructor $c
    macro_rules! vector8 {
        ($ety: ty, $pty: ident, $mty: ty) => {
            #[derive(Copy, Clone, Debug)]
            #[allow(non_camel_case_types)]
            #[repr(simd)]
            pub struct $pty(
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
            );

            conversions!($pty, $mty);

            impl Vector8 for $ety {
                type Machine = $mty;
                type Vec = $pty;

                fn new(
                    x0: Self,
                    x1: Self,
                    x2: Self,
                    x3: Self,
                    x4: Self,
                    x5: Self,
                    x6: Self,
                    x7: Self,
                ) -> Self::Vec {
                    $pty(x0, x1, x2, x3, x4, x5, x6, x7)
                }

                fn get0(x: Self::Vec) -> Self {
                    x.0
                }
                fn get1(x: Self::Vec) -> Self {
                    x.1
                }
                fn get2(x: Self::Vec) -> Self {
                    x.2
                }
                fn get3(x: Self::Vec) -> Self {
                    x.3
                }
                fn get4(x: Self::Vec) -> Self {
                    x.4
                }
                fn get5(x: Self::Vec) -> Self {
                    x.5
                }
                fn get6(x: Self::Vec) -> Self {
                    x.6
                }
                fn get7(x: Self::Vec) -> Self {
                    x.7
                }

                fn to_vec(x: Self::Machine) -> Self::Vec {
                    x.into()
                }
                fn from_vec(x: Self::Vec) -> Self::Machine {
                    x.into()
                }
            }
        };
    }

    /// Define Vector16 implementation for vectors of $ety and constructor $c
    macro_rules! vector16 {
        ($ety: ty, $pty: ident, $mty: ty) => {
            #[derive(Copy, Clone, Debug)]
            #[allow(non_camel_case_types)]
            #[repr(simd)]
            pub struct $pty(
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
            );

            conversions!($pty, $mty);

            impl Vector16 for $ety {
                type Machine = $mty;
                type Vec = $pty;

                fn new(
                    x0: Self,
                    x1: Self,
                    x2: Self,
                    x3: Self,
                    x4: Self,
                    x5: Self,
                    x6: Self,
                    x7: Self,
                    x8: Self,
                    x9: Self,
                    x10: Self,
                    x11: Self,
                    x12: Self,
                    x13: Self,
                    x14: Self,
                    x15: Self,
                ) -> Self::Vec {
                    $pty(
                        x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15,
                    )
                }

                fn get0(x: Self::Vec) -> Self {
                    x.0
                }
                fn get1(x: Self::Vec) -> Self {
                    x.1
                }
                fn get2(x: Self::Vec) -> Self {
                    x.2
                }
                fn get3(x: Self::Vec) -> Self {
                    x.3
                }
                fn get4(x: Self::Vec) -> Self {
                    x.4
                }
                fn get5(x: Self::Vec) -> Self {
                    x.5
                }
                fn get6(x: Self::Vec) -> Self {
                    x.6
                }
                fn get7(x: Self::Vec) -> Self {
                    x.7
                }
                fn get8(x: Self::Vec) -> Self {
                    x.8
                }
                fn get9(x: Self::Vec) -> Self {
                    x.9
                }
                fn get10(x: Self::Vec) -> Self {
                    x.10
                }
                fn get11(x: Self::Vec) -> Self {
                    x.11
                }
                fn get12(x: Self::Vec) -> Self {
                    x.12
                }
                fn get13(x: Self::Vec) -> Self {
                    x.13
                }
                fn get14(x: Self::Vec) -> Self {
                    x.14
                }
                fn get15(x: Self::Vec) -> Self {
                    x.15
                }

                fn to_vec(x: Self::Machine) -> Self::Vec {
                    x.into()
                }
                fn from_vec(x: Self::Vec) -> Self::Machine {
                    x.into()
                }
            }
        };
    }

    /// Define Vector32 implementation for vectors of $ety and constructor $c
    macro_rules! vector32 {
        ($ety: ty, $pty: ident, $mty: ty) => {
            #[derive(Copy, Clone, Debug)]
            #[allow(non_camel_case_types)]
            #[repr(simd)]
            pub struct $pty(
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
                pub $ety,
            );

            conversions!($pty, $mty);

            impl Vector32 for $ety {
                type Machine = $mty;
                type Vec = $pty;

                fn new(
                    x0: Self,
                    x1: Self,
                    x2: Self,
                    x3: Self,
                    x4: Self,
                    x5: Self,
                    x6: Self,
                    x7: Self,
                    x8: Self,
                    x9: Self,
                    x10: Self,
                    x11: Self,
                    x12: Self,
                    x13: Self,
                    x14: Self,
                    x15: Self,
                    x16: Self,
                    x17: Self,
                    x18: Self,
                    x19: Self,
                    x20: Self,
                    x21: Self,
                    x22: Self,
                    x23: Self,
                    x24: Self,
                    x25: Self,
                    x26: Self,
                    x27: Self,
                    x28: Self,
                    x29: Self,
                    x30: Self,
                    x31: Self,
                ) -> Self::Vec {
                    $pty(
                        x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15, x16,
                        x17, x18, x19, x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x30, x31,
                    )
                }

                fn get0(x: Self::Vec) -> Self {
                    x.0
                }
                fn get1(x: Self::Vec) -> Self {
                    x.1
                }
                fn get2(x: Self::Vec) -> Self {
                    x.2
                }
                fn get3(x: Self::Vec) -> Self {
                    x.3
                }
                fn get4(x: Self::Vec) -> Self {
                    x.4
                }
                fn get5(x: Self::Vec) -> Self {
                    x.5
                }
                fn get6(x: Self::Vec) -> Self {
                    x.6
                }
                fn get7(x: Self::Vec) -> Self {
                    x.7
                }
                fn get8(x: Self::Vec) -> Self {
                    x.8
                }
                fn get9(x: Self::Vec) -> Self {
                    x.9
                }
                fn get10(x: Self::Vec) -> Self {
                    x.10
                }
                fn get11(x: Self::Vec) -> Self {
                    x.11
                }
                fn get12(x: Self::Vec) -> Self {
                    x.12
                }
                fn get13(x: Self::Vec) -> Self {
                    x.13
                }
                fn get14(x: Self::Vec) -> Self {
                    x.14
                }
                fn get15(x: Self::Vec) -> Self {
                    x.15
                }
                fn get16(x: Self::Vec) -> Self {
                    x.16
                }
                fn get17(x: Self::Vec) -> Self {
                    x.17
                }
                fn get18(x: Self::Vec) -> Self {
                    x.18
                }
                fn get19(x: Self::Vec) -> Self {
                    x.19
                }
                fn get20(x: Self::Vec) -> Self {
                    x.20
                }
                fn get21(x: Self::Vec) -> Self {
                    x.21
                }
                fn get22(x: Self::Vec) -> Self {
                    x.22
                }
                fn get23(x: Self::Vec) -> Self {
                    x.23
                }
                fn get24(x: Self::Vec) -> Self {
                    x.24
                }
                fn get25(x: Self::Vec) -> Self {
                    x.25
                }
                fn get26(x: Self::Vec) -> Self {
                    x.26
                }
                fn get27(x: Self::Vec) -> Self {
                    x.27
                }
                fn get28(x: Self::Vec) -> Self {
                    x.28
                }
                fn get29(x: Self::Vec) -> Self {
                    x.29
                }
                fn get30(x: Self::Vec) -> Self {
                    x.30
                }
                fn get31(x: Self::Vec) -> Self {
                    x.31
                }

                fn to_vec(x: Self::Machine) -> Self::Vec {
                    x.into()
                }
                fn from_vec(x: Self::Vec) -> Self::Machine {
                    x.into()
                }
            }
        };
    }

    vector32!(u8, u8x32, __m256i);

    vector16!(u8, u8x16, __m128i);
    vector16!(u16, u16x16, __m256i);

    vector8!(u16, u16x8, __m128i);
    vector8!(u32, u32x8, __m256i);

    vector4!(u32, u32x4, __m128i);
    vector4!(u64, u64x4, __m256i);

    vector2!(u64, u64x2, __m128i);

    // lift a binary operation over vector and scalar
    pub fn lift2_vs_v<F, A, B, R>(f: F, a: A::Machine, b: B) -> R::Machine
    where
        F: Fn(A, B) -> R,
        A: Vector2,
        B: Copy,
        R: Vector2,
    {
        let a = A::to_vec(a);
        let r0 = f(A::get0(a), b);
        let r1 = f(A::get1(a), b);
        let r = R::new(r0, r1);
        R::from_vec(r)
    }

    // lift a binary operation over two vectors
    pub fn lift2_vv_v<F, A, B, R>(f: F, a: A::Machine, b: B::Machine) -> R::Machine
    where
        F: Fn(A, B) -> R,
        A: Vector2,
        B: Vector2,
        R: Vector2,
    {
        let a = A::to_vec(a);
        let b = B::to_vec(b);
        let r0 = f(A::get0(a), B::get0(b));
        let r1 = f(A::get1(a), B::get1(b));
        let r = R::new(r0, r1);
        R::from_vec(r)
    }

    // lift a binary operation over vector and scalar
    pub fn lift4_vs_v<F, A, B, R>(f: F, a: A::Machine, b: B) -> R::Machine
    where
        F: Fn(A, B) -> R,
        A: Vector4,
        B: Copy,
        R: Vector4,
    {
        let a = A::to_vec(a);
        let r0 = f(A::get0(a), b);
        let r1 = f(A::get1(a), b);
        let r2 = f(A::get2(a), b);
        let r3 = f(A::get3(a), b);
        let r = R::new(r0, r1, r2, r3);
        R::from_vec(r)
    }

    // lift a binary operation over two vectors
    pub fn lift4_vv_v<F, A, B, R>(f: F, a: A::Machine, b: B::Machine) -> R::Machine
    where
        F: Fn(A, B) -> R,
        A: Vector4,
        B: Vector4,
        R: Vector4,
    {
        let a = A::to_vec(a);
        let b = B::to_vec(b);
        let r0 = f(A::get0(a), B::get0(b));
        let r1 = f(A::get1(a), B::get1(b));
        let r2 = f(A::get2(a), B::get2(b));
        let r3 = f(A::get3(a), B::get3(b));
        let r = R::new(r0, r1, r2, r3);
        R::from_vec(r)
    }

    // lift a binary operation over vector and scalar
    pub fn lift8_vs_v<F, A, B, R>(f: F, a: A::Machine, b: B) -> R::Machine
    where
        F: Fn(A, B) -> R,
        A: Vector8,
        B: Copy,
        R: Vector8,
    {
        let a = A::to_vec(a);
        let r0 = f(A::get0(a), b);
        let r1 = f(A::get1(a), b);
        let r2 = f(A::get2(a), b);
        let r3 = f(A::get3(a), b);
        let r4 = f(A::get4(a), b);
        let r5 = f(A::get5(a), b);
        let r6 = f(A::get6(a), b);
        let r7 = f(A::get7(a), b);
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
        let r0 = f(A::get0(a), B::get0(b));
        let r1 = f(A::get1(a), B::get1(b));
        let r2 = f(A::get2(a), B::get2(b));
        let r3 = f(A::get3(a), B::get3(b));
        let r4 = f(A::get4(a), B::get4(b));
        let r5 = f(A::get5(a), B::get5(b));
        let r6 = f(A::get6(a), B::get6(b));
        let r7 = f(A::get7(a), B::get7(b));
        let r = R::new(r0, r1, r2, r3, r4, r5, r6, r7);
        R::from_vec(r)
    }

    // lift a scalar to a vector
    pub fn lift16_s_v<R>(a: R) -> R::Machine
    where
        R: Vector16,
    {
        let r = R::new(
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
        );
        R::from_vec(r)
    }

    // lift a binary operation over vector and scalar
    pub fn lift16_vs_v<F, A, B, R>(f: F, a: A::Machine, b: B) -> R::Machine
    where
        F: Fn(A, B) -> R,
        A: Vector16,
        B: Copy,
        R: Vector16,
    {
        let a = A::to_vec(a);
        let r0 = f(A::get0(a), b);
        let r1 = f(A::get1(a), b);
        let r2 = f(A::get2(a), b);
        let r3 = f(A::get3(a), b);
        let r4 = f(A::get4(a), b);
        let r5 = f(A::get5(a), b);
        let r6 = f(A::get6(a), b);
        let r7 = f(A::get7(a), b);
        let r8 = f(A::get8(a), b);
        let r9 = f(A::get9(a), b);
        let r10 = f(A::get10(a), b);
        let r11 = f(A::get11(a), b);
        let r12 = f(A::get12(a), b);
        let r13 = f(A::get13(a), b);
        let r14 = f(A::get14(a), b);
        let r15 = f(A::get15(a), b);
        let r = R::new(
            r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15,
        );
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
        let r0 = f(A::get0(a), B::get0(b));
        let r1 = f(A::get1(a), B::get1(b));
        let r2 = f(A::get2(a), B::get2(b));
        let r3 = f(A::get3(a), B::get3(b));
        let r4 = f(A::get4(a), B::get4(b));
        let r5 = f(A::get5(a), B::get5(b));
        let r6 = f(A::get6(a), B::get6(b));
        let r7 = f(A::get7(a), B::get7(b));
        let r8 = f(A::get8(a), B::get8(b));
        let r9 = f(A::get9(a), B::get9(b));
        let r10 = f(A::get10(a), B::get10(b));
        let r11 = f(A::get11(a), B::get11(b));
        let r12 = f(A::get12(a), B::get12(b));
        let r13 = f(A::get13(a), B::get13(b));
        let r14 = f(A::get14(a), B::get14(b));
        let r15 = f(A::get15(a), B::get15(b));
        let r = R::new(
            r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15,
        );
        R::from_vec(r)
    }

    // lift a scalar to a vector
    pub fn lift32_s_v<R>(a: R) -> R::Machine
    where
        R: Vector32,
    {
        let r = R::new(
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
            a,
        );
        R::from_vec(r)
    }

    // lift a binary operation over vector and scalar
    pub fn lift32_vs_v<F, A, B, R>(f: F, a: A::Machine, b: B) -> R::Machine
    where
        F: Fn(A, B) -> R,
        A: Vector32,
        B: Copy,
        R: Vector32,
    {
        let a = A::to_vec(a);
        let r0 = f(A::get0(a), b);
        let r1 = f(A::get1(a), b);
        let r2 = f(A::get2(a), b);
        let r3 = f(A::get3(a), b);
        let r4 = f(A::get4(a), b);
        let r5 = f(A::get5(a), b);
        let r6 = f(A::get6(a), b);
        let r7 = f(A::get7(a), b);
        let r8 = f(A::get8(a), b);
        let r9 = f(A::get9(a), b);
        let r10 = f(A::get10(a), b);
        let r11 = f(A::get11(a), b);
        let r12 = f(A::get12(a), b);
        let r13 = f(A::get13(a), b);
        let r14 = f(A::get14(a), b);
        let r15 = f(A::get15(a), b);
        let r16 = f(A::get16(a), b);
        let r17 = f(A::get17(a), b);
        let r18 = f(A::get18(a), b);
        let r19 = f(A::get19(a), b);
        let r20 = f(A::get20(a), b);
        let r21 = f(A::get21(a), b);
        let r22 = f(A::get22(a), b);
        let r23 = f(A::get23(a), b);
        let r24 = f(A::get24(a), b);
        let r25 = f(A::get25(a), b);
        let r26 = f(A::get26(a), b);
        let r27 = f(A::get27(a), b);
        let r28 = f(A::get28(a), b);
        let r29 = f(A::get29(a), b);
        let r30 = f(A::get30(a), b);
        let r31 = f(A::get31(a), b);
        let r = R::new(
            r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15, r16, r17, r18,
            r19, r20, r21, r22, r23, r24, r25, r26, r27, r28, r29, r30, r31,
        );
        R::from_vec(r)
    }

    // lift a binary operation over two vectors
    pub fn lift32_vv_v<F, A, B, R>(f: F, a: A::Machine, b: B::Machine) -> R::Machine
    where
        F: Fn(A, B) -> R,
        A: Vector32,
        B: Vector32,
        R: Vector32,
    {
        let a = A::to_vec(a);
        let b = B::to_vec(b);
        let r0 = f(A::get0(a), B::get0(b));
        let r1 = f(A::get1(a), B::get1(b));
        let r2 = f(A::get2(a), B::get2(b));
        let r3 = f(A::get3(a), B::get3(b));
        let r4 = f(A::get4(a), B::get4(b));
        let r5 = f(A::get5(a), B::get5(b));
        let r6 = f(A::get6(a), B::get6(b));
        let r7 = f(A::get7(a), B::get7(b));
        let r8 = f(A::get8(a), B::get8(b));
        let r9 = f(A::get9(a), B::get9(b));
        let r10 = f(A::get10(a), B::get10(b));
        let r11 = f(A::get11(a), B::get11(b));
        let r12 = f(A::get12(a), B::get12(b));
        let r13 = f(A::get13(a), B::get13(b));
        let r14 = f(A::get14(a), B::get14(b));
        let r15 = f(A::get15(a), B::get15(b));
        let r16 = f(A::get16(a), B::get16(b));
        let r17 = f(A::get17(a), B::get17(b));
        let r18 = f(A::get18(a), B::get18(b));
        let r19 = f(A::get19(a), B::get19(b));
        let r20 = f(A::get20(a), B::get20(b));
        let r21 = f(A::get21(a), B::get21(b));
        let r22 = f(A::get22(a), B::get22(b));
        let r23 = f(A::get23(a), B::get23(b));
        let r24 = f(A::get24(a), B::get24(b));
        let r25 = f(A::get25(a), B::get25(b));
        let r26 = f(A::get26(a), B::get26(b));
        let r27 = f(A::get27(a), B::get27(b));
        let r28 = f(A::get28(a), B::get28(b));
        let r29 = f(A::get29(a), B::get29(b));
        let r30 = f(A::get30(a), B::get30(b));
        let r31 = f(A::get31(a), B::get31(b));
        let r = R::new(
            r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15, r16, r17, r18,
            r19, r20, r21, r22, r23, r24, r25, r26, r27, r28, r29, r30, r31,
        );
        R::from_vec(r)
    }

    // create a function $f to reduce an array of length $n
    macro_rules! reducer {
        ($n: expr, $f: ident) => {
            pub fn $f<F, A: Copy>(f: F, a: [A; $n]) -> A
            where
                F: Fn(usize, A, A) -> A,
            {
                let mut r = a[0];
                for i in 1..$n {
                    r = f(i, r, a[i]);
                }
                r
            }
        };
    }

    reducer!(2, reduce2);
    reducer!(4, reduce4);
    reducer!(8, reduce8);
    reducer!(16, reduce16);
    reducer!(32, reduce32);

    // lift a unary operation over a vector and reduce
    pub fn lift2_v_s<F, G, A, R>(f: F, g: G, a: A::Machine) -> R
    where
        F: Fn(A) -> R,
        G: Fn(usize, R, R) -> R,
        A: Vector4,
        R: Copy,
    {
        let a = A::to_vec(a);
        let r0 = f(A::get0(a));
        let r1 = f(A::get1(a));
        reduce2(g, [r0, r1])
    }

    // lift a unary operation over a vector and reduce
    pub fn lift4_v_s<F, G, A, R>(f: F, g: G, a: A::Machine) -> R
    where
        F: Fn(A) -> R,
        G: Fn(usize, R, R) -> R,
        A: Vector4,
        R: Copy,
    {
        let a = A::to_vec(a);
        let r0 = f(A::get0(a));
        let r1 = f(A::get1(a));
        let r2 = f(A::get2(a));
        let r3 = f(A::get3(a));
        reduce4(g, [r0, r1, r2, r3])
    }

    // lift a unary operation over a vector and reduce
    pub fn lift8_v_s<F, G, A, R>(f: F, g: G, a: A::Machine) -> R
    where
        F: Fn(A) -> R,
        G: Fn(usize, R, R) -> R,
        A: Vector8,
        R: Copy,
    {
        let a = A::to_vec(a);
        let r0 = f(A::get0(a));
        let r1 = f(A::get1(a));
        let r2 = f(A::get2(a));
        let r3 = f(A::get3(a));
        let r4 = f(A::get4(a));
        let r5 = f(A::get5(a));
        let r6 = f(A::get6(a));
        let r7 = f(A::get7(a));
        reduce8(g, [r0, r1, r2, r3, r4, r5, r6, r7])
    }

    // lift a unary operation over a vector and reduce
    pub fn lift16_v_s<F, G, A, R>(f: F, g: G, a: A::Machine) -> R
    where
        F: Fn(A) -> R,
        G: Fn(usize, R, R) -> R,
        A: Vector16,
        R: Copy,
    {
        let a = A::to_vec(a);
        let r0 = f(A::get0(a));
        let r1 = f(A::get1(a));
        let r2 = f(A::get2(a));
        let r3 = f(A::get3(a));
        let r4 = f(A::get4(a));
        let r5 = f(A::get5(a));
        let r6 = f(A::get6(a));
        let r7 = f(A::get7(a));
        let r8 = f(A::get8(a));
        let r9 = f(A::get9(a));
        let r10 = f(A::get10(a));
        let r11 = f(A::get11(a));
        let r12 = f(A::get12(a));
        let r13 = f(A::get13(a));
        let r14 = f(A::get14(a));
        let r15 = f(A::get15(a));
        reduce16(
            g,
            [
                r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15,
            ],
        )
    }

    // lift a unary operation over a vector and reduce
    pub fn lift32_v_s<F, G, A, R>(f: F, g: G, a: A::Machine) -> R
    where
        F: Fn(A) -> R,
        G: Fn(usize, R, R) -> R,
        A: Vector32,
        R: Copy,
    {
        let a = A::to_vec(a);
        let r0 = f(A::get0(a));
        let r1 = f(A::get1(a));
        let r2 = f(A::get2(a));
        let r3 = f(A::get3(a));
        let r4 = f(A::get4(a));
        let r5 = f(A::get5(a));
        let r6 = f(A::get6(a));
        let r7 = f(A::get7(a));
        let r8 = f(A::get8(a));
        let r9 = f(A::get9(a));
        let r10 = f(A::get10(a));
        let r11 = f(A::get11(a));
        let r12 = f(A::get12(a));
        let r13 = f(A::get13(a));
        let r14 = f(A::get14(a));
        let r15 = f(A::get15(a));
        let r16 = f(A::get16(a));
        let r17 = f(A::get17(a));
        let r18 = f(A::get18(a));
        let r19 = f(A::get19(a));
        let r20 = f(A::get20(a));
        let r21 = f(A::get21(a));
        let r22 = f(A::get22(a));
        let r23 = f(A::get23(a));
        let r24 = f(A::get24(a));
        let r25 = f(A::get25(a));
        let r26 = f(A::get26(a));
        let r27 = f(A::get27(a));
        let r28 = f(A::get28(a));
        let r29 = f(A::get29(a));
        let r30 = f(A::get30(a));
        let r31 = f(A::get31(a));
        reduce32(
            g,
            [
                r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15, r16, r17,
                r18, r19, r20, r21, r22, r23, r24, r25, r26, r27, r28, r29, r30, r31,
            ],
        )
    }
}

mod scalar {
    // todo: there may be some room for sharing code between the different int sizes/signs?

    pub fn and64(x: u64, y: u64) -> u64 {
        x & y
    }

    pub fn andnot64(x: u64, y: u64) -> u64 {
        x & !y
    }

    pub fn or64(x: u64, y: u64) -> u64 {
        x | y
    }

    pub fn xor64(x: u64, y: u64) -> u64 {
        x ^ y
    }

    pub fn cmpeq_u8(x: u8, y: u8) -> u8 {
        if x == y {
            0xff
        } else {
            0x0
        }
    }

    pub fn cmpeq_u16(x: u16, y: u16) -> u16 {
        if x == y {
            0xffff
        } else {
            0x0
        }
    }

    /// Logical shift right by 8-bit immediate (0 if shift distance too large)
    pub fn srl_immed_u8_u8(x: u8, imm8: u8) -> u8 {
        if imm8 > 7 {
            0
        } else {
            x >> imm8
        }
    }

    /// Logical shift right by 8-bit immediate (0 if shift distance too large)
    pub fn srl_immed_u16_u8(x: u16, imm8: u8) -> u16 {
        if imm8 > 15 {
            0
        } else {
            x >> imm8
        }
    }

    /// Logical shift right by 8-bit immediate (0 if shift distance too large)
    pub fn srl_immed_u32_u8(x: u32, imm8: u8) -> u32 {
        if imm8 > 31 {
            0
        } else {
            x >> imm8
        }
    }

    /// Logical shift right by 8-bit immediate (0 if shift distance too large)
    pub fn srl_immed_u64_u8(x: u64, imm8: u8) -> u64 {
        if imm8 > 63 {
            0
        } else {
            x >> imm8
        }
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
unsafe extern "C" fn llvm_x86_sse2_pcmpeqb_epi8(a: __m128i, b: __m128i) -> __m128i {
    lift16_vv_v(scalar::cmpeq_u8, a, b)
}

#[inline]
#[no_mangle]
unsafe extern "C" fn llvm_x86_sse2_pcmpeqw_epi16(a: __m128i, b: __m128i) -> __m128i {
    lift8_vv_v(scalar::cmpeq_u16, a, b)
}

#[inline]
#[no_mangle]
unsafe extern "C" fn llvm_x86_sse2_psrli_b(a: __m128i, imm8: i32) -> __m128i {
    lift16_vs_v(scalar::srl_immed_u8_u8, a, imm8 as u8)
}

#[inline]
#[no_mangle]
unsafe extern "C" fn llvm_x86_sse2_psrli_w(a: __m128i, imm8: i32) -> __m128i {
    lift8_vs_v(scalar::srl_immed_u16_u8, a, imm8 as u8)
}

#[inline]
#[no_mangle]
unsafe extern "C" fn llvm_x86_sse2_psrli_d(a: __m128i, imm8: i32) -> __m128i {
    lift4_vs_v(scalar::srl_immed_u32_u8, a, imm8 as u8)
}

#[inline]
#[no_mangle]
unsafe extern "C" fn llvm_x86_sse2_psrli_q(a: __m128i, imm8: i32) -> __m128i {
    lift2_vs_v(scalar::srl_immed_u64_u8, a, imm8 as u8)
}

#[inline]
#[no_mangle]
unsafe extern "C" fn llvm_x86_avx2_psrli_b(a: __m256i, imm8: i32) -> __m256i {
    lift32_vs_v(scalar::srl_immed_u8_u8, a, imm8 as u8)
}

#[inline]
#[no_mangle]
unsafe extern "C" fn llvm_x86_avx2_psrli_w(a: __m256i, imm8: i32) -> __m256i {
    lift16_vs_v(scalar::srl_immed_u16_u8, a, imm8 as u8)
}

#[inline]
#[no_mangle]
unsafe extern "C" fn llvm_x86_avx2_psrli_d(a: __m256i, imm8: i32) -> __m256i {
    lift8_vs_v(scalar::srl_immed_u32_u8, a, imm8 as u8)
}

#[inline]
#[no_mangle]
unsafe extern "C" fn llvm_x86_avx2_psrli_q(a: __m256i, imm8: i32) -> __m256i {
    lift4_vs_v(scalar::srl_immed_u64_u8, a, imm8 as u8)
}

#[inline]
#[no_mangle]
unsafe extern "C" fn llvm_x86_sse2_pmovmskb_128(a: __m128i) -> i32 {
    lift16_v_s(scalar::sign_u8_i32, |i, x, y| x | (y << i), a)
}

#[inline]
#[no_mangle]
unsafe extern "C" fn _mm_and_si128(a: __m128i, b: __m128i) -> __m128i {
    lift2_vv_v(scalar::and64, a, b)
}

#[inline]
#[no_mangle]
unsafe extern "C" fn _mm256_and_si256(a: __m256i, b: __m256i) -> __m256i {
    lift4_vv_v(scalar::and64, a, b)
}

#[inline]
#[no_mangle]
unsafe extern "C" fn _mm_andnot_si128(a: __m128i, b: __m128i) -> __m128i {
    lift2_vv_v(scalar::andnot64, a, b)
}

#[inline]
#[no_mangle]
unsafe extern "C" fn _mm256_andnot_si256(a: __m256i, b: __m256i) -> __m256i {
    lift4_vv_v(scalar::andnot64, a, b)
}

#[inline]
#[no_mangle]
unsafe extern "C" fn _mm_or_si128(a: __m128i, b: __m128i) -> __m128i {
    lift2_vv_v(scalar::or64, a, b)
}

#[inline]
#[no_mangle]
unsafe extern "C" fn _mm256_or_si256(a: __m256i, b: __m256i) -> __m256i {
    lift4_vv_v(scalar::or64, a, b)
}

#[inline]
#[no_mangle]
unsafe extern "C" fn _mm_xor_si128(a: __m128i, b: __m128i) -> __m128i {
    lift2_vv_v(scalar::xor64, a, b)
}

#[inline]
#[no_mangle]
unsafe extern "C" fn _mm256_xor_si256(a: __m256i, b: __m256i) -> __m256i {
    lift4_vv_v(scalar::xor64, a, b)
}

#[inline]
#[no_mangle]
unsafe extern "C" fn _mm_set1_epi8(a: i8) -> __m128i {
    lift16_s_v(a as u8)
}


#[inline]
#[no_mangle]
unsafe extern "C" fn _mm256_set1_epi8(a: i8) -> __m256i {
    lift32_s_v(a as u8)
}


#[inline]
#[no_mangle]
unsafe extern "C" fn llvm_x86_ssse3_pshuf_b_128(a: __m128i, b: __m128i) -> __m128i {
    union U {
        intel: __m128i,
        arr: [u8; 16],
    }
    let a = unsafe { U { intel: a }.arr };
    let b = unsafe { U { intel: b }.arr };
    let mut r = [0; 16];
    for i in 0..16 {
        let j = b[i] & 15;
        r[i] = a[j as usize];
    }
    unsafe { U { arr: r }.intel }
}

#[inline]
#[no_mangle]
unsafe extern "C" fn llvm_x86_ssse3_pshuf_w_128(a: __m128i, b: __m128i) -> __m128i {
    union U {
        intel: __m128i,
        arr: [u16; 8],
    }
    let a = unsafe { U { intel: a }.arr };
    let b = unsafe { U { intel: b }.arr };
    let mut r = [0; 8];
    for i in 0..8 {
        let j = b[i] & 7;
        r[i] = a[j as usize];
    }
    unsafe { U { arr: r }.intel }
}

#[inline]
#[no_mangle]
unsafe extern "C" fn _mm_loadu_si128(mem_addr: *const __m128i) -> __m128i {
    let mut dst: __m128i = _mm_set1_epi8(0);
    std::ptr::copy_nonoverlapping(
        mem_addr as *const u8,
        &mut dst as *mut __m128i as *mut u8,
        std::mem::size_of::<__m128i>(),
    );
    dst
}

#[inline]
#[no_mangle]
unsafe extern "C" fn _mm256_loadu_si256(mem_addr: *const __m256i) -> __m256i {
    let mut dst: __m256i = _mm256_set1_epi8(0);
    std::ptr::copy_nonoverlapping(
        mem_addr as *const u8,
        &mut dst as *mut __m256i as *mut u8,
        std::mem::size_of::<__m256i>(),
    );
    dst
}

#[inline]
#[no_mangle]
unsafe extern "C" fn _mm_storeu_si128(mem_addr: *mut __m128i, a: __m128i) {
    std::ptr::copy_nonoverlapping(
        &a as *const __m128i as *const u8,
        mem_addr as *mut u8,
        std::mem::size_of::<__m128i>(),
    )
}

#[inline]
#[no_mangle]
unsafe extern "C" fn _mm_storeu_si256(mem_addr: *mut __m256i, a: __m256i) {
    std::ptr::copy_nonoverlapping(
        &a as *const __m256i as *const u8,
        mem_addr as *mut u8,
        std::mem::size_of::<__m256i>(),
    )
}

#[inline]
#[no_mangle]
unsafe extern "C" fn _mm_cvtsi128_si32(a: __m128i) -> i32 {
    let a: u32x4 = a.into();
    a.0 as i32
}

#[inline]
#[no_mangle]
pub unsafe fn _mm_srli_si128(a: __m128i, imm8: i32) -> __m128i {
    let imm8: u8 = imm8 as u8;
    let imm8 = if imm8 > 15 { 16 } else { imm8 };
    union U {
        intel: __m128i,
        u128: u128,
    }
    let a = unsafe { U { intel: a }.u128 };
    let r = a >> (8 * imm8);
    unsafe { U { u128: r }.intel }
}
