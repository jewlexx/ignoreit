use std::net::ToSocketAddrs;

pub fn is_online() -> bool {
    let address = "https://github.com".to_socket_addrs();

    if let Ok(mut addr) = address {
        return addr.next().is_some();
    }

    false
}
