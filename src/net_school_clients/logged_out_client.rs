use crate::{
    user_data::UserData,
    web_clients::{
        logged_out_web_client::LoggedOutWebClient, web_client_wrapper::WebClientWrapper,
    },
};

use super::inactive_client::InactiveClient;

/// A client that was logged in but is now logged out.
pub struct LoggedOutClient<Username, Password> {
    user_data: UserData<Username, Password>,
    web_client: WebClientWrapper<LoggedOutWebClient>,
}

impl<Username, Password> LoggedOutClient<Username, Password> {
    pub const fn new(
        user_data: UserData<Username, Password>,
        web_client: WebClientWrapper<LoggedOutWebClient>,
    ) -> Self {
        Self {
            user_data,
            web_client,
        }
    }
}

impl<Username, Password> From<LoggedOutClient<Username, Password>>
    for InactiveClient<Username, Password>
{
    fn from(logged_out_client: LoggedOutClient<Username, Password>) -> Self {
        Self::new(
            logged_out_client.user_data.into(),
            logged_out_client.web_client,
        )
    }
}
