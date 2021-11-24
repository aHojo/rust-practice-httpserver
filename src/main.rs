use server::Server;
// use http::request::Request;
// use http::method::METHOD;

// These work because of the use statements in http/mod.rs
use http::Request;
use http::METHOD;

mod server;
mod http;
fn main() {
    // let string = String::from("127.0.0.1:8080"); // goes on the heap
    // let string_slice = &string[10..]; // get a range from a String
    // let string_literal = "1234";
    //
    // // dbg!(string); // doesn't work because it's borrowed
    // dbg!(&string); // doesn't work because it's borrowed
    // dbg!(string_slice);
    // dbg!(string_literal);

    // let get = METHOD::GET;
    // let delete = METHOD::DELETE;
    // let post = METHOD::POST;
    // let put = METHOD::PUT;

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

// mod server {
//     pub struct Server {
//         addr: String,
//     }
//
//     impl Server {
//         pub fn new(addr: String) -> Self {
//             Self {
//                 addr
//             }
//         }
//         // fn new(addr: String) -> Server {
//         //     Server {
//         //         addr
//         //     }
//         // }
//
//         pub fn run(self) {
//             println!("Listening on {}", self.addr);
//         }
//     }
// }

// mod http {
//     pub mod method {
//         pub enum METHOD {
//             GET,
//             DELETE,
//             POST,
//             PUT,
//             HEAD,
//             CONNECT,
//             OPTIONS,
//             TRACE,
//             PATCH,
//         }
//     }
//
//     // sub modules are private by default.
//     pub mod request {
//         use super::method::METHOD;
//         pub struct Request {
//             path: String,
//             query_string: Option<String>,
//             method: METHOD,
//         }
//     }
// }

/*
GET /usr?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/