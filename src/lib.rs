use app::App;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

pub mod components;
pub mod types;
pub mod services;

pub mod app;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    Ok(())
}