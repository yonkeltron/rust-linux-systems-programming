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
- Examine a real application: Hermione
- Q&A with co-maintainers

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
- [nix](https://crates.io/crates/nix) - Friendlier \*nix bindings
- [procfs](https://crates.io/crates/procfs) - Interface to `/proc`
- [caps](https://crates.io/crates/caps) - Linux capabilities
- [redbpf](https://github.com/redsift/redbpf) - Build & run BPF/eBPF modules

![bg right](https://source.unsplash.com/8dvTZPVEJWk)

---

# <!-- fit --> Let's look at some code!

---

# We will first build a Linux process viewer!

---

# <!-- fit --> First thing's first:

# <!-- fit --> Meet `cargo`!

# <!-- fit --> (Cargo is Rust's build tool.)

![bg left](https://source.unsplash.com/xewrfLD8emE)

---

# Crates we will use

- [color-eyre](https://crates.io/crates/color-eyre) for pretty error handling
- [procfs](https://crates.io/crates/procfs) - For interfacing with `/proc`
- [paris](https://crates.io/crates/paris) - For stylish output

---

# Add our dependencies to the `Cargo.toml` file

```toml
[dependencies]
color-eyre = "0.5"
paris = "1.5"
procfs = "0.9"
```

---

# Add code to our project

```rust
use color_eyre::eyre::Result;
use paris::Logger;

pub fn view_procs() -> Result<()> {
  let mut logger = Logger::new();

  logger.info("Starting up!").newline(1).log("Processes:");

  procfs::process::all_processes()?
    .into_iter()
    .map(|process| {
      format!(
        "{}: {} - {} bytes",
        process.pid, process.stat.comm, process.stat.vsize
      )
    })
    .for_each(|process_message| {
      logger.indent(1).info(process_message);
    });

  Ok(())
}
```

---

# <!-- fit --> Let's break this down!

---

# Preamble and first bits

```rust
// Imports
use color_eyre::eyre::Result; // Error handling
use paris::Logger; // Stylish logging

// The primary function is fallible and so returns a Result
pub fn view_procs() -> Result<()> {
```

---

# Logging some output

```rust
  // New up a logger, which is marked as mutable with mut
  let mut logger = Logger::new();
  // Emit some friendly output to the terminal
  logger.info("Starting up!").newline(1).log("Processes:");
```

---

# The `?` operator either returns the contents of the `Result` or short circuits by bubbling up the error to the calling function!

---

# The guts of the process viewer

```rust
 procfs::process::all_processes()? // Grab all processes
    .into_iter() // Get them in an iterator
    .map(|process| { // Map processes to Strings
      format!(
        "{}: {} - {} bytes", // Grab the PID, name, and memory usage
        process.pid, process.stat.comm, process.stat.vsize
      )
    }) // Log each string!
    .for_each(|process_message| {
      logger.indent(1).info(process_message);
    });
```

---

# Close it out, bring it home

```rust
  // Signal that it all went well
  Ok(()) // Note: no semicolon means a return expression
}
```

# Walla! We're done!

---

# <!-- fit --> Less than 25 lines, with spaces!

---

# <!-- fit --> It doesn't have to _feel_ low-level to _be_ low-level.

---

# <!-- fit --> Rust usually feels high-level.

---

# <!-- fit --> Ok. Now what?

---

# Next, let's explore the wide world of filesystem event notifications provided by [inotify](https://man7.org/linux/man-pages/man7/inotify.7.html)!

![bg left](https://source.unsplash.com/xv7-GlvBLFw)

---

# <!-- fit --> `inotify(7)` is money, but confusing!

---

# The `nix` crate makes it much simpler, though!

---

# Let's write a little inotify program which watches for filesystem changes.

---

```rust
use color_eyre::eyre::Result;
use nix::sys::inotify;
use paris::Logger;

pub fn setup_watcher(path_str: &str) -> Result<bool> {
  let watcher = inotify::Inotify::init(inotify::InitFlags::empty())?;
  let watch = watcher.add_watch(path_str, inotify::AddWatchFlags::IN_ALL_EVENTS)?;

  let mut logger = Logger::new();
  let mut go = true;

  while go {
    logger.newline(1).loading("Waiting for events...");
    let events = watcher.read_events()?;
    logger.info(format!("Got {} events", events.len()));

    for event in events {
      let msg = format!("Event: {:?} for {:?}", event.mask, event.name);
      logger.indent(1).log(msg);
    }
  }

  watcher.rm_watch(watch)?;

  Ok(go)
}
```

---

# <!-- fit --> Again, we'll break this down!

---

```rust
// Create our function which takes a path as a string slice
pub fn setup_watcher(path_str: &str) -> Result<bool> {
  // Initialize our watcher
  let watcher = inotify::Inotify::init(inotify::InitFlags::empty())?;
  // Create the watch!
  let watch = watcher.add_watch(path_str, inotify::AddWatchFlags::IN_ALL_EVENTS)?;

```

---

```rust
  // New up a logger
  let mut logger = Logger::new();
  // Set a stop variable
  let mut go = true;

  // Loop until not go
  while go {
    logger.newline(1).loading("Waiting for events...");
    // Read events from the queue, otherwise block!
    let events = watcher.read_events()?;
    logger.info(format!("Got {} events", events.len()));
```

---

```rust
    // Loop over events
    for event in events {
      // Make a nice message
      let msg = format!("Event: {:?} for {:?}", event.mask, event.name);
      // Print it out
      logger.indent(1).log(msg);
    }
  }

  // Clean up our watch just in case
  watcher.rm_watch(watch)?;

  // All done!
  Ok(go)
}
```

---

# Problems with this inotify example

1. The `go` variable will always be `true`.
1. It is an overly-broad watch (`IN_ALL_EVENTS`)!
1. It doesn't traverse the directory tree.

Try to ignore these. Work with me, here.

---

# <!-- fit --> Ok. So.

---

# <!-- fit --> Systems Programming!

---

# <!-- fit --> It doesn't have to be painful!

---

# Recap: systems programming with Rust

- Doesn't have to feel low-level to be low-level.
- Excellent ecosystem of crates.
- Versatile interfaces to existing libraries.

![bg left](https://source.unsplash.com/P2ZokcpVrik)

---

# Stuff we didn't cover

- Command-line interfaces
  - The [clap](https://crates.io/crates/clap) crate is exceptional
- Notifications
  - Check out the [notify_rust](https://crates.io/crates/notify_rust) crate for great functionality
- Async programming
  - I am a big fan of [async-std](https://crates.io/crates/async-std)

---

# <!-- fit --> Enter: Hermione

![bg right](https://source.unsplash.com/IV6GyJkiHfg)

---

# Competent magic for your config files and more!

## A package manager for your config files?

---

# Hermione features

- Full Rust CLI
  - Portable across Linux, macOS, and Windows
- Integrated package scaffolding and utilities
- Repository support
- Package hooks

![bg left](https://source.unsplash.com/AJ_Mou1FUS8)

---

# <!-- fit --> Check us out at `https://hermione.dev`

![bg blur](https://source.unsplash.com/0V7_N62zZcU)
![](white)

---

# I want to introduce Egli Hila

- One of the best software engineers I know
- Co-maintainer of Hermione
- A real swell fella

---

# <!-- fit --> Demo!

![bg](darkblue)
![](white)
