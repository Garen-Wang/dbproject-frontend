use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::{
    hooks::user_context::use_user_context, services::auth::register, types::auth::RegisterInfo,
};

#[function_component(Register)]
pub fn register() -> Html {
    let user_context = use_user_context();
    let register_info = use_state(RegisterInfo::default);
    let user_register = {
        let register_info = register_info.clone();
        let info = (*register_info).clone();
        use_async(async move { register(info).await })
    };

    use_effect_with_deps(
        move |user_register| {
            if let Some(data) = &user_register.data {
                user_context.login(data.clone());
            }
            if let Some(error) = &user_register.error {
                log::error!("{:?}", error);
            }
            || ()
        },
        user_register.clone(),
    );

    let oninput_username = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.username = input.value();
            register_info.set(info);
        })
    };
    let oninput_email = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.email = input.value();
            register_info.set(info);
        })
    };
    let oninput_password = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.password = input.value();
            register_info.set(info);
        })
    };
    let onsubmit = {
        let user_register = user_register.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            user_register.run();
        })
    };

    html! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center"> {"Register"} </h1>
                        <form {onsubmit}>
                        <fieldset>
                            <fieldset class="form-group">
                                <input type="text" class="form-control form-control-lg"
                                placeholder="username"
                                value={register_info.username.clone()} oninput={oninput_username}
                                 />
                            </fieldset>
                            <fieldset class="form-group">
                                <input type="email" class="form-control form-control-lg"
                                placeholder="email"
                                value={register_info.email.clone()} oninput={oninput_email}
                                 />
                            </fieldset>
                            <fieldset class="form-group">
                                <input type="password" class="form-control form-control-lg" 
                                placeholder="password"
                                value={register_info.password.clone()} oninput={oninput_password}
                                />
                            </fieldset>
                                <button type="submit" class="btn btn-lg btn-primary pull-xs-right"> {"Register"} </button>
                        </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
