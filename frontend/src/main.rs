mod app;
mod components;
mod error;
mod hooks;
mod pages;
mod routes;
mod services;
mod types;

use app::App;
use services::requests;
use wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
