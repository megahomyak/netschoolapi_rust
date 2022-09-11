use std::borrow::Borrow;

use concat_strs::concat_strs;
use encoding::Encoding;

use crate::{
    auth_data::AuthData,
    schemas::{AuthRequest, AuthResponse, LoginType, PreAuthData, SchoolInfo},
    user_data::{self, UserData},
    web_clients::{
        logged_out_web_client::LoggedOutWebClient, web_client_wrapper::WebClientWrapper,
    },
};

use super::logged_in_client::LoggedInClient;

/// A client that was never logged in.
pub struct InactiveClient<Username, Password> {
    auth_data: AuthData<Username, Password>,
    web_client: WebClientWrapper<LoggedOutWebClient>,
}

pub enum AuthError {
    RequestError(reqwest::Error),
    /// It's either the response is not JSON at all, or it does not match the expected structure.
    InvalidJsonInServerResponse,
    InvalidCharactersInPassword,
    InvalidCredentials(reqwest::Error),
}

impl From<user_data::MakingError> for AuthError {
    fn from(error: user_data::MakingError) -> Self {
        use crate::user_data::MakingError;

        match error {
            MakingError::RequestError(error) => Self::RequestError(error),
            MakingError::InvalidJsonInServerResponse => Self::InvalidJsonInServerResponse,
        }
    }
}

pub enum SchoolsGettingError {
    RequestError(reqwest::Error),
    /// It's either the response is not JSON at all, or it does not match the expected structure.
    InvalidJsonInServerResponse,
}

pub enum SchoolByNameGettingError {
    SchoolsGettingError(SchoolsGettingError),
    /// School with the specified name was not found.
    SchoolNotFound,
}

impl From<SchoolsGettingError> for SchoolByNameGettingError {
    fn from(error: SchoolsGettingError) -> Self {
        Self::SchoolsGettingError(error)
    }
}

impl<Username, Password> InactiveClient<Username, Password> {
    pub const fn new(
        auth_data: AuthData<Username, Password>,
        web_client: WebClientWrapper<LoggedOutWebClient>,
    ) -> Self {
        Self {
            auth_data,
            web_client,
        }
    }

    pub fn set_auth_data(&mut self, auth_data: AuthData<Username, Password>) {
        self.auth_data = auth_data;
    }
}

impl<Username: Borrow<str> + Send + Sync, Password: Borrow<str> + Send + Sync>
    InactiveClient<Username, Password>
{
    pub async fn schools(&self) -> Result<Vec<SchoolInfo>, SchoolsGettingError> {
        let schools: Vec<SchoolInfo> = match self
            .web_client
            .get("addresses/schools")
            .unwrap()
            .send()
            .await
        {
            Ok(resp) => match resp.json().await {
                Ok(schools) => schools,
                Err(_error) => return Err(SchoolsGettingError::InvalidJsonInServerResponse),
            },
            Err(error) => return Err(SchoolsGettingError::RequestError(error)),
        };
        Ok(schools)
    }

    pub async fn school_by_name(
        &self,
        school_name: impl Borrow<str> + Send,
    ) -> Result<SchoolInfo, SchoolByNameGettingError> {
        #[allow(clippy::never_loop)]
        let school_info = 'school: loop {
            for school in self.schools().await? {
                if school.name == school_name.borrow() {
                    break 'school school;
                }
            }
            return Err(SchoolByNameGettingError::SchoolNotFound);
        };
        Ok(school_info)
    }

    #[allow(clippy::too_many_lines)]
    pub async fn log_in(mut self) -> Result<LoggedInClient<Username, Password>, (AuthError, Self)> {
        macro_rules! error {
            ($error:expr) => {
                return Err(($error, self))
            };
        }

        macro_rules! to_json {
            ($response:expr) => {
                match $response {
                    Ok(resp) => match resp.json().await {
                        Ok(json) => json,
                        Err(_parsing_error) => {
                            return Err((AuthError::InvalidJsonInServerResponse, self))
                        }
                    },
                    Err(error) => return Err((AuthError::RequestError(error), self)),
                }
            };
        }

        if let Err(error) = self.web_client.get("logindata").unwrap().send().await {
            error!(AuthError::RequestError(error));
        }; // Gathering the necessary cookies
        let pre_auth_data: PreAuthData =
            to_json!(self.web_client.post("auth/getdata").unwrap().send().await);

        let encoded_password_with_salt = {
            let encoded_password = format!(
                "{:x}",
                md5::compute(
                    match encoding::all::WINDOWS_1251
                        .encode(&pre_auth_data.salt, encoding::EncoderTrap::Strict)
                    {
                        Ok(encoded_password) => encoded_password,
                        Err(_raw_password) => error!(AuthError::InvalidCharactersInPassword),
                    }
                )
            );

            format!(
                "{:x}",
                md5::compute(concat_strs!(&pre_auth_data.salt, &encoded_password))
            )
        };

        let logged_in_web_client = {
            let auth_response: AuthResponse = match self
                .web_client
                .post("login")
                .unwrap()
                .json({
                    &AuthRequest {
                        login_type: LoginType::Regular,
                        school_info: &self.auth_data.school_info,
                        username: self.auth_data.username.borrow(),
                        full_encoded_password: &encoded_password_with_salt
                            [..self.auth_data.password.borrow().len()],
                        trimmed_encoded_password: &encoded_password_with_salt,
                        pre_auth_data,
                    }
                })
                .send()
                .await
                .and_then(reqwest::Response::error_for_status)
            {
                Ok(resp) => match resp.json().await {
                    Ok(auth_response) => auth_response,
                    Err(_parsing_error) => {
                        error!(AuthError::InvalidJsonInServerResponse)
                    }
                },
                Err(error) => {
                    if let Some(status) = error.status() {
                        if status == reqwest::StatusCode::CONFLICT {
                            error!(AuthError::InvalidCredentials(error));
                        }
                    }
                    error!(AuthError::RequestError(error));
                }
            };
            let access_token = match auth_response.auth_token.try_into() {
                Ok(access_token) => access_token,
                Err(_conversion_error) => {
                    error!(AuthError::InvalidJsonInServerResponse)
                }
            };
            WebClientWrapper::new(self.web_client.into_inner().log_in(access_token))
        };

        let user_data = match UserData::make(&logged_in_web_client, self.auth_data).await {
            Ok(user_data) => user_data,
            Err((error, auth_data)) => {
                self.auth_data = auth_data;
                self.web_client =
                    WebClientWrapper::new(logged_in_web_client.into_inner().log_out());
                error!(error.into())
            }
        };

        Ok(LoggedInClient::new(user_data, logged_in_web_client))
    }
}
