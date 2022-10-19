use yew::prelude::*;
use yew_router::prelude::*;

use std::ops::Deref;

use crate::{routes::AppRoute, services::token::{set_token, get_token}, types::auth::UserInfo};

#[derive(Clone)]
pub struct UserContextHandle {
    inner: UseStateHandle<UserInfo>,
    history: AnyHistory,
}

impl Deref for UserContextHandle {
    type Target = UserInfo;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl UserContextHandle {
    pub fn is_authenticated(&self) -> bool {
        get_token().is_some()
    }

    pub fn login(&self, user_info: UserInfo) {
        log::debug!("user context login");
        set_token(Some(user_info.email.clone()));
        self.inner.set(user_info);
        self.history.push(AppRoute::Home);
    }

    pub fn update(&self, user_info: UserInfo) {
        log::debug!("user context update");
        self.inner.set(user_info);
        self.history.push(AppRoute::Home);
    }

    pub fn logout(&self) {
        log::debug!("user context logout");
        set_token(None);
        self.inner.set(UserInfo::default());
        self.history.push(AppRoute::Home);
    }
}

pub fn use_user_context() -> UserContextHandle {
    UserContextHandle {
        inner: use_context().unwrap(),
        history: use_history().unwrap(),
    }
}
