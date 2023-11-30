use super::method::Method;

/* Request would look like this:

GET /api?param=value HTTP/1.1\r\n
HEADERS \r\n
BODY

*/

pub struct Request {
    path: String,

    // if our request do not have a query string,
    // we will store Option:None
    query_string: Option<String>,
    method: Method,
}
