use wasm_bindgen::prelude::*;

use yew;
use wasm_bindgen::JsValue;

mod utils;
// mod use app;
mod app;
use app::App;



// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<App>();
    Ok(())
}