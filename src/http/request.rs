use super::method::METHOD;
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: METHOD,
}