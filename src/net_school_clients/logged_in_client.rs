use std::borrow::Borrow;

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

impl<Username: Borrow<str>, Password: Borrow<str>> LoggedInClient<Username, Password> {
    pub const fn new(
        user_data: UserData<Username, Password>,
        web_client: WebClientWrapper<LoggedInWebClient>,
    ) -> Self {
        Self {
            user_data,
            web_client,
        }
    }

    pub fn log_out(self) -> LoggedOutClient<Username, Password> {
        LoggedOutClient::new(
            self.user_data,
            WebClientWrapper::new(self.web_client.into_inner().log_out()),
        )
    }
}
