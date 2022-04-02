mod app;
mod components;
mod error;
mod pages;
mod routes;
mod services;

use app::App;
use services::requests;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
