# mac-brightness

A Rust port of the tools to control Mac brightness from [pirate's repo](https://github.com/pirate/mac-keyboard-brightness).

## Be aware
- This is a work in progress
- Only works on pre-2015 Macs

## Compilation
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