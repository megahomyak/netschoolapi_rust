use std::collections::HashMap;

use num::BigInt;

use crate::{
    auth_data::AuthData,
    schemas::{Assignment, CurrentYear, DiaryInfo},
    web_clients::{logged_in_web_client::LoggedInWebClient, web_client_wrapper::WebClientWrapper, request_sender::RequestSender},
};

pub struct UserData<Username, Password> {
    student_id: BigInt,
    year_id: BigInt,
    assignment_types: HashMap<BigInt, String>,
    auth_data: AuthData<Username, Password>,
}

pub enum MakingError {
    RequestError(reqwest::Error),
    /// It's either the response is not JSON at all, or it does not match the expected structure.
    InvalidJsonInServerResponse,
}

impl<Username: Send, Password: Send> UserData<Username, Password> {
    pub async fn make<R: RequestSender>(
        web_client: &WebClientWrapper<LoggedInWebClient, R>,
        auth_data: AuthData<Username, Password>,
    ) -> Result<Self, (MakingError, AuthData<Username, Password>)> {
        macro_rules! error {
            ($error:expr) => {
                return Err(($error, auth_data))
            };
        }

        macro_rules! to_json {
            ($response:expr) => {
                match $response {
                    Ok(resp) => match resp.json().await {
                        Ok(json) => json,
                        Err(_parsing_error) => error!(MakingError::InvalidJsonInServerResponse),
                    },
                    Err(error) => error!(MakingError::RequestError(error)),
                }
            };
        }

        let student = {
            let mut diary_info: DiaryInfo =
                match web_client.get("student/diary/init").unwrap().send().await {
                    Ok(resp) => match resp.json().await {
                        Ok(json) => json,
                        Err(_parsing_error) => error!(MakingError::InvalidJsonInServerResponse),
                    },
                    Err(error) => error!(MakingError::RequestError(error)),
                };
            let student_index = match diary_info.current_student_id.try_into() {
                Ok(index) => index,
                Err(_conversion_error) => error!(MakingError::InvalidJsonInServerResponse),
            };
            if student_index >= diary_info.students.len() {
                error!(MakingError::InvalidJsonInServerResponse);
            }
            diary_info.students.swap_remove(student_index)
        };

        let current_year: CurrentYear =
            to_json!(web_client.get("years/current").unwrap().send().await);

        let assignment_types: HashMap<BigInt, String> = {
            let assignments: Vec<Assignment> = to_json!(
                web_client
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

        Ok(Self {
            auth_data,
            student_id: student.student_id,
            assignment_types,
            year_id: current_year.id,
        })
    }
}

impl<Username, Password> UserData<Username, Password> {
    pub const fn student_id(&self) -> &BigInt {
        &self.student_id
    }

    pub const fn year_id(&self) -> &BigInt {
        &self.year_id
    }

    pub const fn assignment_types(&self) -> &HashMap<BigInt, String> {
        &self.assignment_types
    }

    pub const fn auth_data(&self) -> &AuthData<Username, Password> {
        &self.auth_data
    }
}

impl<Username, Password> From<UserData<Username, Password>>
    for AuthData<Username, Password>
{
    fn from(user_data: UserData<Username, Password>) -> Self {
        user_data.auth_data
    }
}
