# Copyright 2021 The Propverify authors
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

# default target backend: override on command line
TGT = klee

# this rule comes first to make sure that it is the default target
default: rvt-$(TGT).bc

CC = clang-10
CFLAGS = -flto=thin
CFLAGS += -O1

RUSTC = rustc
RSFLAGS = --emit=llvm-bc --crate-type=lib -O
RSFLAGS += --emit=mir
LINK = llvm-link-10

C_SRCS := $(wildcard src/*.c)
C_OBJS := $(patsubst src/%.c, build_$(TGT)/%.bc, $(C_SRCS))

RS_SRCS := $(wildcard src/*.rs)
RS_OBJS := $(patsubst src/%.rs, build_$(TGT)/%.bc, $(RS_SRCS))

OBJS := $(C_OBJS) $(RS_OBJS)

build_$(TGT)/%.bc: src/%.c
	mkdir -p build_$(TGT)
	$(CC) $(CFLAGS) $^ -c -o $@

build_$(TGT)/%.bc: src/%.rs
	mkdir -p build_$(TGT)
	$(RUSTC) $(RSFLAGS) $^ -o $@

rvt-$(TGT).bc: $(OBJS)
	$(LINK) $(OBJS) -o $@

clean::
	$(RM) $(OBJS)
	$(RM) rvt-$(TGT).bc

# End of Makefile
