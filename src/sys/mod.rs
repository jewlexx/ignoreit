extern "C" {
    fn IsOnline() -> cty::c_int;
}

pub fn is_online() -> bool {
    unsafe { IsOnline() != 1 }
}
