use super::{
    request_builder::RequestBuilder,
    web_client_trait::{RequestError, WebClient},
};

/// Wraps the web client to provide the necessary methods.
/// Not made as a default trait method so it cannot be overridden.
pub struct WebClientWrapper<Inner>(Inner);

impl<Inner: WebClient> WebClientWrapper<Inner> {
    pub const fn new(inner: Inner) -> Self {
        Self(inner)
    }

    pub fn request(
        &self,
        url: &str,
        method: reqwest::Method,
    ) -> Result<RequestBuilder, RequestError> {
        self.0
            .base_url()
            .inner()
            .join(url)
            .map(|url| {
                RequestBuilder::new(
                    self.0
                        .add_headers(self.0.inner().inner().request(method, url))
                        .header("user-agent", "NetSchoolAPI/5.0.3")
                        .header("referer", self.0.base_url().inner().as_str()),
                )
            })
            .or(Err(RequestError::IllFormedUrl))
    }

    pub fn get(&self, url: &str) -> Result<RequestBuilder, RequestError> {
        self.request(url, reqwest::Method::GET)
    }

    pub fn post(&self, url: &str) -> Result<RequestBuilder, RequestError> {
        self.request(url, reqwest::Method::POST)
    }

    // Allowing `missing_const_for_fn` because the method also needs to invoke a destructor, but
    // clippy misses it.
    #[allow(clippy::missing_const_for_fn)]
    pub fn into_inner(self) -> Inner {
        self.0
    }
}
