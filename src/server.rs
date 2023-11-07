use std::{
    error::Error,
    net::{SocketAddr, TcpListener},
};

pub struct Server<I, S> {
    pub incoming: I,
    pub make_service: S,
}

pub struct Builder<I> {
    pub incoming: I,
}

impl<I> Server<I, ()> {
    pub fn builder(incoming: I) -> Builder<I> {
        println!("Building Server with");
        Builder { incoming }
    }
}

impl Server<AddrIncoming, ()> {
    pub fn bind(addr: &SocketAddr) -> Builder<AddrIncoming> {
        let addr_incoming = AddrIncoming::new(addr).unwrap();
        println!("Server binding at address: {}", addr);
        Builder {
            incoming: addr_incoming,
        }
    }
}

pub struct AddrIncoming {
    pub addr: SocketAddr,
    pub listener: TcpListener,
}

impl AddrIncoming {
    pub fn new(addr: &SocketAddr) -> Result<Self, Box<dyn Error>> {
        let listener = TcpListener::bind(addr)?;

        Ok(AddrIncoming {
            addr: *addr,
            listener,
        })
    }
}
