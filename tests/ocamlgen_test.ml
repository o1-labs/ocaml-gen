let test_single_tuple_access_field () =
  let b = Bindings.new_t () in
  assert (b.inner = "Hello")

let () =
  let open Alcotest in
  run "Test binding generations"
    [ ( "single_tuple"
      , [ test_case "Access field and check value" `Quick
            test_single_tuple_access_field
        ] )
    ]
