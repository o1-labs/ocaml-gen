use ocaml_gen::prelude::*;
use ocamlgen_test_stubs::*;

use std::fmt::Write as _;

fn main() {
    let mut w = String::new();
    let env = &mut Env::new();

    decl_fake_generic!(T1, 0);

    decl_type!(w, env, SingleTuple => "single_tuple");
    decl_func!(w, env, new => "new_t");
    decl_func!(w, env, print => "print_t");

    decl_module!(w, env, "Car", {
        decl_type!(w, env, Car => "t");
    });

    decl_module!(w, env, "Toyota", {
        decl_type_alias!(w, env, "t" => Car);
        decl_func!(w, env, create_toyota => "create_toyota");
    });

    decl_module!(w, env, "Packages", {
        decl_type!(w, env, Package<T1> => "t");
    });

    decl_module!(w, env, "Gifts", {
        decl_type_alias!(w, env, "t" => Package<String>);
        decl_func!(w, env, pack_present => "pack_present");
    });

    // read bindings.ml content
    let bindings = include_str!("../bindings.ml");
    if bindings != w {
        eprintln!("the tests compiled as a different bindings file.");
        eprintln!("what was expected:");
        eprintln!("---");
        eprintln!("{bindings}");
        eprintln!("---");
        eprintln!("what was generated:");
        eprintln!("---");
        eprintln!("{w}");
        eprintln!("---");
        panic!("see error above");
    };
}
