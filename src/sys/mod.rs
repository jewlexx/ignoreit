extern "C" {
    fn IsOnline() -> u8;
}

pub fn is_online() -> bool {
    unsafe { IsOnline() != 1 }
}
