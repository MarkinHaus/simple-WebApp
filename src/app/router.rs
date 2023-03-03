use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::main_page::MainPage;
use crate::app::quick_note::QuickNoteApp;
use crate::app::daytree::DayTreeApp;
use crate::app::cloundm::{Login, CreatAccMhs};


#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/app/Login/")]
    CloudMLogin,
    #[at("/app/Create-Account/")]
    CloudMCreate,
    //#[at("/app/Settings/")]
    ////CloudMLoginSettings,
    #[at("/app/QuickNote")]
    QuickNote,
    //#[at("/app/QuickNote/*path")]
    //QuickNoteSub {path: String},
    #[at("/app/DayTree")]
    DayTree,
    //#[at("/app/DayTree/*path")]
    ////DayTreeSub {path: String},
    #[not_found]
    #[at("/app/404")]
    NotFound,
}


pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <MainPage/> },
        Route::CloudMLogin => html! { <Login/> },
        Route::CloudMCreate => html! { <CreatAccMhs/> },
        Route::QuickNote => html! { <QuickNoteApp/> },
        Route::DayTree => html! { <DayTreeApp/> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
