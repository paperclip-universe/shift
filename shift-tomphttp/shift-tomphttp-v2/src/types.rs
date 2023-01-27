use reqwest::{header::HeaderMap, Url};

pub struct BareHeaderData {
    remote: Url,
    send_headers: HeaderMap,
    pass_headers: Vec<String>,
    pass_status: Vec<i32>,
    forward_headers: Vec<String>,
}
