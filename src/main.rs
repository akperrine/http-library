use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener},
    thread::spawn,
};

use server::Server;

mod server;
fn main() {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8000);

    let server = Server::bind(&socket);

    for stream in server.incoming.listener.incoming() {
        match stream {
            Ok(_) => {
                println!("listening");
            }
            Err(e) => {
                println!("There was an error {}", e)
            }
        }
    }
}
