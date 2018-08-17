# Rust Function Invoker

The _Rust function invoker_, or simply the _Rust invoker_, loads and invokes a Rust function packaged as a shared library.
The Rust invoker is a binary executable which listens on port 8080 and, for each HTTP request received, calls the
function and responds with a HTTP response.

Rust functions which are callable by the Rust invoker must conform to one of the function types in the `function_types` directory.

The `samples` directory contains sample functions written in Rust.

To build the Rust invoker and the sample functions, [install Rust](https://www.rust-lang.org/en-US/install.html),
clone this repository, change directory to the clone, and issue the following:
```bash
$ cargo build --release
```
To try the invoker, issue:
```bash
$ target/release/rust_invoker target/release/libhello.dylib
```
and then, from another terminal window:
```bash
$ curl localhost:8080 -d Rust
hello Rust
```

## Supported Function Types

So that function shared libraries do not have to be compiled with the same version of Rust as was used to compile the function invoker and since Rust does not have a [define ABI](https://github.com/rust-lang/rfcs/issues/600), functions must conform to the C ABI: function declared as `extern "C"`; parameter types declared as `#[repr(C)]`.

### Basic

The `Basic` function type 
