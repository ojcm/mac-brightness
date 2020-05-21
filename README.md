# mac-brightness

A Rust port of the tools to control Mac brightness from [pirate's repo](https://github.com/pirate/mac-keyboard-brightness).

## Be aware
- Display brightness has not been ported.
- Only works on pre-2015 Macs
- Use at your own risk etc.

## Compilation
### Build the binary
It's recommended to disable warnings as the generated C bindings prompt a lot of warnings.
```
cd keyboard-brightness
RUSTFLAGS=-Awarnings cargo build --release
```

## Usage
### Set keyboard brightness
Set the brightness with a float value between 0 and 1
```
./keyboard-brightness 0.5
```

### Get keyboard brightness
```
./keyboard-brightness
```