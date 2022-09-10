use super::{
    logged_out_web_client::LoggedOutWebClient, url_with_api_base_route::UrlWithApiBaseRoute,
    web_client_with_cookies::WebClientWithCookies, web_client_trait::WebClient,
};

pub struct LoggedInWebClient {
    inner_client: WebClientWithCookies,
    base_url: UrlWithApiBaseRoute,
    access_token: reqwest::header::HeaderValue,
}

impl LoggedInWebClient {
    pub const fn new(
        inner_client: WebClientWithCookies,
        base_url: UrlWithApiBaseRoute,
        access_token: reqwest::header::HeaderValue,
    ) -> Self {
        Self {
            inner_client,
            base_url,
            access_token,
        }
    }

    // Allowing `missing_const_for_fn` because the method also needs to invoke a destructor, but
    // clippy misses it.
    #[allow(clippy::missing_const_for_fn)]
    pub fn log_out(self) -> LoggedOutWebClient {
        LoggedOutWebClient::new(self.inner_client, self.base_url)
    }
}

impl WebClient for LoggedInWebClient {
    fn add_headers(&self, request_builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        request_builder.header("at", &self.access_token)
    }

    fn inner(&self) -> &WebClientWithCookies {
        &self.inner_client
    }

    fn base_url(&self) -> &UrlWithApiBaseRoute {
        &self.base_url
    }
}
