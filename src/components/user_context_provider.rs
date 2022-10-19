use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::{types::{auth::UserInfo, error::MyError}, services::{auth::get_user_info, token::{get_token, set_token}}};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user_context = use_state(UserInfo::default);
    let user_info = use_async(async move {get_user_info().await});
    {
        let user_info = user_info.clone();
        use_mount(move || {
            if get_token().is_some() {
                user_info.run();
            }
        });
    }
    {
        let user_context = user_context.clone();
        use_effect_with_deps(move |user_info| {
            if let Some(user_info) = &user_info.data {
                user_context.set(user_info.clone());
            }
            if let Some(error) = &user_info.error {
                match error {
                    MyError::Unauthorized | MyError::Forbidden => set_token(None),
                    _ => (),
                }
            }
            || ()
        }, user_info);
    }

    html! {
        <ContextProvider<UseStateHandle<UserInfo>> context={user_context}>
            {props.children.clone()}
        </ContextProvider<UseStateHandle<UserInfo>>>
    }
}