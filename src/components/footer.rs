use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
        <div class="container">
        <Link<AppRoute> to={AppRoute::Home} classes="logo-font">{"dbproject"}</Link<AppRoute>>
        <span class="attribution"> 
        { "© 2022. dbproject" }
        </span>
        </div>
        </footer>
    }
}