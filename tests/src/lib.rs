#[derive(ocaml::IntoValue, ocaml::FromValue, ocaml_gen::Struct)]
pub struct SingleTuple(String);

#[ocaml_gen::func]
#[ocaml::func]
pub fn new() -> SingleTuple {
    SingleTuple(String::from("Hello"))
}

#[ocaml_gen::func]
#[ocaml::func]
pub fn print(s: SingleTuple) {
    println!("{}", s.0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ocaml_gen::{decl_func, decl_type, Env};
    use std::fmt::Write;

    #[test]
    fn test_bindings() {
        let mut w = String::new();
        let env = &mut Env::default();

        decl_type!(w, env, SingleTuple => "single_tuple");
        decl_func!(w, env, new => "new_t");
        decl_func!(w, env, print => "print_t");

        assert_eq!(
            w,
            r#"type nonrec single_tuple = { inner: string } [@@boxed]
external new_t : unit -> single_tuple = "new"
external print_t : single_tuple -> unit = "print"
"#,
        );
    }
}
