use serde::{Serialize, Deserialize};


#[derive(Serialize)]
pub struct LoginInfo {
    email: String,
    password: String,
}
#[derive(Serialize)]
pub struct LoginInfoWrapper {
    inner: LoginInfo,
}

#[derive(Serialize)]
pub struct RegisterInfo {
    username: String,
    email: String,
    password: String,
}
#[derive(Serialize)]
pub struct RegisterInfoWrapper {
    inner: RegisterInfo,
}

#[derive(Clone, PartialEq, Deserialize, Default)]
pub struct UserInfo {
    username: String,
    email: String,
    token: String,
    bio: Option<String>,
    img: Option<String>,
}
#[derive(Clone, PartialEq, Deserialize)]
pub struct UserInfoWrapper {
    pub inner: UserInfo,
}
impl UserInfo {
    pub fn is_authenticated(&self) -> bool {
        !self.token.is_empty()
    }
}

#[derive(Serialize)]
pub struct UserUpdateInfo {
    username: String,
    email: String,
    password: String,
    bio: Option<String>,
    img: Option<String>,
}
#[derive(Serialize)]
pub struct UserUpdateInfoWrapper {
    inner: UserUpdateInfo,
}