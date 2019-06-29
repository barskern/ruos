# RUOS

A operating system based on [Philipp Oppermann's blog](https://os.phil-opp.com/), built as a learning project.

# Requirements

In addition to [`rustup`](https://rustup.rs/), this project has some external dependencies to compile:

```
rustup component add rust-src llvm-tools-preview
cargo install cargo-xbuild bootimage
```

Further to run the resulting binary one needs to either run it using an emulator (e.g [`qemu`](https://www.qemu.org/)) or burn it to a drive and boot it using actual hardware. The simplest solution is to have `qemu` installed and then using `cargo xrun` (a command created by `cargo-xbuild` which both compiles and runs the binary).
