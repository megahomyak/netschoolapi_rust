use crate::{
    user_data::UserData,
    web_clients::{logged_in_web_client::LoggedInWebClient, web_client_wrapper::WebClientWrapper},
};

use super::logged_out_client::LoggedOutClient;

/// A client that is logged on.
pub struct LoggedInClient<Username, Password> {
    user_data: UserData<Username, Password>,
    web_client: WebClientWrapper<LoggedInWebClient>,
}

impl<Username, Password> LoggedInClient<Username, Password> {
    pub const fn new(
        user_data: UserData<Username, Password>,
        web_client: WebClientWrapper<LoggedInWebClient>,
    ) -> Self {
        Self {
            user_data,
            web_client,
        }
    }
}

impl<Username: Send, Password: Send> LoggedInClient<Username, Password> {
    pub async fn log_out_anyway(
        self,
    ) -> (
        LoggedOutClient<Username, Password>,
        Result<reqwest::Response, reqwest::Error>,
    ) {
        let logging_out_result = self.web_client.post("auth/logout").unwrap().send().await;
        (
            LoggedOutClient::new(
                self.user_data,
                WebClientWrapper::new(self.web_client.into_inner().log_out()),
            ),
            logging_out_result,
        )
    }

    pub async fn log_out(
        self,
    ) -> Result<LoggedOutClient<Username, Password>, (reqwest::Error, Self)> {
        match self.web_client.post("auth/logout").unwrap().send().await {
            Ok(_resp) => Ok(LoggedOutClient::new(
                self.user_data,
                WebClientWrapper::new(self.web_client.into_inner().log_out()),
            )),
            Err(error) => {
                if let Some(status) = error.status() {
                    if status == reqwest::StatusCode::UNAUTHORIZED {
                        return Ok(LoggedOutClient::new(
                            self.user_data,
                            WebClientWrapper::new(self.web_client.into_inner().log_out()),
                        ));
                    }
                }
                Err((error, self))
            }
        }
    }
}
