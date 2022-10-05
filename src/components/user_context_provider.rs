use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::{types::{auth::UserInfo, error::MyError}, services::{auth::get_user_info, token::{get_token, set_token}}};

#[derive(PartialEq, Properties)]
struct Props {
    children: Children,
}

#[function_component(UserContextProvider)]
fn user_context_provider(props: &Props) -> Html {
    let user_ctx = use_state(UserInfo::default);
    let current_user_info = use_async(async move {get_user_info().await});
    {
        let current_user_info = current_user_info.clone();
        use_mount(move || {
            if get_token().is_some() {
                current_user_info.run();
            }
        });
    }
    {
        let user_ctx = user_ctx.clone();
        use_effect_with_deps(move |current_user_info| {
            if let Some(user_info) = &current_user_info.data {
                user_ctx.set(user_info.inner.clone());
            }
            if let Some(error) = &current_user_info.error {
                match error {
                    MyError::Unauthorized | MyError::Forbidden => set_token(None),
                    _ => (),
                }
            }
            || ()
        }, current_user_info);
    }

    html! {
        <ContextProvider<UseStateHandle<UserInfo>> context={user_ctx}>
            {props.children.clone()}
        </ContextProvider<UseStateHandle<UserInfo>>>
    }
}