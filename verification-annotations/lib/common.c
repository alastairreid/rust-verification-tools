// Copyright 2020-2021 The Propverify authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#include <stdint.h>

// An opaque function intended to prevent the Rust/LLVM
// compiler from optimizing out a piece of code.
void verification_use_u32(uint32_t x __attribute__((unused))) {
}
