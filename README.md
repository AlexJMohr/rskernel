# x86_64 Kernel Project

## Licence

GPL 3.0 or any later version

## Install dependencies
```sh
rustup component add llvm-tools-preview rust-src
cargo install bootimage
```

## Run with QEMU

```sh
cargo run
```

## Progress

Following this guide: https://os.phil-opp.com/

### Bare Bones

- [x] A Freestanding Rust Binary
- [x] A Minimal Rust Kernel
- [x] VGA Text Mode
- [x] Testing

### Interrupts

- [ ] CPU Exceptions
- [ ] Double Faults
- [ ] Hardware Interrupts

### Memory Management

- [ ] Introduction to Paging
- [ ] Paging Implementation
- [ ] Heap Allocation
- [ ] Allocator Designs

### Multitasking

- [ ] Async/Await

After that:

- [ ] Multiboot2 header
- [ ] UEFI

