(lang dune 3.0)
(name ocamlgen-test)

(generate_opam_files true)

(source (github o1-labs/ocaml-gen))
(license Apache-2.0)
(authors "O(1) Labs, LLC <opensource@o1labs.org>")
(maintainers "O(1) Labs, LLC <opensource@o1labs.org>")
(bug_reports "https://github.com/o1-labs/ocaml-gen/issues")
(homepage "https://github.com/o1-labs/ocaml-gen/")

(package
 (name ocamlgen-test)
 (synopsis "Test library for ocaml-gen")
 (depends
  (dune (>= 3.0))
  (ocaml (and (>= 4.12) (< 5.0)))
  (conf-rust (>= 0.1))
 )
)
