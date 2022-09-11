// Resolves a bug in clippy that lets it invoke `::use_self` on a macro-generated code.
#![allow(clippy::use_self)]

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
