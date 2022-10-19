use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::{hooks::user_context::use_user_context, types::auth::UpdateInfo, services::auth::{update_user_info, get_user_info}};

#[function_component(Settings)]
pub fn settings() -> Html {
    let user_context = use_user_context();
    let update_info = use_state(UpdateInfo::default);

    let user_info = use_async_with_options(
        async move { get_user_info().await }, UseAsyncOptions::enable_auto(),
    );

    let user_update = {
        let update_info = update_info.clone();
        let info = (*update_info).clone();
        use_async(async move { update_user_info(info).await })
    };

    // {
    //     let user_info = user_info.clone();
    //     let update_info = update_info.clone();
    //     use_effect_with_deps(move |user_info| {
    //         if let Some(data) = &user_info.data {
    //             update_info.set(UpdateInfo {
    //                 username: None,
    //                 password: None,
    //             });
    //         }
    //         if let Some(error) = &user_info.error {
    //             log::error!("{:?}", error);
    //         }
    //         || ()
    //     }, user_info);
    // }

    {
        let user_context = user_context.clone();
        let user_update = user_update.clone();
        use_effect_with_deps(move |user_update| {
            if let Some(data) = &user_update.data {
                user_context.login(data.clone());
            }
            if let Some(error) = &user_update.error {
                log::error!("{:?}", error);
            }
            || ()
        }, user_update);
    }

    let oninput_username = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            let val = input.value();
            info.username = if val.is_empty() { None } else { Some(val) };
            update_info.set(info);
        })
    };
    let oninput_password = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            let val = input.value();
            info.password = if val.is_empty() { None } else { Some(val) };
            update_info.set(info);
        })
    };
    let onsubmit = {
        let user_update = user_update.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            user_update.run();
        })
    };
    let onclick = Callback::from(move |_| { user_context.logout(); });
    html! {
        <div class="settings-page">
        <div class="container page">
        <div class="row">
        <div class="col-md-6 offset-md-3 col-xs-12">
            <h1 class="text-xs-center">{"Settings"}</h1>
            <form {onsubmit}>
            <fieldset>
                <fieldset class="form-group">
                    <input type="text" class="form-control form-control-lg" 
                    placeholder="new username"
                    value={update_info.username.clone()}
                    oninput={oninput_username}
                    />
                </fieldset>
                <fieldset class="form-group">
                    <input type="password" class="form-control form-control-lg" 
                    placeholder="new password"
                    value={update_info.password.clone()}
                    oninput={oninput_password}
                    />
                </fieldset>
                // <fieldset class="form-group">
                //     <input type="password" class="form-control form-control-lg" 
                //     placeholder="confirm new password"
                //     />
                // </fieldset>
                <button class="btn btn-lg btn-primary pull-xs-right" type="submit"> {"Update settings"} </button>
            </fieldset>
            </form>
            <hr />
            <button class="btn btn-outline-danger" {onclick}
            disabled={user_info.loading || user_update.loading} >
            {"logout"}
            </button>
        </div>
        </div>
        </div>
        </div>
    }
}