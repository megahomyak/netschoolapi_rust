use std::collections::HashMap;

use num::BigInt;

use crate::auth_data::AuthData;

pub struct UserData<Username, Password> {
    student_id: BigInt,
    year_id: BigInt,
    assignment_types: HashMap<BigInt, String>,
    auth_data: AuthData<Username, Password>,
}

impl<Username, Password> UserData<Username, Password> {
    pub const fn new(
        student_id: BigInt,
        year_id: BigInt,
        assignment_types: HashMap<BigInt, String>,
        auth_data: AuthData<Username, Password>,
    ) -> Self {
        Self {
            student_id,
            year_id,
            assignment_types,
            auth_data,
        }
    }

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
