use crate::{
    user_data::UserData,
    web_clients::{
        logged_out_web_client::LoggedOutWebClient, web_client_wrapper::WebClientWrapper,
    },
};

use super::inactive_client::InactiveClient;

/// A client that was logged in but is now logged out.
pub struct LoggedOutClient<Username, Password, RequestSender> {
    user_data: UserData<Username, Password>,
    web_client: WebClientWrapper<LoggedOutWebClient, RequestSender>,
}

impl<Username, Password, RequestSender> LoggedOutClient<Username, Password, RequestSender> {
    pub const fn new(
        user_data: UserData<Username, Password>,
        web_client: WebClientWrapper<LoggedOutWebClient, RequestSender>,
    ) -> Self {
        Self {
            user_data,
            web_client,
        }
    }
}

impl<Username, Password, RequestSender> From<LoggedOutClient<Username, Password, RequestSender>>
    for InactiveClient<Username, Password, RequestSender>
{
    fn from(logged_out_client: LoggedOutClient<Username, Password, RequestSender>) -> Self {
        Self::new(
            logged_out_client.user_data.into(),
            logged_out_client.web_client,
        )
    }
}
