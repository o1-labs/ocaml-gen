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

#[derive(ocaml::IntoValue, ocaml::FromValue, ocaml_gen::CustomType)]
pub struct Car {
    name: String,
    doors: usize,
}

#[ocaml_gen::func]
#[ocaml::func]
pub fn create_toyota() -> Car {
    Car {
        name: String::from("Toyota"),
        doors: 4,
    }
}

#[derive(ocaml::IntoValue, ocaml::FromValue, ocaml_gen::Struct)]
pub struct Package<T> {
    gift: T,
}

#[ocaml_gen::func]
#[ocaml::func]
pub fn pack_present() -> Package<String> {
    Package {
        gift: "hello".to_string(),
    }
}

#[ocaml_gen::func]
#[ocaml::func]
pub fn fn_with_more_than_5_parameters(
    v1: Car,
    _v2: usize,
    _v3: usize,
    _v4: usize,
    _v5: usize,
    _v6: usize,
) -> Car {
    v1
}
