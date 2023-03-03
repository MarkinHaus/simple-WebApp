use std::ops::Deref;
use bounce::use_atom;

use gloo::console::log;
use serde::de::Unexpected::Option;
use serde_json::json;
use stylist::Style;
use yew::{
    html, Html,classes
};
use yew::prelude::*;
use yew_hooks::{use_local_storage, use_scrolling, use_timeout};
use yew_router::prelude::Link;
use crate::app::cloundm::{ResultCon, ServiceURL};
use crate::app::router::Route;
use crate::app::userdata::UserData;
use crate::util::text_input::TextInputField;

const STYLE: &str = include_str!("styles.css");


#[function_component(MainPage)]
pub fn main_app_vew() -> Html {

    let stylesheet = Style::new(STYLE).unwrap();

    let userdata = use_atom::<UserData>();

    let fist_time_scrolling = use_state(||false);
    let fade_aut = use_state(||"");
    let search = use_state(||String::from(""));

    let clone_userdata = userdata.clone();
    let storage = use_local_storage::<String>("token".to_string());
    let tok =  &*storage;
    if let Some(token) = tok {
        if !clone_userdata.auth {
            clone_userdata.set(UserData::init_withe_token(token.as_str()));
            let _timeout = {
            let state = fade_aut.clone();
            use_timeout(move || {
                state.set("fade-aut");
            }, 100);
                let state = fist_time_scrolling.clone();
            use_timeout(move || {
                state.set(true);
            }, 3000)
            };
        }
    }

    // if login

    let clone_search = search.clone();
    let search_changed = Callback::from(move |data: String| {
        let c_data = data.clone();
        clone_search.set(String::from(c_data))
    });


    let clone_userdata = userdata.clone();
    let clone_storage = storage.clone();
    let logaut = Callback::from(move |_| {
        clone_userdata.set(UserData::default());
        clone_storage.delete();
    });

    let clone_fts = fist_time_scrolling.clone();
    let clone_alim = fade_aut.clone();
    let clone_userdata = userdata.clone();
    let scip_alim = Callback::from(move |_| {
        let clone_alim = clone_alim.clone();
        let clone_fts = clone_fts.clone();
        if clone_userdata.auth {
            clone_alim.set("fade-aut-speed");
            clone_fts.set(true);
        }
    });


    let clone_userdata = userdata.clone();
    let fts = *fist_time_scrolling.deref();
    let class = *fade_aut.deref();

    html! {
        <div class={stylesheet} id="lines">
        if clone_userdata.auth {
            <>
                <h3> { ""} <button class="Logaut" onclick={logaut}>{"Logaut"}</button> {""} </h3>
            </>
        }

        <div class="main-page" style={"padding-top: 25vh;"}>

        if !fts {
                <div class={classes!("wrapper", class)}>
                    <div onclick={scip_alim}>
                    <p>{"Welcome to"} </p>
                    <div class="words">
                        <span>{""}</span>
                        <span>{"Simple"}</span>
                        <span>{"Simples"}</span>
                        <span>{"Eenvoudig"}</span>
                        <span>{"Vienkāršs"}</span>
                    </div>
                    </div>
                </div>
        }
        if clone_userdata.auth {
            if fts{
                <div class="Apps">
                    <div class="search-box">
                <TextInputField
            class=""
            id="username-create_acc_mhs"
            name="search"
            type_="text"
            value=""
            placeholder="Search for Apps"//anything u like"
            label="We ar Searching for :"
            set_value={search_changed}
                open={true}
            />
                    </div>
                    <div class="app-box">
<Link<Route> to={Route::QuickNote}>{"QuickNote"}</Link<Route>><br/>
<Link<Route> to={Route::DayTree}>{"DayTree"}</Link<Route>>
                    </div>
                </div>
            }else{
                <footer class="info">
                    <h3> { "Redy to Continue."} </h3>
                </footer>
            }
        }else{
            <>
                <footer>
                    <h3> { "Pleas "} <Link<Route> to={Route::CloudMLogin}>{"LogIn "}</Link<Route>> {"to Continue."} </h3>
                    <h3> { "Or "} <Link<Route> to={Route::CloudMCreate}>{"Register"}</Link<Route>> {""} </h3>
                </footer>
            </>
        }
        </div>
        </div>
    }
}
