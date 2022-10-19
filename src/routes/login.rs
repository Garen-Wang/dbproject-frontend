use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::{hooks::user_context::use_user_context, services::auth::login, types::auth::LoginInfo};

#[function_component(Login)]
pub fn login() -> Html {
    let user_context = use_user_context();
    let login_info = use_state(LoginInfo::default);

    let user_login = {
        let login_info = login_info.clone();
        let info = (*login_info).clone();
        use_async(async move { login(info).await })
    };
    use_effect_with_deps(
        move |user_login| {
            if let Some(data) = &user_login.data {
                user_context.login(data.clone());
            }
            if let Some(error) = &user_login.error {
                log::error!("{:?}", error);
            }
            || ()
        },
        user_login.clone(),
    );

    let oninput_email = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.email = input.value();
            login_info.set(info);
        })
    };
    let oninput_password = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.password = input.value();
            login_info.set(info);
        })
    };

    let onsubmit = {
        let user_login = user_login.clone();
        Callback::from(move |e: FocusEvent| {
            log::debug!("login onsubmit callback");
            e.prevent_default();
            user_login.run();
        })
    };

    html! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center"> {"Login"} </h1>
                        <form {onsubmit}>
                        <fieldset>
                            <fieldset class="form-group">
                                <input type="email" name="email" id="email" class="form-control form-control-lg" 
                                placeholder="email"
                                value={login_info.email.clone()} oninput={oninput_email} />
                            </fieldset>
                            <fieldset class="form-group">
                                <input type="password" name="password" id="password" class="form-control form-control-lg"
                                placeholder="password"
                                value={login_info.password.clone()} oninput={oninput_password} />
                            </fieldset>
                            <button type="submit" class="btn btn-lg btn-primary pull-xs-right"> {"Login"} </button>
                        </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
