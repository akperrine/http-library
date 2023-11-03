use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use server::Server;

mod server;
fn main() {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    let server = Server::bind(&socket);
}
