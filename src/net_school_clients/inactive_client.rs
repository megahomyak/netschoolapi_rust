use std::{borrow::Borrow, collections::HashMap};

use concat_strs::concat_strs;
use encoding::Encoding;
use num::BigInt;

use crate::{
    auth_data::AuthData,
    user_data::UserData,
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
    /// School with the specified name was not found.
    SchoolNotFound,
}

pub enum SchoolsGettingError {
    RequestError(reqwest::Error),
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

// Resolves a bug in clippy that lets it invoke `::use_self` on a macro-generated code.
#[allow(clippy::use_self)]
mod schemas {
    use num::BigInt;
    use serde::{Deserialize, Serialize};

    /// Used to distinguish between the Gosuslugi log-on and username-password log-on.
    #[repr(u8)]
    #[derive(Serialize)]
    pub enum LoginType {
        /// With username and password.
        Regular = 1,
    }

    #[repr(u8)]
    #[derive(Serialize, Deserialize)]
    pub enum EducationalInstitutionType {
        PreSchool = 1,
        School = 2,
        /// An institution used simultaneously with any other insitution. Examples: sports school, arts
        /// school. This does not have to be a distinct institution though, so the institutions with
        /// the `Additional` type are highly likely to be present in the `School` or `PreSchool` lists
        /// (sometimes even in both of them).
        Additional = 3,
    }

    impl Default for EducationalInstitutionType {
        fn default() -> Self {
            Self::School
        }
    }

    #[derive(Deserialize, Serialize)]
    pub struct PreAuthData {
        /// I have no idea what this is.
        pub lt: String,
        /// I have no idea what this is.
        pub ver: String,
        /// A salt that should be used when hashing a password.
        pub salt: String,
    }

    #[derive(Deserialize, Serialize)]
    pub struct SchoolInfo {
        #[serde(rename(serialize = "cid", deserialize = "countryId"))]
        pub country_id: BigInt,
        #[serde(rename(serialize = "sid", deserialize = "stateId"))]
        pub state_id: BigInt,
        #[serde(rename(serialize = "pid", deserialize = "municipalityDistrictId"))]
        pub pid: BigInt,
        #[serde(rename(serialize = "cn", deserialize = "cityId"))]
        pub city_id: BigInt,
        #[serde(rename(serialize = "sft"), default)]
        pub educational_institution_type: EducationalInstitutionType,
        #[serde(rename(serialize = "scid", deserialize = "id"))]
        pub id: BigInt,
        #[serde(skip_serializing)]
        pub name: String,
    }

    #[derive(Serialize)]
    pub struct AuthRequest<'auth_request, Username> {
        #[serde(rename(serialize = "loginType"))]
        pub login_type: LoginType,
        #[serde(rename(serialize = "un"))]
        pub username: Username,
        /// The md5_hex(salt + md5_hex(windows_cp1251(password))) encoded password.
        #[serde(rename(serialize = "pw2"))]
        pub full_encoded_password: &'auth_request str,
        /// Same as `full_encoded_password`, but trimmed (from the end) to the length of the original
        /// password.
        #[serde(rename(serialize = "pw"))]
        pub trimmed_encoded_password: &'auth_request str,
        #[serde(flatten)]
        pub pre_auth_data: PreAuthData,
        #[serde(flatten)]
        pub school_info: &'auth_request SchoolInfo,
    }

    #[derive(Deserialize)]
    pub struct AuthResponse {
        #[serde(rename(deserialize = "at"))]
        pub auth_token: String,
    }

    #[derive(Deserialize)]
    pub struct Student {
        #[serde(rename(serialize = "studentId"))]
        pub student_id: BigInt,
    }

    #[derive(Deserialize)]
    pub struct DiaryInfo {
        pub students: Vec<Student>,
        #[serde(rename(serialize = "currentStudentId"))]
        pub current_student_id: BigInt,
    }

    #[derive(Deserialize)]
    pub struct CurrentYear {
        pub id: BigInt,
    }

    #[derive(Deserialize)]
    pub struct Assignment {
        pub id: BigInt,
        pub name: String,
    }
}

pub use schemas::SchoolInfo;
#[allow(clippy::wildcard_imports)]
use schemas::*;

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
        macro_rules! to_json {
            ($expr:expr) => {
                match $expr {
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
            return Err((AuthError::RequestError(error), self));
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
                        Err(_raw_password) =>
                            return Err((AuthError::InvalidCharactersInPassword, self)),
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
                        return Err((AuthError::InvalidJsonInServerResponse, self))
                    }
                },
                Err(error) => {
                    if let Some(status) = error.status() {
                        if status == reqwest::StatusCode::CONFLICT {
                            return Err((AuthError::InvalidCredentials(error), self));
                        }
                    }
                    return Err((AuthError::RequestError(error), self));
                }
            };
            let access_token = match auth_response.auth_token.try_into() {
                Ok(access_token) => access_token,
                Err(_conversion_error) => {
                    return Err((AuthError::InvalidJsonInServerResponse, self))
                }
            };
            WebClientWrapper::new(self.web_client.into_inner().log_in(access_token))
        };

        macro_rules! error {
            ($error:expr) => {{
                self.web_client =
                    WebClientWrapper::new(logged_in_web_client.into_inner().log_out());
                return Err(($error, self));
            }};
        }

        macro_rules! to_json {
            ($expr:expr) => {
                match $expr {
                    Ok(resp) => match resp.json().await {
                        Ok(json) => json,
                        Err(_parsing_error) => error!(AuthError::InvalidJsonInServerResponse),
                    },
                    Err(error) => error!(AuthError::RequestError(error)),
                }
            };
        }

        let student = {
            let mut diary_info: DiaryInfo = match logged_in_web_client
                .get("student/diary/init")
                .unwrap()
                .send()
                .await
            {
                Ok(resp) => match resp.json().await {
                    Ok(json) => json,
                    Err(_parsing_error) => error!(AuthError::InvalidJsonInServerResponse),
                },
                Err(error) => error!(AuthError::RequestError(error)),
            };
            let student_index = match diary_info.current_student_id.try_into() {
                Ok(index) => index,
                Err(_conversion_error) => error!(AuthError::InvalidJsonInServerResponse),
            };
            if student_index >= diary_info.students.len() {
                error!(AuthError::InvalidJsonInServerResponse);
            }
            diary_info.students.swap_remove(student_index)
        };

        let current_year: CurrentYear = to_json!(
            logged_in_web_client
                .get("years/current")
                .unwrap()
                .send()
                .await
        );

        let assignment_types: HashMap<BigInt, String> = {
            let assignments: Vec<Assignment> = to_json!(
                logged_in_web_client
                    .get("grade/assignment/types")
                    .unwrap()
                    .send()
                    .await
            );
            assignments
                .into_iter()
                .map(|assignment| (assignment.id, assignment.name))
                .collect()
        };

        Ok(LoggedInClient::new(
            UserData::new(
                student.student_id,
                current_year.id,
                assignment_types,
                self.auth_data,
            ),
            logged_in_web_client,
        ))
    }
}
