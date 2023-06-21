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
pub fn fn_one_parameter(v1: Car) -> Car {
    v1
}

#[ocaml_gen::func]
#[ocaml::func]
pub fn fn_two_parameters(v1: Car, _v2: usize) -> Car {
    v1
}

#[ocaml_gen::func]
#[ocaml::func]
pub fn fn_three_parameters(v1: Car, _v2: usize, _v3: usize) -> Car {
    v1
}

#[ocaml_gen::func]
#[ocaml::func]
pub fn fn_four_parameters(v1: Car, _v2: usize, _v3: usize, _v4: usize) -> Car {
    v1
}

#[ocaml_gen::func]
#[ocaml::func]
pub fn fn_five_parameters(v1: Car, _v2: usize, _v3: usize, _v4: usize, _v5: usize) -> Car {
    v1
}

#[ocaml_gen::func]
#[ocaml::func]
pub fn fn_six_parameters(
    v1: Car,
    _v2: usize,
    _v3: usize,
    _v4: usize,
    _v5: usize,
    _v6: usize,
) -> Car {
    v1
}
// Test OCamlDesc is implemented for i32
#[ocaml_gen::func]
#[ocaml::func]
pub fn test_add_i32(s1: i32, s2: i32) -> i32 {
    s1 + s2
}

#[ocaml_gen::func]
#[ocaml::func]
pub fn test_add_usize(s1: usize, s2: usize) -> usize {
    s1 + s2
}

#[ocaml_gen::func]
#[ocaml::func]
pub fn test_bytes_get(s1: &[u8], i: usize) -> u8 {
    s1[i]
}

#[ocaml_gen::func]
#[ocaml::func]
pub fn test_get_ascii_code(ascii: u8) -> i32 {
    ascii as i32
}
