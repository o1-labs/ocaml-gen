# Ocaml-gen tests

There's an obvious lack of tests in this library. But here's some foundational intro on how we could improve this situation:

* `cargo test` should test "I want the bindings to look like this, does it?" and make use of [`src/lib.rs`](src/lib.rs).
* `dune test` should test the usage of these bindings in ocaml-land. It should use [`src/main.rs`](src/main.rs) to generate the bindings and test them from [`src/lib.ml`](src/lib.ml).

Ideally the ocaml test package should be re-created with esy, because we don't fix an ocaml version at the moment.
