# Panduza Device PicoHa DIO

## Flash your pico

First install Rust in your computer (https://www.rust-lang.org/tools/install)

```sh
rustup target install thumbv6m-none-eabi
cargo install --locked elf2uf2-rs
```

- Reset the board and keep it in reset
- then press the 'boot' button 
- release the reset

```sh
# To build and flash the pico
cd pza-dev-picoha-dio/firmware
cargo run --release --features uart0_debug
```
