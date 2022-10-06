use crate::app::App;

pub mod components;
pub mod types;
pub mod services;
pub mod routes;
pub mod hooks;

pub mod app;


fn main() {
    yew::start_app::<App>();
}