---
id: install
slug: /install
title: Install
---

## Application Setup

*Get your tools*

----

We’ll start by creating a new application with cargo, and setting it to use
nightly Rust.

```bash
$ cargo new --bin teensy
$ cd teensy
$ rustup override set nightly
```

We will need the `rust-src` crate for several functions. Now move over to
[Compiling and Linking](arduino/index.md)

```bash
$ rustup +nightly component add rust-src
```

