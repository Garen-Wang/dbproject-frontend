use reqwest::Method;
use serde::{Serialize, de::DeserializeOwned};

use crate::types::error::MyError;

use super::token::get_token;

async fn request<T, B>(method: Method, url: String, body: B) -> Result<T, MyError> 
where 
T: DeserializeOwned,
B: Serialize,
{
    let have_body = method == Method::POST || method == Method::PUT;
    let mut builder = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");
    if let Some(token) = get_token() {
        builder = builder.bearer_auth(token);
    }
    if have_body {
        builder = builder.json(&body);
    }

    let response = builder.send().await;
    if let Ok(response) = response {
        if response.status().is_success() {
            // status within 200-299
            let data = response.json::<T>().await;
            if let Ok(data) = data {
                log::debug!("get response");
                Ok(data)
            } else {
                Err(MyError::SerdeError)
            }
        } else {
            match response.status().as_u16() {
                401 => Err(MyError::Unauthorized),
                403 => Err(MyError::Forbidden),
                404 => Err(MyError::NotFound),
                500 => Err(MyError::InternalServerError),
                status => {
                    log::error!("unknown error status: {}", status);
                    Err(MyError::UnknownError)
                }
            }
        }
    } else {
        Err(MyError::ReqwestError)
    }
}

pub async fn request_get<T: DeserializeOwned>(url: String) -> Result<T, MyError> {
    request(Method::GET, url, ()).await
}

pub async fn request_post<T, B>(url: String, body: B) -> Result<T, MyError> 
where
T: DeserializeOwned,
B: Serialize,
{
    request(Method::POST, url, body).await
}

pub async fn request_put<T, B>(url: String, body: B) -> Result<T, MyError>
where
T: DeserializeOwned,
B: Serialize,
{
    request(Method::PUT, url, body).await
}

pub async fn request_delete<T: DeserializeOwned>(url: String) -> Result<T, MyError> {
    request(Method::DELETE, url, ()).await
}