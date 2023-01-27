use lazy_static::lazy_static;
use reqwest::{header::HeaderMap, Client, Error, Method, Response, Url};

///
///
/// # Examples
///
/// ```
/// use shift_core_request::request;
/// use reqwest::{Method, Url, header::HeaderMap};
///
/// let result = request(
///     Method::GET,
///     Url::parse("https://example.com").unwrap(),
///     HeaderMap::new()
/// );
/// ```
pub async fn request(method: Method, url: Url, headers: HeaderMap) -> Result<Response, Error> {
    lazy_static! {
        static ref CLIENT: Client = Client::new();
    }

    let request = CLIENT.request(method, url).headers(headers);
    request.send().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        request(
            Method::GET,
            Url::parse("https://example.com").unwrap(),
            HeaderMap::new(),
        )
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    }
}
