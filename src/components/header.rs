use yew::prelude::*;
use yew_router::prelude::*;

use crate::{routes::AppRoute, hooks::user_context::use_user_context};

#[function_component(Header)]
pub fn header() -> Html {
    let user_context = use_user_context();
    html! {
        <nav class="navbar navbar-light">
        <div class="container">
        <Link<AppRoute> to={AppRoute::Home} classes="navbar-brand">
        {"dbproject"}
        </Link<AppRoute>>
        {
            if user_context.is_authenticated() {
                login_view()
            } else {
                logout_view()
            }
        }
        </div>
        </nav>
    }
}

#[allow(dead_code)]
fn login_view() -> Html {
    html! {
        <ui class="nav navbar-nav pull-xs-right">
        <li class="nav-item"><Link<AppRoute> to={AppRoute::Home}>{"home"}</Link<AppRoute>></li>
        </ui>
    }
}

#[allow(dead_code)]
fn logout_view() -> Html {
    html! {
        <ui class="nav navbar-nav pull-xs-right">
        <li class="nav-item"><Link<AppRoute> to={AppRoute::Home}>{"home"}</Link<AppRoute>></li>
        <li class="nav-item"><Link<AppRoute> to={AppRoute::Login}>{"login"}</Link<AppRoute>></li>
        <li class="nav-item"><Link<AppRoute> to={AppRoute::Register}>{"register"}</Link<AppRoute>></li>
        </ui>
    }
}