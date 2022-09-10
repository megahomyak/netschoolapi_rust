use serde::Serialize;

pub struct RequestBuilder(reqwest::RequestBuilder);

impl RequestBuilder {
    pub async fn send(self) -> Result<reqwest::Response, reqwest::Error> {
        self.0
            .send()
            .await
            .and_then(reqwest::Response::error_for_status)
    }

    pub fn json<T: Serialize + ?Sized>(mut self, json: &T) -> Self {
        self.0 = self.0.json(json);
        self
    }

    pub const fn new(inner: reqwest::RequestBuilder) -> Self {
        Self(inner)
    }

    pub fn query<T: Serialize + ?Sized>(mut self, query: &T) -> Self {
        self.0 = self.0.query(query);
        self
    }
}
