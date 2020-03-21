# rust-webasm
Basic WebAssembly using Rust

## Installation 

If you have MacOS: `brew install rust `

Path Stuff in your bash_profile : `$PATH:/usr/local/Cellar/rust/1.41.1/bin`

else, just install from 

`$ curl https://sh.rustup.rs -sSf | sh`

>verify
```
$ rustc --version
rustc 1.41.1

$ cargo --version
cargo 1.41.0

$ cargo update // updates the dependency versions
```


## Hello World Rust
```
cd hello_world
rustc main.rs
./main
```

## Cargo Rust
```
$ cargo new hello_cargo

$ cargo build
    Compiling hello_cargo v0.1.0 (github.com/shreyasgune/rust-webasm/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.53s


$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
    Running `target/debug/hello_cargo`

Hello, cargo!

$ cargo check
    Checking hello_cargo v0.1.0 (github.com/shreyasgune/rust-webasm/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.40s

$ cargo build --release
    Compiling hello_cargo v0.1.0 (github.com/shreyasgune/rust-webasm/hello_cargo)
    Finished release [optimized] target(s) in 0.52s
```

## WASM Hello World

### Pre-requisites 
```
cargo install wasm-pack

```
@e3aOv59*D2Q