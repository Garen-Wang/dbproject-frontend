use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Default, Clone)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}
#[derive(Serialize)]
pub struct LoginInfoWrapper {
    inner: LoginInfo,
}
impl From<UseStateHandle<LoginInfo>> for LoginInfoWrapper {
    fn from(info: UseStateHandle<LoginInfo>) -> Self {
        let inner = &*info;
        Self {
            inner: inner.clone(),
        }
    }
}

#[derive(Serialize, Default, Clone)]
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub password: String,
}
#[derive(Serialize)]
pub struct RegisterInfoWrapper {
    inner: RegisterInfo,
}
impl From<UseStateHandle<RegisterInfo>> for RegisterInfoWrapper {
    fn from(info: UseStateHandle<RegisterInfo>) -> Self {
        let inner = &*info;
        Self {
            inner: inner.clone(),
        }
    }
}

#[derive(Clone, PartialEq, Deserialize, Default)]
pub struct UserInfo {
    username: String,
    email: String,
    pub token: String,
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
