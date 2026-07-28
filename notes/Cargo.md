# Cargo - The Rust Package Manager

Cargo is the Rust package manager. It is a tool that allows Rust packages to declare their various dependencies and ensure that you’ll always get a repeatable build.

To accomplish this goal, Cargo does four things:

1. Introduces two metadata files with various bits of package information.
2. Fetches and builds your package’s dependencies.
3. Invokes rustc or another build tool with the correct parameters to build your package.
4. Introduces conventions to make working with Rust packages easier.

---
## Use cargo instead of rustc

To compile the classic “hello world” program, we use the following command:

```rust
$ rustc hello.rs
$ ./hello
Hello, world!
```
Note that the above command required that you specify the file name explicitly. We also need to manage dependencies ourselves. 

Rather than invoke `rustc` directly, you can instead invoke something generic such as `cargo build` and let cargo worry about constructing the correct `rustc` invocation.

```rust
# Compile the program. Binary crate created in ./target/debug folder
$ cargo build

# Now we can run the code
$ ./target/debug/hello_world

# Compile the program with Production optimization. Binary crate created in ./target/release folder
$ cargo build --release

```

For dev builds, we can use the `cargo run` command that compiles and runs the program in one step.

Furthermore, Cargo will automatically fetch any dependencies you have defined for your artifact from a registry, and arrange for them to be added into your build as needed.

---

## Cargo.toml

`Cargo.toml` is a manifest file that contains all of the metadata that Cargo needs to compile your package. This file is written in the TOML format.

```Cargo.toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2024"

[dependencies]

```
---

## Cargo.toml vs Cargo.lock

`Cargo.toml` and `Cargo.lock` serve two different purposes. Before we talk about them, here’s a summary:

1. Cargo.toml is about describing your dependencies in a broad sense, and is written by you.
2. Cargo.lock contains exact information about your dependencies. It is maintained by Cargo and should not be manually edited.
3. When in doubt, check Cargo.lock into the version control system (e.g. Git).

---

## Using a Faster Linker: lld (LLVM linker)

By default, Rust uses the system linker (`ld` on Linux, `ld64` on macOS, `link.exe` on Windows). For large projects the linker can dominate build time. Switching to `lld` — LLVM's linker — can significantly reduce link time.

### Why lld is faster

- `lld` is multi-threaded; the default `ld` (GNU ld / ld64) is largely single-threaded.
- `lld` is purpose-built for LLVM-compiled code, which is exactly what rustc produces.
- In debug builds especially, where code generation is fast but linking is still a single serial step, switching the linker often yields the biggest single speedup.

### How to configure it

There are two approaches:

#### Approach 1: Set the linker in .cargo/config.toml

Create `.cargo/config.toml` in your project (or `~/.cargo/config.toml` globally):

```toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

This tells rustc to use `clang` as the linker driver and pass `-fuse-ld=lld` so clang invokes `lld` under the hood. (Using clang as the driver is the simplest way to select lld on Linux.)

On macOS, `lld` is not the default — Apple's `ld64` is used. You generally keep `ld64` or use the `ld64.lld` variant:

```toml
[target.aarch64-apple-darwin]
rustflags = ["-C", "linker-flavor=ld.lld", "-C", "linker=ld64.lld"]

```

Note: macOS lld support is still maturing — test before committing to it.

#### Approach 2: Use the rust-lld wrapper via RUSTFLAGS

```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
linker = "rust-lld"
```

`rust-lld` is a wrapper shipped with rustup's LLVM toolchain. It selects the correct lld flavor for the target automatically. This is the simplest approach on Linux if your rustup component includes it (it usually does).

### Verifying it works

```bash
# Build and check which linker was used
cargo build --timings
# or inspect the build output
cargo build -v 2>&1 | grep linker
```

If you see `-fuse-ld=lld` or `rust-lld` in the rustc invocation, it's working.

### Trade-offs

- `lld` produces functionally identical binaries — it is not a downgrade in binary quality.
- On Linux, you need `lld` installed (e.g., `apt install lld` or `dnf install lld`). The `rust-lld` wrapper from rustup bundles it, so that's the easier path.
- On macOS, `ld64` is already fast and well-integrated; `lld` support is less battle-tested. Measure before switching.
- On Windows, `lld-link` is available and is the default when using the `x86_64-pc-windows-msvc` target with recent toolchains.

---

