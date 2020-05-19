# mac-brightness

A Rust port of the tools to control Mac brightness from [pirate's repo](https://github.com/pirate/mac-keyboard-brightness).

## Be aware
- This is a work in progress
- Only works on pre-2015 Macs

## Compilation
### Dependencies
bindgen, cbindgen.  Install with
```
cargo install --force bindgen
cargo install --force cbindgen
```

### Generate third-party C library headers for our rust library
```
cd keyboard-brightness/src
bindgen --ctypes-prefix=cty bindings.h > bindings.rs
```

### Build the library
```
cd keyboard-brightness
cargo build --release
```

### Build the header file
```
cd keyboard-brightness
cbindgen --crate keyboard-brightness --output rust_keyboard_brightness.h --config cbindgen.toml
```

### Compile the C source with rust library
```
gcc -I./keyboard-brightness/ -o kbrightness keyboard-brightness.c ./keyboard-brightness/target/release/libkeyboard_brightness.a -framework IOKit -framework ApplicationServices
```