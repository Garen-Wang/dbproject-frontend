use yew::prelude::*;
use yew_router::prelude::*;

use std::ops::Deref;

use crate::{routes::AppRoute, services::token::set_token, types::auth::UserInfo};

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
    pub fn login(&self, user_info: UserInfo) {
        set_token(Some(user_info.token.clone()));
        self.inner.set(user_info);
        self.history.push(AppRoute::Home);
    }

    pub fn logout(&self) {
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
