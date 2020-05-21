# mac-brightness

A Rust port of the tools to control Mac brightness from [pirate's repo](https://github.com/pirate/mac-keyboard-brightness).

## Be aware
- This is a work in progress
- Only works on pre-2015 Macs

## Compilation
### Dependencies
If `bindgen` is not installed, then install with:
```
cargo install --force bindgen
```

### Generate third-party C library headers for our Rust library
```
cd keyboard-brightness/src
bindgen --ctypes-prefix=cty bindings.h > bindings.rs
```

### Build the binary
It's recommended to disable warnings as the generated `bindings.rs` prompts a lot of warnings.
```
cd keyboard-brightness
RUSTFLAGS=-Awarnings cargo build --release
```

## Usage
### Set keyboard brightness
Set the brightness with a float value between 0 and 1
./keyboard-brightness 0.5

### Get keyboard brightness
./keyboard-brightness