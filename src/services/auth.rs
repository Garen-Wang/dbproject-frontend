use crate::types::{
    auth::{LoginInfoWrapper, RegisterInfoWrapper, UserInfoWrapper, UserUpdateInfoWrapper},
    error::MyError,
};

use super::requests::{request_get, request_post, request_put};

pub async fn login(login_info: LoginInfoWrapper) -> Result<UserInfoWrapper, MyError> {
    request_post("/users/login".into(), login_info).await
}

pub async fn register(register_info: RegisterInfoWrapper) -> Result<UserInfoWrapper, MyError> {
    request_post("/users".into(), register_info).await
}

pub async fn get_user_info() -> Result<UserInfoWrapper, MyError> {
    request_get("/user".into()).await
}

pub async fn update_user_info(
    user_update_info: UserUpdateInfoWrapper,
) -> Result<UserInfoWrapper, MyError> {
    request_put("/user".into(), user_update_info).await
}
