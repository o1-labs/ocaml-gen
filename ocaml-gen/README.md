# OCaml-gen

This crate provides automatic generation of OCaml bindings.
Refer to the rustdoc for more information.

## Binding generation

Here's an example of generating some bindings:

```rust
// initialize your environment
let env = &mut Env::default();

// choose where you want to write the bindings
let w = &mut std::io::stdout();

// we need to create fake generic placeholders for generic structs
decl_fake_generic!(T1, 0);
decl_fake_generic!(T2, 1);

// you can declare modules
decl_module!(w, env, "Types", {
    
    // and types
    decl_type!(w, env, SomeType);

    // and even generic types
    decl_type!(w, env, SomeGenericType::<T1>);

    // you can also rename a type
    decl_type!(w, env, SomeOtherGenericType::<T1> => "thing");
});

decl_module!(w, env, "ImportantType", {

    // the OCaml way is often to rename your module's main type as `t`
    decl_type!(w, env, CamlBigInteger256 => "t");

    // you can declare functions as well
    // note that the underlying function calls `caml_of_numeral_to_ocaml`
    // so you have to either import all (e.g. `use path::*`)
    // or you have to import `caml_of_numeral_to_ocaml`
    decl_func!(w, env, caml_of_numeral => "of_numeral");
});
```

which should generate OCaml bindings that looks like this:

```ocaml
module Types = struct
  type nonrec some_type

  type nonrec 'generic_name some_generic_type

  type nonrec 'stuff thing
end

module ImportantType = struct
  type nonrec t

  val of_numeral : int -> t
end
```

Note that you can create type aliases with the [`decl_type_alias!`](https://o1-labs.github.io/ocaml-gen/ocaml_gen/macro.decl_type_alias.html) macro but it is **highly experimental**.
It has a number of issues: 

* the alias doesn't work outside of the module it is declared current scope (which is usually what you want)
* the alias is ignoring the instantiation of type parameters. This means that it might rename `Thing<usize>` to `t1`, eventhough `t1` was an alias to `Thing<String>` (this is the main danger, see [this tracking issue](https://github.com/o1-labs/ocaml-gen/issues/4))
* it won't work (binding generation will throw an error) if you try to alias two instantiations of the same generic type (for example, `t1` is the alias of `Thing<usize>` and `t2` is the alias of `Thing<String>`)

## Binding description

To allow the previous example to work, you must derive the correct functions on your types and functions.
To do that, you can use the [ocaml-gen-derive](/ocaml-gen/derive) crate.

To allow generation of bindings on structs, use [`ocaml_gen::Struct`](https://o1-labs.github.io/ocaml-gen/ocaml_gen/derive.Struct.html):

```rust
#[ocaml_gen::Struct]
struct MyType {
  // ...
}
```

To allow generation of bindings on enums, use [`ocaml_gen::Enum`](https://o1-labs.github.io/ocaml-gen/ocaml_gen/derive.Enum.html):

```rust
#[ocaml_gen::Enum]
enum MyType {
  // ...
}
```

To allow generation of bindings on functions, use [`ocaml_gen::func`](https://o1-labs.github.io/ocaml-gen/ocaml_gen/attr.func.html):

```rust
#[ocaml_gen::func]
#[ocaml::func] // if you use the crate ocaml-rs' macro, it must appear after
pub fn your_function(arg1: String) {
  //...
}
```

To allow generation of bindings on custom types, use [`ocaml_gen::CustomType`](https://o1-labs.github.io/ocaml-gen/ocaml_gen/derive.CustomType.html):

```rust
#[ocaml_gen::CustomType]
struct MyCustomType {
  // ...
}
```

## Organization

* [ocaml-gen](./ocaml-gen): the tool that allows us to generate the OCaml bindings from the Rust code.
* [ocaml-gen-derive](./ocaml-gen/derive): derive macros have to be a in separate crate, so they are here. This crate is re-exported by ocaml-gen so end users should not have to worry about it.
* [tests/](./tests/): contains some tests. If you are looking for examples on how to use ocaml-gen, you can check that folder.

## Additional resources

you can check the [recording](https://www.youtube.com/watch?v=LuXo2cNkgyA&feature=youtu.be) I made when I first introduced the tool internally.
