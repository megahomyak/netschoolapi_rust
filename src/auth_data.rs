use crate::net_school_clients::inactive_client::SchoolInfo;

pub struct AuthData<Username, Password> {
    pub username: Username,
    pub password: Password,
    pub school_info: SchoolInfo,
}
