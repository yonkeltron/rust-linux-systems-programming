---
marp: true
---

# Linux systems programming with Rust

## Jonathan E. Magen

### PLUG: Philly Linux User Group

![bg left](https://source.unsplash.com/EWLBteE2o5o)

---

# Who is this guy?

- Principal Computer Scientist
- 6 years at current company (healthcare)
- 5 years in startups before that
- Linux user since early middle school

![bg right](https://source.unsplash.com/EwKXn5CapA4)

---

# <!-- fit --> Why are we here?

---

# <!-- fit --> To learn about Rust and Linux!

![bg](https://source.unsplash.com/bWiH0PDTB-g)
![](white)

---

# Agendanomics

- Intro to Rust
- Linux systems programming
- Examine a real application
- Q&A

![bg left](https://source.unsplash.com/boE2xft0cAo)

---

# <!-- fit --> Rust :heart: Linux

---

# <!-- fit --> Rust :heart: Windows

---

# <!-- fit --> Rust :heart: macOS

---

# <!-- fit --> Ok, sure.

---

# No, seriously. Rust is fantastic!

- Originally developed at Mozilla
- Used by many, including AWS
- Designed with some very novel features

---

# Novel features of Rust

- Safety
- Ergonomics
- Efficiency

![bg right](https://source.unsplash.com/oax_efzQ9FI)

---

# Safety

- Affine types
  - From [affine logic](https://en.wikipedia.org/wiki/Affine_logic), a substructural logic
  - Values may be used at most once
- Borrow checker
  - Makes sure your code doesn't use values it shouldn't
  - Higher learning curve
  - Added to D, being added to Swift
- Compile-time memory management with lifetimes
  - Compiler does the hard work for you
  - Fine-grained control, without `malloc` and `free` details.
- No `null` or equivalent, `Option<T>` instead

---

# Ergonomics

- Best compiler I've ever worked with
  - Fantastic error messages
  - A bit slow, though
- Incredible attention to programmer productivity
  - Pattern matching
  - Expressions
  - Macros
- Objects but no inheritance
  - Traits
- Compiler-checked errors with `Result<T, E>`

---

# Efficiency

- Zero-cost abstractions
  - You don't pay for what you don't use
- Optimizing compiler
  - Slow because it does a LOT!
- Speed, relative to C: ~90%
  - Common Lisp: ~80%
  - Go: ~70%
- Concurrency and parallelism programming
  - Threads (stdlib)
  - Futures (`async` and `await`)
  - Actors ([Riker](https://riker.rs/), [spaad](https://lib.rs/crates/spaad), others...)

---

# <!-- fit --> Sounds good.

---

# <!-- fit --> Yes. It is pretty good.

---

# <!-- fit --> So where does Linux come in?

---

# Lots of Linux software being written in Rust

- [vopono](https://github.com/jamesmcm/vopono) Manage per-app VPN tunnels
- [kmon](https://github.com/orhun/kmon) Linux kernel monitor + activity
- [lfs](https://github.com/Canop/lfs) Linux filesystem info tool

![bg left](https://source.unsplash.com/jh8iVTrMfHQ)

---

# Helpful Rust crates (libraries) for systems programming

Some of my favorites:

- [libc](https://crates.io/crates/libc) - Foreign-Function Interface (FFI)
- [procfs](https://crates.io/crates/procfs) - Interface to `/proc`
- [caps](https://crates.io/crates/caps) - Linux capabilities
- [redbpf](https://github.com/redsift/redbpf) - Build & run BPF/eBPF modules

![bg right](https://source.unsplash.com/8dvTZPVEJWk)

---

# <!-- fit --> Let's look at some code!

---

# <!-- fit --> First thing's first:

# <!-- fit --> Meet `cargo`!

# <!-- fit --> (Cargo is Rust's build tool.)

![bg left](https://source.unsplash.com/xewrfLD8emE)

---

# Crates we will use

- [color-eyre](https://crates.io/crates/color-eyre) for pretty error handling
-
