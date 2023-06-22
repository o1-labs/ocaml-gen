[![CI](https://github.com/o1-labs/ocaml-gen/actions/workflows/ci.yml/badge.svg)](https://github.com/o1-labs/ocaml-gen/actions/workflows/ci.yml)
[![dependency status](https://deps.rs/repo/github/o1-labs/ocaml-gen/status.svg?style=flat-square)](https://deps.rs/repo/github/o1-labs/ocaml-gen)
​[![Crate](https://img.shields.io/crates/v/badgen.svg)](https://crates.io/crates/ocaml-gen)
[![Docs](https://docs.rs/badgen/badge.svg)](https://o1-labs.github.io/ocaml-gen/)
# OCaml-gen

## Motivation

[`ocaml-rs`](https://github.com/zshipko/ocaml-rs) provides functions and
wrappers to interact with the OCaml runtime which implements a garbage collector
to manage memory automatically. The library allows developers to write functions
in Rust that can be called from OCaml and vice versa. It provides automatic
conversion between OCaml and Rust value representations. The users can use
macros on the Rust side like `ocaml::FromValue` and `ocaml::ToValue` to convert
back and forth the values without thinking about the garbage
collector.
Macros `ocaml::func` in conjunction with `ocaml::sig` can be used on functions
to generate codes which will be compatible with the OCaml runtime.
More information is available in the [ocaml-rs book](https://zshipko.github.io/ocaml-rs/).

Even if `ocaml-rs` provides some macros, the user will need to write the OCaml
definitions with the corresponding types and use external definitions. Also,
macros to access nested values in structures are not provided in `ocaml-rs`.
The goal of `ocaml-gen` and `ocaml-gen-derive` is to provide automatic binding
generations and to add macros easing the development of large applications.
It is meant to be used in conjunction with [`ocaml-rs`](https://github.com/zshipko/ocaml-rs).

**SECURITY WARNING: this is still an experimental library, you should verify that the bindings generated are correct if you are using this in production**.

See the [ocaml-gen/README](/ocaml-gen) for more information.

## Setup development environment

```
opam switch create ./ 4.14.0
opam install merlin ocamlformat.$(awk -F = '$1 == "version" {print $2}' .ocamlformat)
```

## Run tests

```
dune build @runtest
```

If you change the file `tests/ocamlgen_test_stubs/src/bin/main.rs` or anything related to code generation, you will need to update `tests/expected_bindings.ml`. You can use:
```
dune build @runtest --auto-promote
```
to rely on `dune` to update the file. You will need to commit it to make the CI happy.

## Organization

* [ocaml-gen](ocaml-gen): the tool that allows us to generate the OCaml bindings from the Rust code.
* [ocaml-gen-derive](ocaml-gen/derive): derive macros have to be a in separate crate, so they are here. This crate is re-exported by ocaml-gen so end users should not have to worry about it.
* [tests/](tests/): contains some tests. If you are looking for examples on how to use ocaml-gen, you can check that folder.

## Additional resources

You can check the [recording](https://www.youtube.com/watch?v=LuXo2cNkgyA&feature=youtu.be) I made when I first introduced the tool internally.

## Publish a new release

An automatic release is set up with GitHub actions. To make a new release of
ocaml-gen and ocaml-gen-derive:

1. Change the version of `ocaml-gen-derive` in `ocaml-gen/Cargo.toml` to the new
   release version.
2. Create a tag with `git tag -m "VERSION" -a "x.y.z"`
3. Push the tag with `git push --tags`
