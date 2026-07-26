# Rust Compilation Pipeline

When you run `cargo build`, Cargo orchestrates the journey from source code to a final executable. Cargo is the build system that invokes `rustc` with the right flags for each crate and then invokes the system linker.

## The Compilation Stages

Rust does NOT go straight from source to machine code in one step. The pipeline is:

1. **Source code (.rs) → AST** — Parsing the source text into an Abstract Syntax Tree.
2. **AST → HIR (High-level IR)** — Used for type checking, name resolution, and macro expansion. Still has Rust-specific abstractions like iterators, pattern matching, closures.
3. **HIR → MIR (Mid-level IR)** — Rust's own intermediate representation. This is where Rust-specific work happens:
   - **Borrow checking** — verifying that references don't outlive what they point to, no overlapping mutable borrows, etc.
   - **Monomorphization** — generating concrete code for each generic type instantiation (Vec\<i32\>, Vec\<String\>, etc. are separate code paths).
   - **Drop insertion** — figuring out exactly where to insert drop calls for values going out of scope (early returns, breaks, ? operator, etc.).
   - **Constant evaluation** — const folding.
4. **MIR → LLVM IR** — Handed off to LLVM. Rust does NOT have its own machine code generator; it uses LLVM as its backend (same as Clang for C/C++).
5. **LLVM IR → Object code** — LLVM optimizes and generates machine code (.o files) targeting a specific platform (x86, ARM, etc.).
6. **Object code → Linker → Final executable** — The system linker (usually `ld` or `lld`) combines object files and libraries into the final binary.

### Why MIR exists as a separate stage

- **HIR is too high-level** — still has Rust abstractions, not detailed enough to track flow of borrows through control flow.
- **LLVM IR is too low-level** — Rust concepts like ownership and lifetimes are gone. LLVM only knows pointers and memory. Borrow checking there would be impossible.
- **MIR is the sweet spot** — low enough to represent control flow precisely (loops, branches, assignments), but high enough to retain Rust's reference and ownership semantics.

## Key Distinction: Assembly vs Object Code

- **Assembly** — human-readable text (mov, add, ret, etc.). The lowest level of "source code."
- **Object code** — the binary machine encoding of assembly (.o / .obj files). Not human-readable.

They are one step apart: assembly is the text representation, object code is its binary form.

## Why Rust Compiles Slower Than C

Rust compilation is slower than C because of the work done before LLVM even sees the code:

1. **Borrow checking on MIR** — analyzing every reference's lifetime across all control flow paths. C doesn't do this at all.
2. **Monomorphization** — for every generic type instantiation, rustc generates a concrete version and runs it through the whole pipeline. C has no generics. (C++ templates do the same thing, which is why C++ is also slow to compile.)
3. **Drop insertion** — analyzing every code path to determine where to insert drop calls. Non-trivial with early returns, loops with breaks, error paths with ?.

## Cargo Build Profiles

Cargo has build profiles that trade compile time for runtime performance:

### dev profile (default for `cargo build`)
```toml
[profile.dev]
opt-level = 0      # no optimizations, fast compile, slow runtime
debug = true       # full debug info
```

### release profile (`cargo build --release`)
```toml
[profile.release]
opt-level = 3        # full optimizations, slow compile, fast runtime
lto = "fat"          # link-time optimization across crates
codegen-units = 1    # single codegen unit = better optimization, slower compile
```

### Key settings explained

- **opt-level** — LLVM optimization level (0 = none, 3 = max).
- **codegen-units** — how LLVM splits a single crate into chunks for parallel code generation.
  - Default for dev: 16 — LLVM splits the crate into 16 pieces and optimizes in parallel. Fast compile, but LLVM can't see across chunk boundaries, so cross-function optimizations like inlining are lost.
  - Setting to 1 — the entire crate is one chunk. LLVM sees everything, can inline freely, eliminate dead code across modules. Better runtime, but single-threaded for that crate, so slower compile.
  - Tradeoff: parallelism (fast compile) vs optimization quality (fast runtime).
- **lto (Link-Time Optimization)** — merges crate boundaries so LLVM can optimize across crates.
  - `lto = "fat"` — merges everything into one giant module, full cross-crate optimization. Slowest, best runtime.
  - `lto = "thin"` — parallelizable LTO, middle ground. Faster than fat, better than no LTO. Often the pragmatic choice in CI with tight time budgets.

### Comparison of LTO Settings

Link-time optimization (LTO) is a whole-program optimization technique that can
improve runtime speed by 10-20% or more, and also reduce binary size, at the cost of worse compile times. 

| LTO Value | Runtime Speed | Binary Size | Compile Time | Parallelization  |
| --- | --- | --- | --- | --- |
| false | Baseline | Baseline | Fast | High (Default)  |
| "off" | Slower | Larger | Fastest | High  |
| "thin" | Fast | Smaller | Moderate | High (Parallel)  |
| "fat" | Fastest | Smallest | Slowest | None (Single-threaded)  |


To get the absolute best performance out of your binary, combine LTO with single code generation units.
By default, Rust splits a crate into multiple pieces (codegen-units) to compile them in parallel.
Setting this value to 1 stops the split and unlocks the highest optimization potential for the linker


## Dev vs Release target directories

Dev and release builds use completely separate target directories:
- dev build → `target/debug/`
- release build → `target/release/`

Intermediate representations (HIR, MIR, LLVM IR) are NOT reused between dev and release builds. They are isolated pipelines.

## Practical Production Notes

- For development: fast iteration with dev profile (opt-level=0, many codegen units).
- For production release: max optimization with release profile (opt-level=3, lto, codegen-units=1).
- In CI with tight time budgets: `lto = "thin"` is often the pragmatic choice. `lto = "fat"` for final release artifacts where you have time.
- Use `cargo build --timings` to generate an HTML report showing how long each crate took to compile. Useful for diagnosing slow builds.

## References

- [The Rust Book | Cargo Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [Rustc Dev Guide | MIR](https://rustc-dev-guide.rust-lang.org/mir/index.html)
- [LLVM Documentation](https://llvm.org/docs/)
