# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ocaml-gen is a Rust library that automatically generates OCaml bindings from Rust code. It works alongside `ocaml-rs` to eliminate the need for manually writing OCaml type and function declarations. Users annotate Rust types/functions with macros, then run a binary that outputs the corresponding `.ml` file.

## Build Commands

Use Makefile targets for standard operations:

```bash
make build              # build the project
make release            # build in release mode
make lint               # run clippy
make format             # format Rust/TOML (requires nightly)
make check-format       # check all formatting (Rust/TOML/OCaml)
make setup-ocaml-deps   # install OCaml dependencies
```

Tests and OCaml formatting use dune directly:

```bash
dune build @runtest             # run tests
dune build @runtest --auto-promote  # update expected_bindings.ml after code gen changes
dune fmt --auto-promote         # auto-fix OCaml formatting
```

## Development Environment Setup

```bash
opam switch create ./ 4.14.0
opam install merlin ocamlformat.0.28.1
```

## Architecture

**Workspace Structure:**
- `ocaml-gen/` - Main library crate with `Env` for tracking declarations, traits (`OCamlBinding`, `OCamlDesc`), and macros (`decl_module!`, `decl_type!`, `decl_func!`)
- `ocaml-gen/derive/` - Proc-macro crate (re-exported by ocaml-gen) providing derive macros: `#[ocaml_gen::Struct]`, `#[ocaml_gen::Enum]`, `#[ocaml_gen::CustomType]`, `#[ocaml_gen::func]`
- `tests/ocamlgen_test_stubs/` - Test crate demonstrating usage and generating test bindings

**How Code Generation Works:**
1. Rust types are annotated with `#[ocaml_gen::Struct]` or `#[ocaml_gen::CustomType]`
2. Functions are annotated with `#[ocaml_gen::func]` (placed before `#[ocaml::func]`)
3. A binary uses `Env` and macros to write bindings to stdout: `decl_module!`, `decl_type!`, `decl_func!`
4. Dune captures the output as `bindings.ml` and builds OCaml code against it

**Test Flow (`tests/dune`):**
- Runs `cargo build --release` to create `libocamlgen_test_stubs.a`
- Runs `cargo run --bin main` to generate `bindings.ml`
- Compares `bindings.ml` against `expected_bindings.ml` (diff test)
- Runs `ocamlgen_test.exe` (OCaml tests using alcotest/qcheck)

## Key Files

- `tests/ocamlgen_test_stubs/src/bin/main.rs` - Binding generation entry point
- `tests/ocamlgen_test_stubs/src/lib/lib.rs` - Annotated Rust types/functions
- `tests/expected_bindings.ml` - Reference bindings (commit changes after `--auto-promote`)

## Development Guidelines

**Commit requirements:**
- Never add co-author lines to commits
- Never include emojis in commit messages

**Before committing:**
- Run `make format` for Rust/TOML files
- Run `dune fmt --auto-promote` for OCaml files
