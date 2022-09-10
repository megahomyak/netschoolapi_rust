pub struct WebClientWithCookies(reqwest::Client);

impl WebClientWithCookies {
    pub fn new(client_builder: reqwest::ClientBuilder) -> Result<Self, reqwest::Error> {
        client_builder
            .cookie_store(true)
            .build()
            .map(Self)
    }

    pub const fn inner(&self) -> &reqwest::Client {
        &self.0
    }
}
