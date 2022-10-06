use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::{login::Login, register::Register, home::Home};

pub mod home;
pub mod login;
pub mod register;

#[derive(Clone, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &AppRoute) -> Html {
    match route {
        AppRoute::Login => html! {<Login />},
        AppRoute::Register => html! {<Register />},
        AppRoute::Home => html! {<Home />},
        AppRoute::NotFound => html! {"Page Not Found"},
    }
}