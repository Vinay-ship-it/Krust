# Krust 

A minimal, freestanding x86_64 operating system kernel written in Rust for bare metal, following Philipp Oppermann's [Writing an OS in Rust](https://os.phil-opp.com/) blog series.

## Technical Features

- **Freestanding Binary**: Utilizes `#![no_std]`, `#![no_main]`, and a custom `_start` entry point.
- **VGA Text-Mode Driver**: A `Writer` over the `0xb8000` VGA buffer featuring 16-color support and standard formatting macros (`print!`, `println!`).
- **Serial Output**: UART 16550 driver for logging to the host console via QEMU, utilizing `serial_print!` and `serial_println!` macros.
- **Global Descriptor Table & TSS**: Sets up a Task State Segment with a dedicated interrupt stack specifically for double faults.
- **Interrupt Descriptor Table**: Contains handlers for CPU exceptions (breakpoint, double fault, page fault) and hardware interrupts via a chained 8259 PIC (timer and PS/2 keyboard with scancode decoding).
- **Custom Test Framework**: A `#![no_std]`-compatible test runner that exits QEMU with a status code via the `isa-debug-exit` device for headless `cargo test` execution.
- **Panic Handling**: Tailored operational flows explicitly separated for normal boot sequences versus test runs.

## Prerequisites

Krust requires a nightly Rust toolchain, specific components, and the `bootimage` tool.

```bash
rustup component add rust-src llvm-tools-preview
cargo install bootimage
```

You will also need QEMU to emulate the x86_64 hardware environment:

| Operating System | Installation Command |
|---|---|
| Ubuntu / Debian | `sudo apt install qemu-system-x86` |
| macOS | `brew install qemu` |
| Windows | `winget install qemu` |

## Build & Run

Compile the kernel for the custom `x86_64-krust.json` target, wrap it into a bootable image, and launch it in QEMU:

```bash
cargo run
```

## Testing

Krust utilizes a custom test harness to boot tests in QEMU, report results over the serial port, and exit via a special debug-exit device. A successful QEMU test run exits with code `33`.

```bash
cargo test
```

**Test Suite Coverage:**

- Unit tests embedded within `src/` for core functionality validation.
- `tests/basic_boot.rs` validates standard kernel boot and print capabilities.
- `tests/should_panic.rs` ensures deliberately failing assertions panic appropriately.
- `tests/stack_overflow.rs` triggers a stack overflow to verify the double-fault handler catches it via the TSS-backed interrupt stack.

## Boot Sequence

1. The `bootloader` crate sets up long mode, configures paging, and calls `_start` in `main.rs`.
2. The `krust::init()` function loads the GDT and TSS, configures a dedicated double-fault stack, loads the IDT, remaps the 8259 PIC, and enables interrupts.
3. The kernel outputs startup diagnostics and enters a low-power `hlt_loop()`, waking only to service hardware interrupts.

## Roadmap

- Physical and virtual memory management alongside a heap allocator.
- Multitasking with cooperative or preemptive scheduling.
- A basic filesystem implementation.
- Comprehensive keyboard/PS2 handling and expansion of peripheral device drivers.

## Acknowledgments

Krust was built following Philipp Oppermann's excellent *Writing an OS in Rust* blog series. Huge thanks to him for providing an exceptional practical resource for learning bare-metal OS development in Rust.
