let test_single_tuple_access_field () =
  let b = Bindings.new_t () in
  assert (b.inner = "Hello")

let test_built_in_type_int32 () =
  let s1 = Int32.of_int 3 in
  let s2 = Int32.of_int 6 in
  assert (Int32.(add s1 s2) = Bindings.test_add_i32 s1 s2)

let () =
  let open Alcotest in
  run "Test binding generations"
    [ ( "single_tuple"
      , [ test_case "Access field and check value" `Quick
            test_single_tuple_access_field
        ] )
    ; ("builtin types", [ test_case "add i32" `Quick test_built_in_type_int32 ])
    ]
