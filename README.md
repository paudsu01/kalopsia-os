# Kalopsia-OS minimal development Setup
1. Clone this repo
2. Setup rust nightly: `rustup default nightly`. You can change it back to stable with `rustup default stable`.
3. Install `QEMU` emulator
4. Install bootimage `cargo install bootimage`. Bootimage will compile the bootloader and the kernel and link the bootloader with the kernel ELF file 
5. To run the kernel in qemu: `cargo run`

## Notes:
`cargo run` behind the scences is just doing the following:
1. `cargo bootimage`
2. `qemu-system-x86_64 -drive format=raw,file=target/kalopsia-os-x86-64/debug/bootimage-kalopsia-os.bin`
