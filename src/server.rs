use std::io::Read;
use std::convert::TryFrom; // we need to pull in this trait to use try_from on Request.
use std::convert::TryInto; // get this for free because of TryFrom
use crate::http::Request; // Crate means go to the root of the crate.


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
                    let mut buffer = [0; 1024]; // give 1024 zeroes and is an Array.
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recieved a request: {}", String::from_utf8_lossy(&buffer));
                            // Request::try_from(&buffer as &[u8]); // convert to byte slice
                            // this also converts to a slice.
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }
                            // let res: &Result<Request, _> = &buffer[..].try_into();

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