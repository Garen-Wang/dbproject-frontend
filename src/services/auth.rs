use crate::types::{
    auth::{LoginInfo, RegisterInfo, UserInfo, UpdateInfo},
    error::MyError,
};

use super::requests::{request_post, request_get, request_put};

pub async fn login(login_info: LoginInfo) -> Result<UserInfo, MyError> {
    request_post("/auth/login".into(), login_info).await
}

pub async fn register(register_info: RegisterInfo) -> Result<UserInfo, MyError> {
    request_post("/auth/register".into(), register_info).await
}

pub async fn get_user_info() -> Result<UserInfo, MyError> {
    request_get("/auth/info".into()).await
}

pub async fn update_user_info( update_info: UpdateInfo) -> Result<UserInfo, MyError> {
    request_put("/auth/update".into(), update_info).await
}
