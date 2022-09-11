use crate::schemas::SchoolInfo;

pub struct AuthData<Username, Password> {
    pub username: Username,
    pub password: Password,
    pub school_info: SchoolInfo,
}
