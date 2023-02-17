use std::ops::Deref;

use gloo::console::log;
use stylist::Style;
use stylist::yew::styled_component;
use yew::{
    Callback, html, Html,
};
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::app::cloudm_manager::SettingsRouteCloudM;
use crate::app::router::Route;
use crate::util::text_input::TextInputField;

const STYLE: &str = include_str!("styles.css");

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub service_url: Route,
    pub service_name: String,
}

impl Module {
    pub fn new(service_url: Route, service_name: String) -> Self {
        Self {
            service_url,
            service_name,
        }
    }
}

pub struct MainAppVewState {
    pub modules: Vec<Module>,
}

#[derive(Properties, PartialEq)]
pub struct ModuleVewProps {
    pub set_modules: Callback<Vec<Module>>,
    pub set_auth: Callback<bool>,
}

#[derive(Properties, PartialEq)]
pub struct ModuleVew {
    pub module: Module,
    pub index: usize,
}


fn rap_module_to_card(notes: Box<Vec<Module>>) -> Vec<Html> {
    let mut i = 0usize;
    notes.iter().map(|module| {
        i += 1;
        html! {
        <ModuleCard
            module={module.clone()}
            index={i}
        />
    }
    }).collect()
}

#[styled_component(ModuleCard)]
pub fn module_card(props: &ModuleVew) -> Html {
    html! {
        <div class={"moduleCarte"}>
              <h1>{&props.module.service_name}</h1>
              <Link<Route> to={props.module.service_url.clone()}>{"Open"}</Link<Route>>
        </div>
    }
}

#[styled_component(MainPage)]
pub fn main_app_vew() -> Html {
    let mut modules: Vec<Module>;

    modules = vec![
        Module { service_url: Route::CloudM, service_name: "CloudM".to_string() },
        Module { service_url: Route::QuickNote, service_name: "QuickNote".to_string() },
        Module { service_url: Route::DayTree, service_name: "DayTree".to_string() }];


    let state = use_state(|| MainAppVewState { modules: modules.clone() });

    let clone_state = state.clone();
    let modules = rap_module_to_card(Box::new(clone_state.modules.clone()));

    let set_value = Callback::from(|val: String| {
        log!(val)
    });

    let app_toggle = use_state(|| false);

    let clone_app_toggle = app_toggle.clone();
    let onclick = Callback::from(move |_| clone_app_toggle.set(!clone_app_toggle.deref().clone()));

    let stylesheet = Style::new(STYLE).unwrap();

    html! {
        <div class={stylesheet}>
            <div class="main-page" style={"padding-top: 25vh;"}>
                <h1> {"Welcome to Simpel"} </h1>

        if app_toggle.deref().clone() {
            <div class="app-page" onclick={onclick.clone()}>
            <TextInputField
                    class="search-input"
                    id=""
                    name="name"
                    type_="text"
                    value=""
                    placeholder="Searcher"
                    open={true}
                    label=""
                    set_value={set_value}
                />
            {modules.clone()}
            </div>
        }else {
            <div class="app-page-close" onclick={onclick.clone()}>
                <h2>{"App"}</h2>
            </div>
        }
                <h3> { "Pleas "} <Link<SettingsRouteCloudM> to={SettingsRouteCloudM::Login}>{"LogIn "}</Link<SettingsRouteCloudM>> {"to Continue"} </h3>
            </div>
        </div>
    }
}