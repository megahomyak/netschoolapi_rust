use super::{
    logged_in_web_client::LoggedInWebClient, url_with_api_base_route::UrlWithApiBaseRoute,
    web_client_with_cookies::WebClientWithCookies, web_client_trait::WebClient,
};

pub struct LoggedOutWebClient {
    inner_client: WebClientWithCookies,
    base_url: UrlWithApiBaseRoute,
}

impl LoggedOutWebClient {
    pub const fn new(inner_client: WebClientWithCookies, base_url: UrlWithApiBaseRoute) -> Self {
        Self {
            inner_client,
            base_url,
        }
    }

    // Allowing `missing_const_for_fn` because the method also needs to invoke a destructor, but
    // clippy misses it.
    #[allow(clippy::missing_const_for_fn)]
    pub fn log_in(self, access_token: reqwest::header::HeaderValue) -> LoggedInWebClient {
        LoggedInWebClient::new(self.inner_client, self.base_url, access_token)
    }
}

impl WebClient for LoggedOutWebClient {
    fn add_headers(&self, request_builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        request_builder
    }

    fn base_url(&self) -> &UrlWithApiBaseRoute {
        &self.base_url
    }

    fn inner(&self) -> &WebClientWithCookies {
        &self.inner_client
    }
}
