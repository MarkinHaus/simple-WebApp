#[allow(unused_imports)]
mod app;
mod util;

use crate::app::MainApp;

fn main() {
    yew::start_app::<MainApp>();
}
