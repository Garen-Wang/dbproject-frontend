use wee_alloc::WeeAlloc;
use wasm_bindgen::prelude::*;

use crate::app::App;

pub mod components;
pub mod types;
pub mod services;
pub mod routes;
pub mod hooks;

pub mod app;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    Ok(())
}
