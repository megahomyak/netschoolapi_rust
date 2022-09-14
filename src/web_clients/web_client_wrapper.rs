use super::{
    request_builder::RequestBuilder, request_sender::RequestSender, web_client_trait::WebClient,
};

#[derive(Debug)]
pub enum RequestError {
    IllFormedUrl,
}

/// Wraps the web client to provide the necessary methods.
/// Not made as a default trait method so it cannot be overridden.
pub struct WebClientWrapper<Inner, RequestSender> {
    pub inner: Inner,
    pub request_sender: RequestSender,
}

impl<Inner: WebClient, R: RequestSender> WebClientWrapper<Inner, R> {
    pub const fn new(inner: Inner, request_sender: R) -> Self {
        Self {
            inner,
            request_sender,
        }
    }

    pub fn request(
        &self,
        url: &str,
        method: reqwest::Method,
    ) -> Result<RequestBuilder<R>, RequestError> {
        self.inner
            .base_url()
            .inner()
            .join(url)
            .map(|url| {
                RequestBuilder::new(
                    self.inner
                        .add_headers(self.inner.inner().inner().request(method, url))
                        .header("user-agent", "NetSchoolAPI/5.0.3")
                        .header("referer", self.inner.base_url().inner().as_str()),
                    &self.request_sender,
                )
            })
            .or(Err(RequestError::IllFormedUrl))
    }

    pub fn get(&self, url: &str) -> Result<RequestBuilder<R>, RequestError> {
        self.request(url, reqwest::Method::GET)
    }

    pub fn post(&self, url: &str) -> Result<RequestBuilder<R>, RequestError> {
        self.request(url, reqwest::Method::POST)
    }
}
