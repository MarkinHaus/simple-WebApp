mod router;
mod main_page;
mod userdata;
mod cloundm;
mod quick_note;
mod daytree;
mod dt_classes;
mod dt_now;

use bounce::*;

use yew_router::prelude::*;
use yew::prelude::*;
use gloo::console::log;
use stylist::Style;

use crate::app::router::Route;
use crate::app::router::switch;

const STYLE: &str = include_str!("root.css");


#[function_component(MainApp)]
pub fn main_app() -> Html {
    log!("Starting");
    let stylesheet = Style::new(STYLE).unwrap();
    html! {
        <>
        <div class={stylesheet} id ={"hnjkm"}>
        <div class={"root"}>
            <BounceRoot>
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
            </BounceRoot>
        </div>
        </div>
        </>
    }
}


#[function_component(Hader)]
pub fn hader() -> Html {


    html! {

        <div class={"Nav"}>
            <h3><Link<Route> to={Route::CloudMLogin}>{"Logaut"}</Link<Route>></h3>
            <h3><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></h3>
            <h3><Link<Route> to={Route::Home}>{"Mods"}</Link<Route>></h3>
        </div>
    }
}
