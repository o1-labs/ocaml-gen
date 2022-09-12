use ocaml_gen::{decl_func, decl_module, decl_type, decl_type_alias, Env};
use ocamlgen_test_stubs::*;

use std::fmt::Write as _;

fn main() {
    let mut w = String::new();
    let env = &mut Env::new();

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

    // read bindings.ml content
    let bindings = include_str!("../bindings.ml");
    assert_eq!(bindings, w);
}
