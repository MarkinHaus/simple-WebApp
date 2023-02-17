use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::cloudm_manager::SettingsRouteCloudM;
use crate::app::cloudm_manager::switch_cloudm;
use crate::app::day_tree_manager::SettingsRouteDayTree;
use crate::app::day_tree_manager::switch_day_tree;
use crate::app::main_page::MainPage;
use crate::quick_note::QuickNotesVewPot;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/cloudm/:s")]
    CloudM,
    #[at("/dtt/:s")]
    DayTree,
    #[at("/QuickNote")]
    QuickNote,
    #[not_found]
    #[at("/404")]
    NotFound,
}


pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <MainPage/> },
        Route::CloudM => html! {
            <Switch<SettingsRouteCloudM> render={Switch::render(switch_cloudm)} />
        },
        Route::DayTree => html! {
            <>
            <Link<SettingsRouteDayTree> to={SettingsRouteDayTree::Now}>{ "Now" }</Link<SettingsRouteDayTree>>
            <Link<SettingsRouteDayTree> to={SettingsRouteDayTree::Designer}>{ "Designer" }</Link<SettingsRouteDayTree>>
            <Link<SettingsRouteDayTree> to={SettingsRouteDayTree::Calender}>{ "Calender" }</Link<SettingsRouteDayTree>>
            <Switch<SettingsRouteDayTree> render={Switch::render(switch_day_tree)} />
            </>
        },
        Route::QuickNote => html! {
            <QuickNotesVewPot />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}


pub struct NavItems;

impl Component for NavItems {
    type Message = Route;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let history = ctx.link().history().unwrap();

        let go_home_button = {
            let history = history.clone();
            let onclick = Callback::from(move |_| history.push(Route::Home));
            html! {
                <button {onclick}>{"home"}</button>
            }
        };

        let go_login_button = {
            let history = history.clone();
            let onclick = Callback::from(move |_| history.push(SettingsRouteCloudM::Login));
            html! {
                <button {onclick}>{"CloudM Login"}</button>
            }
        };

        let go_create_acc_button = {
            let history = history.clone();
            let onclick = Callback::from(move |_| history.push(Route::QuickNote));
            html! {
                <button {onclick}>{"QuickNote"}</button>
            }
        };

        html! {
            <>
                {go_home_button}
                {go_login_button}
                {go_create_acc_button}
            </>
        }
    }
}