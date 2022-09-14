pub enum SendingError<SenderSpecificError> {
    SenderSpecificError(SenderSpecificError),
    SendingError(reqwest::Error),
}

#[async_t::async_trait]
pub trait RequestSender {
    type Error;

    async fn send(
        &self,
        request: reqwest::Request,
    ) -> Result<reqwest::Response, SendingError<Self::Error>>;
}
