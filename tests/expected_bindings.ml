type nonrec single_tuple = { inner: string } [@@boxed]
external new_t : unit -> single_tuple = "new"
external print_t : single_tuple -> unit = "print"

module Car = struct 
  type nonrec t
end

external fn_one_parameter : Car.t -> Car.t = "fn_one_parameter"
external fn_two_parameters : Car.t -> int -> Car.t = "fn_two_parameters"
external fn_three_parameters : Car.t -> int -> int -> Car.t = "fn_three_parameters"
external fn_four_parameters : Car.t -> int -> int -> int -> Car.t = "fn_four_parameters"
external fn_five_parameters : Car.t -> int -> int -> int -> int -> Car.t = "fn_five_parameters"
external fn_six_parameters : Car.t -> int -> int -> int -> int -> int -> Car.t = "fn_six_parameters_bytecode" "fn_six_parameters"

module Toyota = struct 
  type nonrec t = Car.t
  external create_toyota : unit -> t = "create_toyota"
end


module Packages = struct 
  type nonrec ('t) t = { gift: 't } [@@boxed]
end


module Gifts = struct 
  type nonrec t = (string) Packages.t
  external pack_present : unit -> t = "pack_present"
end

