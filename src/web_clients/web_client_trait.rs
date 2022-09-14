use super::{
    url_with_api_base_route::UrlWithApiBaseRoute, web_client_with_cookies::WebClientWithCookies,
};

pub trait WebClient {
    fn add_headers(&self, request_builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder;
    fn inner(&self) -> &WebClientWithCookies;
    fn base_url(&self) -> &UrlWithApiBaseRoute;
}
