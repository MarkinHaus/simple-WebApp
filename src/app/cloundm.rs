use std::ops::Deref;
use bounce::use_atom;
use gloo::console::log;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use stylist::Style;
use yew::{Callback, FocusEvent, function_component, Html, html, use_state};
use yew_hooks::{use_async, use_effect_once, use_local_storage, use_session_storage, UseAsyncHandle};
use yew_router::history::History;
use crate::app::userdata::UserData;
use crate::util::text_input::TextInputField;
use yew_router::prelude::Link;
use crate::app::router::Route;
use yew_router::hooks::use_history;

const STYLE: &str = include_str!("cloundm.css");
// api calls

#[derive(Clone, PartialEq, Deserialize, Serialize, Default, Debug)]
pub struct ResultCon<T> {
    pub res: T,
}

#[derive(Serialize, Deserialize)]
pub struct RequestSender {
    pub token: Option<String>,
    pub data: Option<String>,
}

pub struct ServiceURL {
    pub url: String,
    pub error: Option<String>,
}

const URL: &str = include_str!("url");

impl ServiceURL {
    pub fn default() -> Self {
        Self {
            url: String::from(URL),
            error: None,
        }
    }

    pub async fn post_request(&mut self, data: Value, token: String, mod_name: String, function: String, command: String) -> String {
        let client = reqwest::Client::new();
        let res = client.post(format!("{}/post/{}/run/{}?command={}", ServiceURL::default().url, mod_name, function, command))
            .header("content-type", "application/json")
            .json(&json!({
                    "data": data,
                    "token": token
                }))
            .send()
            .await.unwrap();
        let body: String = String::from(res.text().await.unwrap());
        log!(body.clone(), "BODY");
        if body != "".to_string() {
            // let res: ResultCon<T> = serde_json::fr(&body.as_str()).unwrap();
            // print_type_of(&res.res);
            return body.to_string();
        }
        return "[]".to_string();
    }
}

pub enum _CreateAccError {
    CanNotAccessToken,
    UserNameAlreadyExists,
    UserEmailAlreadyExists,

}

pub enum _LoginAccError {
    InvalidPassword,
    InvalidUsername,
    CanNotAccessTokenServer,
    CanNotAccessTokenUser,
}

pub enum _VerificationError {
    TokenExpired,
    InvalidSignature,
    CanNotAccessTokenServer,
}

// Log in
#[derive(Clone, Debug, Default)]
pub struct CreateAccMHsData {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[function_component(CreatAccMhs)]
pub fn create_acc_mhs() -> Html {
    let userdata = use_atom::<UserData>();
    let acc_data = use_state(CreateAccMHsData::default);
    let storage = use_local_storage::<String>("token".to_string());

    let clone_data = acc_data.clone();
    let username_changed = Callback::from(move |username: String| {
        clone_data.set(
            CreateAccMHsData {
                username,
                ..clone_data.deref().clone()
            });
    });

    let clone_data = acc_data.clone();
    let password_changed = Callback::from(move |password: String| {
        clone_data.set(
            CreateAccMHsData {
                password,
                ..clone_data.deref().clone()
            });
    });

    let clone_data = acc_data.clone();
    let email_changed = Callback::from(move |email: String| {
        clone_data.set(
            CreateAccMHsData {
                email,
                ..clone_data.deref().clone()
            });
    });

    let clone_data = acc_data.clone();
    let clone_userdata = userdata.clone();
    let clone_storage = storage.clone();
    let state: UseAsyncHandle<String, _> = use_async(async move {
            let mut service = ServiceURL::default();
            let token = service.post_request(json!({
                    "username" : clone_data.username.clone(),
                    "email" : clone_data.email.clone(),
                    "password" : clone_data.password.clone(),
                }), "".to_string(), "cloudM".to_string(), "create_user".to_string(), "".to_string()).await;

            let res: ResultCon<String> = serde_json::from_str(&token.as_str()).unwrap();

            if res.res != "".to_string() {
                if res.res.len()> 200 {
                    let ud = UserData::init_withe_token(res.res.as_str());
                    clone_storage.set(ud.token.to_string());
                    clone_userdata.set(ud);
                }
                return Ok(res.res);
            }
        return Err(res.res)

    });
    let clone_state = state.clone();

    let on_sub = Callback::from(move |event: FocusEvent| {
        event.prevent_default();
        clone_state.run();
    });

    let clone_userdata = userdata.clone();
    let stylesheet = Style::new(STYLE).unwrap();
    html! {
        <div class={stylesheet}>
        <div class="login">
            <form onsubmit={on_sub}>
                <div class="login-bg">
                <h1>{"New Account "}</h1>
                <h3>
                if let Some(error) = &state.error {
                    {error}
                }
                </h3>
            </div>
            if clone_userdata.auth.clone() {
            <div class="login">
                <h1>{"Logged in"}</h1>
                <h2>{"Username : ... "}</h2>
                <Link<Route> to={Route::Home}>{"Go Home"}</Link<Route>>
            </div>
             }else {
            <div class="content">
                <TextInputField
                    class=""
                    id="email-create_acc_mhs"
                    name="email"
                    type_="email"
                    value=""
                    placeholder="email"
                    label="email:"
                    set_value={email_changed}
                        open={true}
                />
                <TextInputField
                    class=""
                    id="text-create_acc_mhs"
                    name="text"
                    type_="text"
                    value=""
                    placeholder="Username"
                    label="Username:"
                    set_value={username_changed}
                        open={true}
                />
                <TextInputField
                    class=""
                    id="password-create_acc_mhs"
                    name="password"
                    type_="password"
                    value=""
                    placeholder="password"
                    label="password:"
                    set_value={password_changed}
                        open={true}
        perma_open={true}
                />
                    <button type="submit">{"Create Account"}</button>
            </div>
        }
 <h3> { "Go "} <Link<Route> to={Route::Home}>{"Home"}</Link<Route>> { " Or "} <Link<Route> to={Route::CloudMLogin}>{"Login"}</Link<Route>> </h3>
            </form>
        </div>
        </div>

    }
}

#[derive(Clone, Debug, Default)]
pub struct LoginAccMHsData {
    pub username: String,
    pub password: String,
}

#[function_component(Login)]
pub fn login_to_mhs() -> Html {

    let userdata = use_atom::<UserData>();
    let acc_data = use_state(||LoginAccMHsData::default());
    let storage = use_local_storage::<String>("token".to_string());


    let version = use_state(||"-.-".to_string());

    let clone_data = acc_data.clone();
    let username_changed = Callback::from(move |username: String| {
        clone_data.set(
            LoginAccMHsData {
                username,
                ..clone_data.deref().clone()
            });
    });

    let clone_data = acc_data.clone();
    let password_changed = Callback::from(move |password: String| {
        clone_data.set(
            LoginAccMHsData {
                password,
                ..clone_data.deref().clone()
            });
    });
    let clone_data = acc_data.clone();
    let clone_userdata = userdata.clone();
    let clone_storage = storage.clone();
    let state: UseAsyncHandle<String, _> = use_async(async move {
            let mut service = ServiceURL::default();
            log!("get url");
            let token = service.post_request(json!({
                    "username" : clone_data.username.clone(),
                    "password" : clone_data.password.clone(),
                }), "".to_string(), "cloudM".to_string(), "log_in_user".to_string(), "".to_string()).await;

            let res: ResultCon<String> = serde_json::from_str(&token.as_str()).unwrap();
            if res.res != "".to_string() {
                if res.res.len() > 200 {
                    let ud = UserData::init_withe_token(res.res.as_str());
                    clone_storage.set(ud.token.to_string());
                    clone_userdata.set(ud);
                }
                return Ok(res.res);
            }
        return Err("Server error")

    });


    let clone_version = version.clone();
    let get_version: UseAsyncHandle<String, _> = use_async(async move {
            let mut service = ServiceURL::default();
            let token = service.post_request(json!({}), "".to_string(), "cloudM".to_string(), "Version".to_string(), "".to_string()).await;

            let res: ResultCon<String> = serde_json::from_str(&token.as_str()).unwrap();
            if res.res != "".to_string() {
                if res.res.len()> 200 {
                    clone_version.set(res.res.as_str().to_string());
                }
                return Ok(res.res);
            }
        return Err(())
    });


    let clone_state = state.clone();
    let on_sub = Callback::from(move |event: FocusEvent| {
        event.prevent_default();
        clone_state.run();

    });

    let stylesheet = Style::new(STYLE).unwrap();
    let clone_state = get_version.clone();
    use_effect_once(move || {
        {
        let clone_state = clone_state.clone();
        log!("Get Data");
        clone_state.run();
        } move||{
        log!("");
        }
    });

    let clone_userdata = userdata.clone();

    html! {
        <div class={stylesheet}>
            <div class="login">

            <form onsubmit={on_sub}>
            <div class="login-bg">
                <h1>{"Login V:"}  if let Some(data) = &get_version.data {
                    {data}
                } </h1>
         <h3>if let Some(data) = &state.data {
                    {data}
                } </h3>
        <h3>
        if let Some(error) = &state.error {
            {error}
        }
        </h3>
            </div>
                if clone_userdata.auth.clone() {
            <div class="login">
                <h1>{"Logged in"}</h1>
                <h3>{"Username : ... "}</h3>
                <Link<Route> to={Route::Home}>{"Go Home"}</Link<Route>>
            </div>
         }else {
            <div class="content">
                        <TextInputField
            class=""
            id="username-create_acc_mhs"
            name="username"
            type_="text"
            value=""
            placeholder="Username"
            label="Username:"
            set_value={username_changed}
                open={true}
            />
                                <TextInputField
            class=""
            id="password-create_acc_mhs"
            name="password"
            type_="password"
            value=""
            placeholder="password"
            label="password:"
            set_value={password_changed}
                open={true}
        perma_open={true}

            />
            <button type="submit">{"Login"}</button>
            </div>
        }
            <h3> { "Go "} <Link<Route> to={Route::Home}>{"Home"}</Link<Route>> { " Or "}
            <Link<Route> to={Route::CloudMCreate}>{"Register"}</Link<Route>> </h3>
            </form>

        </div>
        </div>

    }
}
