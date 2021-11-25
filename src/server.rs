use std::io::Read;
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

            match listener.accept() {
                // because result is Result<(TcpStream, SocketAddr)> | _ is ignoring SocketAddr
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024]; // give 1024 zeroes.
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recieved a request: {}", String::from_utf8_lossy(&buffer))
                        },
                        Err(e) => println!("Failed to read from the socket: {}", e),
                    }


                },
                Err(e) => println!("Failed to establish a connection:  {}",  e)
            }

            // let res = listener.accept();
            //
            // if res.is_err() {
            //     continue;
            // }
            // let (stream, address) = res.unwrap();

        }
    }
}