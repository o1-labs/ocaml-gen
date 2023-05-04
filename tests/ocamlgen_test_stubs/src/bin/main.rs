use ocaml_gen::prelude::*;
use ocamlgen_test_stubs::*;

use std::fmt::Write as _;
use std::fs;

fn main() -> std::io::Result<()> {
    let mut w = String::new();
    let env = &mut Env::new();

    ocaml_gen::decl_fake_generic!(T1, 0);

    ocaml_gen::decl_type!(w, env, SingleTuple => "single_tuple");
    ocaml_gen::decl_func!(w, env, new => "new_t");
    ocaml_gen::decl_func!(w, env, print => "print_t");

    ocaml_gen::decl_module!(w, env, "Car", {
        ocaml_gen::decl_type!(w, env, Car => "t");
    });

    ocaml_gen::decl_module!(w, env, "Toyota", {
        ocaml_gen::decl_type_alias!(w, env, "t" => Car);
        ocaml_gen::decl_func!(w, env, create_toyota => "create_toyota");
    });

    ocaml_gen::decl_module!(w, env, "Packages", {
        ocaml_gen::decl_type!(w, env, Package<T1> => "t");
    });

    ocaml_gen::decl_module!(w, env, "Gifts", {
        ocaml_gen::decl_type_alias!(w, env, "t" => Package<String>);
        ocaml_gen::decl_func!(w, env, pack_present => "pack_present");
    });

    fs::write("bindings.ml", w)?;
    Ok(())
}
