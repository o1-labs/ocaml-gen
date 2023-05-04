[![CI](https://github.com/o1-labs/ocaml-gen/actions/workflows/ci.yml/badge.svg)](https://github.com/o1-labs/ocaml-gen/actions/workflows/ci.yml)
[![dependency status](https://deps.rs/repo/github/o1-labs/ocaml-gen/status.svg?style=flat-square)](https://deps.rs/repo/github/o1-labs/ocaml-gen)
​[![Crate](https://img.shields.io/crates/v/badgen.svg)](https://crates.io/crates/ocaml-gen)
[![Docs](https://docs.rs/badgen/badge.svg)](https://o1-labs.github.io/ocaml-gen/)
# Ocaml-gen

This Rust library allows you to automatically generate OCaml bindings for your Rust code. It is meant to be used in conjunction with [`ocaml-rs`](https://github.com/zshipko/ocaml-rs).

**SECURITY WARNING: this is still an experimental library, you should verify that the bindings generated are correct if you are using this in production**.

See the [ocaml-gen/ README](/ocaml-gen) for more information.

## Setup development environment

```
opam switch create ./ 4.14.0
opam install merlin ocamlformat.$(awk -F = '$1 == "version" {print $2}' .ocamlformat)
```

## Organization

* [ocaml-gen](ocaml-gen): the tool that allows us to generate the OCaml bindings from the Rust code.
* [ocaml-gen-derive](ocaml-gen/derive): derive macros have to be a in separate crate, so they are here. This crate is re-exported by ocaml-gen so end users should not have to worry about it.
* [tests/](tests/): contains some tests. If you are looking for examples on how to use ocaml-gen, you can check that folder.

## Additional resources

you can check the [recording](https://www.youtube.com/watch?v=LuXo2cNkgyA&feature=youtu.be) I made when I first introduced the tool internally.
