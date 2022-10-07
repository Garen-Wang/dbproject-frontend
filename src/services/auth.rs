use crate::types::{
    auth::{LoginInfo, RegisterInfo, UserInfoWrapper},
    error::MyError,
};

use super::requests::{request_post, request_get};

pub async fn login(login_info: LoginInfo) -> Result<UserInfoWrapper, MyError> {
    request_post("/auth/login".into(), login_info).await
}

pub async fn register(register_info: RegisterInfo) -> Result<UserInfoWrapper, MyError> {
    request_post("/auth/register".into(), register_info).await
}

pub async fn get_user_info() -> Result<UserInfoWrapper, MyError> {
    request_get("/auth/info".into()).await
}

// pub async fn update_user_info(
//     user_update_info: UserUpdateInfoWrapper,
// ) -> Result<UserInfoWrapper, MyError> {
//     request_put("/user".into(), user_update_info).await
// }
