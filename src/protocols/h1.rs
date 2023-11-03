

pub struct MessageHead<S> {
    pub version: http::Version,
    pub subject: S,
    pub headers: http::HeaderMap
}

pub type RequestHead = MessageHead<RequestLine>;

pub struct RequestLine {
    pub method: http::Method,
    pub uri: http::Uri
}

pub type ResponseHead = MessageHead<http::StatusCode>;


