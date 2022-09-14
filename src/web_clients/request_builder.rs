use serde::Serialize;

use super::request_sender::{RequestSender, SendingError};

pub struct RequestBuilder<'rb, RequestSender> {
    request_builder: reqwest::RequestBuilder,
    request_sender: &'rb RequestSender,
}

enum RequestError<WebClientError> {
    BuildingError(reqwest::Error),
    SenderSpecificError(WebClientError),
    SendingError(reqwest::Error),
}

impl<SenderSpecificError> From<SendingError<SenderSpecificError>>
    for RequestError<SenderSpecificError>
{
    fn from(error: SendingError<SenderSpecificError>) -> Self {
        match error {
            SendingError::SendingError(error) => Self::SendingError(error),
            SendingError::SenderSpecificError(error) => Self::SenderSpecificError(error),
        }
    }
}

impl<R: RequestSender> RequestBuilder<'_, R> {
    pub const fn new(request_builder: reqwest::RequestBuilder, request_sender: &R) -> Self {
        Self {
            request_builder,
            request_sender,
        }
    }

    pub async fn send(self) -> Result<reqwest::Response, RequestError<R::Error>> {
        match self.request_builder.build() {
            Ok(request) => self
                .request_sender
                .send(request)
                .await
                .map_err(|error| error.into()),
            Err(error) => Err(RequestError::BuildingError(error)),
        }
    }

    pub fn json<T: Serialize + ?Sized>(mut self, json: &T) -> Self {
        self.request_builder = self.request_builder.json(json);
        self
    }

    pub fn query<T: Serialize + ?Sized>(mut self, query: &T) -> Self {
        self.request_builder = self.request_builder.query(query);
        self
    }
}
