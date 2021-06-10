---
layout: post
title: Using KLEE on Coreutils
---

![KLEE logo](https://klee.github.io/images/klee.svg){: style="float: left; width: 10%; padding: 1%"}
A lot of our work over the last year was on [identifying and fixing obstacles to using KLEE with Rust][KLEE status]
and the main technique we used for finding new obstacles was to try to use
[KLEE] with different Rust programs and libraries.
One of the largest suites of programs we tackled was the [Rust CoreUtils]
library: a Rust rewrite and drop in replacement for the GNU CoreUtils suite
that includes programs like ls, cp, df, cat, and about 90 other standard Unix shell
commands.
This is a brief summary of how to use [RVT][RVT git repo] and [KLEE] on the
[Rust CoreUtils].

## Setup

The first step is to fetch RVT and build a docker image (this will take about
15-20 minutes).

``` shell
# install docker
sudo apt-get install -y docker
sudo groupadd docker
sudo usermod -aG docker $USER
# consider logging out and back in to make the last command take effect

# install RVT and build a docker image
git clone https://github.com/project-oak/rust-verification-tools.git rvt
cd rvt
docker/build

# fetch coreutils
git clone https://github.com/uutils/coreutils
```

And, from now on, let's run everything in docker.
This ensures that you are using the right version of the Rust compiler, etc.

```
docker/run
```

(Actually, I find it convenient to have two terminals open: one not using docker
that I run an editor in and one inside docker where I execute commands.)


## Using `cargo-verify` on one coreutils application

Let's start with the first coreutils application: `arch`

```
cd coreutils/src/uu/arch
```

To use [KLEE] on a Rust crate, you need to

1. Add some extra dependencies to the Cargo.toml file
2. Use RVT's `cargo-verify` program to build the crate
3. Run KLEE on the resulting LLVM bitcode file

The extra dependencies look like this and should be added to the end of
`coreutils/src/uu/arch/Cargo.toml`.

```
[target.'cfg(not(verify))'.dependencies]
proptest = { version = "0.10" }

[target.'cfg(verify)'.dependencies]
propverify = { path="/home/rust-verification-tools/propverify" }

[features]
verifier-klee = ["propverify/verifier-klee"]
verifier-crux = ["propverify/verifier-crux"]
verifier-seahorn = ["propverify/verifier-seahorn"]
```

We need to build two *.bc files that will be linked since this is our first time using the docker container.
```bash
pushd /home/rust-verification-tools/runtime/; make; popd
pushd /home/rust-verification-tools/simd_emulation/; make; popd
```

Note that these mention the [PropTest]
property-based testing library, the [Crux-MIR] verifier and the [SeaHorn]
verifier as well – I like to add the same text no matter what tool I am
using.

Now we use RVT's `cargo-verify` tool to build the crate and generate
a bitcode file `app.bc` that is suitable for KLEE to use.
The `-v` flag increases verbosity which helps reduce anxiety a little since the command takes a
minute or so to run.

```
cargo verify -v --bin arch -o app.bc
```

Finally, we run [KLEE] on the bitcode file[^ignoring-warnings-may-not-be-safe].
To start off, we will use a subset of the KLEE recommended flags
described in [KLEE's page about the GNU coreutils experiments](https://klee.github.io/docs/coreutils-experiments/)[^only-a-subset]

[^ignoring-warnings-may-not-be-safe]:
    Note that we are ignoring warnings. Some of these may be important but we can
    still get useful results from KLEE despite the warnings.)

[^only-a-subset]:
    We are using a subset of the recommended arguments only for simplicity.
    The only recommended KLEE argument that we cannot use is `--optimize`.
    If you want to try with the full set of flags, you could try

        klee --disable-verify --warnings-only-to-file --simplify-sym-indices \
        --write-cvcs --write-cov --output-module --max-memory=1000 \
        --disable-inlining --use-forked-solver --use-cex-cache --libc=klee \
        --posix-runtime --external-calls=all --only-output-states-covering-new \
        --env-file=test.env --max-sym-array-size=4096 --max-solver-time=30s \
        --max-time=60min --watchdog --max-memory-inhibit=false \
        --max-static-fork-pct=1 --max-static-solve-pct=1 \
        --max-static-cpfork-pct=1 --switch-type=internal --search=random-path \
        --search=nurs:covnew --use-batching-search --batch-instructions=10000 \
        app.bc \
        --sym-args 0 1 10 --sym-args 0 2 2 --sym-files 1 8 --sym-stdin 8 --sym-stdout


```
klee --libc=uclibc --posix-runtime --disable-verify --warnings-only-to-file app.bc --sym-args 0 3 10 --sym-files 2 8
```

This produces the following output (actually, this is just the output it
produces in the first minute or so – I hit ctrl-C after a while).

```
KLEE: NOTE: Using POSIX model: /usr/lib/x86_64-linux-gnu/klee/runtime/libkleeRuntimePOSIX64_Debug+Asserts.bca
KLEE: NOTE: Using klee-uclibc : /usr/lib/x86_64-linux-gnu/klee/runtime/klee-uclibc.bca
KLEE: output directory is "/usr/local/google/home/adreid/rust/rvt/coreutils/src/uu/arch/klee-out-1"
KLEE: Using STP solver backend
x86_64
error: Found argument '' which wasn't expected, or isn't valid in this context

USAGE:
    app.bc

For more information try --help
error: Found argument '�' which wasn't expected, or isn't valid in this context

USAGE:
    app.bc

For more information try --help
error: Found argument '�' which wasn't expected, or isn't valid in this context

USAGE:
    app.bc

For more information try --help
arch 0.0.6thread 'main' panicked at 'unexpected invalid UTF-8 code point', /home/adreid/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-2.33.3/src/app/parser.rs:1685:46
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
KLEE: ERROR: library/panic_abort/src/lib.rs:46: abort failure
KLEE: NOTE: now ignoring this error at this location
thread 'main' panicked at 'unexpected invalid UTF-8 code point', /home/adreid/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-2.33.3/src/app/parser.rs:1685:46
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'mainthread '' panicked at 'mainunexpected invalid UTF-8 code point' panicked at '', unexpected invalid UTF-8 code point/home/adreid/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-2.33.3/src/app/parser.rs', :/home/adreid/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-2.33.3/src/app/parser.rs:1685:1685:4646

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: Found argument '' which wasn't expected, or isn't valid in this context

USAGE:
    app.bc

For more information try --help
error: Found argument '�' which wasn't expected, or isn't valid in this context
arch 0.0.6error: Found argument '��' which wasn't expected, or isn't valid in this context
```

Unfortunately, the output is a bit garbled. This is a side-effect of the way
that KLEE works, we get the output of several independent runs of the program
all interleaved.
But, despite this, some of the things that we can see though are

- some (probably legitimate) output from arch
  - `x86_64`
  - `arch 0.0.6`
- `clap-2.33.3/src/app/parser.rs:1685:46` is panicking because of a UTF-8 problem.


### Investigating the clap error

The [reported line](https://github.com/clap-rs/clap/blob/v2.33.3/src/app/parser.rs#L1685)
looks like this

``` rust
        self.did_you_mean_error(arg.to_str().expect(INVALID_UTF8), matcher, &args_rest2[..])
```

Obviously the problem here is the use of `arg.to_str().expect(...)` which will panic if
`arg` is not a legal UTF-8 string.
Is this a bug?
If you think that errors in user input should never result in a panic, it is a bug.
If you think that it is sufficient to detect and report user input errors, then it is not a bug.
I think it is up to the developers and users of the clap library to decide what they want
to do here.

To get a bit more information, let's look at a stack dump that KLEE saved (I have elided parts of
the stack dump).

In this case, this just confirms that test000003 involves the clap problem

```
$ ls klee-list/*.err
klee-last/test000002.ptr.err  klee-last/test000003.abort.err

$ cat klee-last/test000003.abort.err
Error: abort failure
File: library/panic_abort/src/lib.rs
Line: 46
assembly.ll line: 9201
State: 2875
Stack:
	#000009201 in _ZN11panic_abort18__rust_start_panic5abort17ha6bf52af8894b7d1E () at library/panic_abort/src/lib.rs:46
...
	#800118975 in _ZN4clap3app6parser6Parser14parse_long_arg17h74eb316bc08e4b00E (=94462742846208, self=94462677901952, matcher=94462700423712, full_arg.0=94462676176056, full_arg.1=3, it=94462712847040) at /home/adreid/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-2.33.3/src/app/parser.rs:1685
...
	#1900003442 in main (=8, =94462604547424, =94462604547496) at runtime/POSIX/klee_init_env.c:245
```

Knowing that test000003 is the test we want, we can look at the values that triggered the problem.
(Since the error involve argument parsing, I omit parts that are concerned with file contents.)


```
$ ktest-tool klee-last/test000003.ktest
ktest file : 'klee-last/test000003.ktest'
args       : ['app.bc', '--sym-args', '0', '3', '10', '--sym-files', '2', '8']
num objects: 7
object 0: name: 'n_args'
object 0: size: 4
object 0: data: b'\x01\x00\x00\x00'
object 0: hex : 0x01000000
object 0: int : 1
object 0: uint: 1
object 0: text: ....
object 1: name: 'arg00'
object 1: size: 11
object 1: data: b'--\xe0\x00\xff\xff\xff\xff\xff\xff\xff'
object 1: hex : 0x2d2de000ffffffffffffff
object 1: text: --.........
```

This shows this test was using a single argument `arg00` that has the value '--\xe0\x00\xff\xff\xff\xff\xff\xff\xff'

Given this value, we can use the shell command `printf ' --\xe0\x00\xff\xff\xff\xff\xff\xff\xff')`
to generate this value and us that to reproduce the error[^leading-space].
And we can use this to reproduce the error

[^leading-space]:
    Note the leading space in the format string to printf:
    `printf ' --\xe0\x00\xff\xff\xff\xff\xff\xff\xff')`.
    This prevents printf from interpreting the format string as a flag.


```
$ cargo run $(printf ' --\xe0\x00\xff\xff\xff\xff\xff\xff\xff')
bash: warning: command substitution: ignored null byte in input
thread 'main' panicked at 'unexpected invalid UTF-8 code point', /cargo/registry/src/github.com-1ecc6299db9ec823/clap-2.33.0/src/app/parser.rs:1632:26
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Summary

This has been a short sketch of how to use KLEE with the Rust version of CoreUtils.

We have found one possible bug associated with UTF-8 handling.

We have not attempted to test all of the tools in CoreUtils.
If you want to try that, I recommend two things:

1. Write some scripts to automate some of the manual steps above.
2. Read [KLEE's page about the GNU coreutils experiments](https://klee.github.io/docs/coreutils-experiments/)[^only-a-subset].


Enjoy!


--------

[Crux-MIR]:                       https://github.com/GaloisInc/mir-verifier/
[KLEE]:                           https://klee.github.io/
[KLEE status]:                    {{site.baseurl}}{% post_url 2021-03-29-klee-status %}
[PropTest]:                       https://github.com/AltSysrq/proptest/
[Rust CoreUtils]:                 https://github.com/uutils/coreutils
[RVT git repo]:                   {{site.gitrepo}}/
[SeaHorn]:                        https://seahorn.github.io/
