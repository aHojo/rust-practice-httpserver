// Every file is treated as a module this is like mod server{}
use std::net::TcpListener;
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        println!("Listening on {}", self.addr);

        loop {
            listener.accept();
        }
    }
}