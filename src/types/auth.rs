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
    pub id: String,
    pub username: String,
    pub email: String,
}

#[derive(Clone, Serialize, Default)]
pub struct UpdateInfo {
    pub username: Option<String>,
    pub password: Option<String>,
}
