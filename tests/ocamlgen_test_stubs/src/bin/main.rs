use ocaml_gen::prelude::*;
use ocamlgen_test_stubs::*;

use std::fmt::Write as _;
use std::io;
use std::io::Write;

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

    ocaml_gen::decl_func!(w, env, fn_one_parameter => "fn_one_parameter");
    ocaml_gen::decl_func!(w, env, fn_two_parameters => "fn_two_parameters");
    ocaml_gen::decl_func!(w, env, fn_three_parameters => "fn_three_parameters");
    ocaml_gen::decl_func!(w, env, fn_four_parameters => "fn_four_parameters");
    ocaml_gen::decl_func!(w, env, fn_five_parameters => "fn_five_parameters");
    ocaml_gen::decl_func!(w, env, fn_six_parameters => "fn_six_parameters");
    ocaml_gen::decl_func!(w, env, test_add_i32 => "test_add_i32");
    ocaml_gen::decl_func!(w, env, test_add_usize => "test_add_usize");

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

    io::stdout().write_all(w.as_bytes())?;
    Ok(())
}
