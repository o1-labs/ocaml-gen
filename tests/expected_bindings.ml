type nonrec single_tuple = { inner: string } [@@boxed]
external new_t : unit -> single_tuple = "new"
external print_t : single_tuple -> unit = "print"

module Car = struct 
  type nonrec t
end


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

