use std::borrow::{Borrow, BorrowMut};
use std::fmt;
use std::ops::Deref;
use bounce::*;
use gloo::console::log;
use serde_json::json;
use stylist::Style;
use bounce::use_atom;
use chrono::{Local, NaiveDate};

use yew::{Callback, Html, html, use_state, function_component, classes, Properties};
use yew_hooks::{use_async, use_effect_once, use_local_storage, UseAsyncHandle, UseLocalStorageHandle, UseSetHandle};
use yew_router::history::History;
use yew_router::hooks::use_history;
use crate::app::cloundm::{ResultCon, ServiceURL};
use crate::app::router::Route;
use crate::app::userdata::{DayTreeData, QuickNoteData, UserData};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use yew_router::prelude::Link;
use web_sys::{Event, HtmlTextAreaElement, HtmlSelectElement, HtmlInputElement};
use crate::util::text_input::{TextInputField, TextInputStagesMsg};
use crate::app::dt_classes::{Designer};
use crate::app::dt_now::DayTreeNow;
use crate::app::dt_classes::Calendar;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TaskAtt {
    pub t: String,
    pub v: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Task {
    pub name: String,
    pub att: Vec<TaskAtt>,
}

impl Task {
    pub fn default() -> Task {
        let att = TaskAtt { t: "Name".to_string(), v: "".to_string() };
        Task {
            name: "TEST".to_string(),
            att: vec![att],
        }
    }
}

const STYLE: &str = include_str!("daytree.css");


#[function_component(DayTreeApp)]
pub fn quick_note() -> Html {

    let userdata = use_atom::<UserData>();

    let clone_userdata = userdata.clone();
    let storage = use_local_storage::<String>("token".to_string());
    let tok =  &*storage;
    if let Some(token) = tok {
        if !clone_userdata.auth {
            clone_userdata.set(UserData::init_withe_token(token.as_str()));
        }
    }


    let tabs_n = use_state(|| "ds".to_string());
    let e_task = use_state(|| Task::default());

    let clone_e_task = e_task.clone();
    let clone_tabs_n = tabs_n.clone();
    let set_edit = Callback::from(move | t :Task| {
        clone_e_task.set(t);
        clone_tabs_n.set("ds".to_string());
    });



    let clone_tabs_n = tabs_n.clone();
    let open_ds = Callback::from(move |_| {
        clone_tabs_n.set("ds".to_string());
    });
    let clone_tabs_n = tabs_n.clone();
    let open_nw = Callback::from(move |_| {
        clone_tabs_n.set("nw".to_string());
    });
    let clone_tabs_n = tabs_n.clone();
    let open_ca = Callback::from(move |_| {
        clone_tabs_n.set("ca".to_string());
    });
    let clone_tabs_n = tabs_n.clone();
    let open_pa = Callback::from(move |_| {
        clone_tabs_n.set("pa".to_string());
    });
    let clone_tabs_n = tabs_n.clone();
    let open_op = Callback::from(move |_| {
        clone_tabs_n.set("op".to_string());
    });

    let mut classes_var = vec!["".to_string(),"ac".to_string(),"".to_string(),"".to_string(),"".to_string()];
    let val_tab = tabs_n.deref().clone();
    if val_tab=="nw".to_string() {
        classes_var = vec!["ac".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string()];
    }
    if val_tab=="ca".to_string() {
        classes_var = vec!["".to_string(),"".to_string(),"ac".to_string(),"".to_string(),"".to_string()];
    }
    if val_tab=="pa".to_string() {
        classes_var = vec!["".to_string(),"".to_string(),"".to_string(),"ac".to_string(),"".to_string()];
    }
    if val_tab=="op".to_string() {
        classes_var = vec!["".to_string(),"".to_string(),"".to_string(),"".to_string(),"ac".to_string()];
    }

    let stylesheet = Style::new(STYLE).unwrap();
    let clone_tabs_n = tabs_n.deref().clone();
    let token = userdata.token.clone();
    let task = e_task.deref().clone();


    return html! {
        <div class={stylesheet}>
            <div class="Daytree">
                <div class="login-bg">
                    <h1>{"DayTree"}</h1>
                </div>
                <div class="header">
                    <div class={classes!("header-nav",classes_var[0].clone())} onclick={open_nw.clone()}> <h3>{"Now"}</h3></div>
                    <div class={classes!("header-nav",classes_var[1].clone())} onclick={open_ds.clone()}> <h3>{"Designer"}</h3></div>
                    <div class={classes!("header-nav",classes_var[2].clone())} onclick={open_ca.clone()}> <h3>{"Calender"}</h3></div>
                    <div class={classes!("header-nav",classes_var[3].clone())} onclick={open_pa.clone()}> <h3>{"Past"}</h3></div>
                    <div class={classes!("header-nav",classes_var[4].clone())} onclick={open_op.clone()}> <h3>{"Options"}</h3></div>
                </div>

            <div>
            if clone_tabs_n=="nw".to_string() {
                <DayTreeNow token={token.clone()}/> // TODO: Now
            }
            if clone_tabs_n=="ds".to_string() {
                <h1>{"Designer"}</h1>
                <Designer task={task} token={token.clone()}/>
            }
            if clone_tabs_n=="ca".to_string() {
             <Calendar/>
                //<Calender edit_task={set_edit} token={token.clone()}/>
            }
            if clone_tabs_n=="pa".to_string() {
                <h1>{"Past"}</h1> // TODO: Past
            }
            if clone_tabs_n=="op".to_string() {
                <h1>{"Options"}</h1> // TODO: Options
            }
            </div>
                <h3> { "Go "} <Link<Route> to={Route::Home}>{"Home "}</Link<Route>> </h3>
            </div>
        </div>
    }

}

/*
#[function_component(Designer)]
pub fn designer_component() -> Html {

    let mut atom_data = use_atom::<DayTreeData>();
    let mut task_data = atom_data.tasks.clone();
    let mut clone_atom_data = atom_data.clone();
    if atom_data.tasks.len() == 0{
        task_data.push(Task::default());
        atom_data.set(DayTreeData{tasks:task_data, ..clone_atom_data.deref().clone()});

    };

    let content = use_state(|| "".to_string());

    log!("len atom data : ",atom_data.tasks.len());
    let save_callback = Callback::from(move |_| {
        log!("save_callback");
        //tasks.push(Task::default());
    });

    let delete_callback = Callback::from(move |_| {

        log!("delete_callback");
        // tasks.remove(data_value.len()-1);
        // tasks.push(Task::default());
    });

    let clone_content = content.clone();
    let write_callback = Callback::from(move |s: String| {
        log!(&s);
        clone_content.set(s);
    });


    let add = Callback::from(move |_| {
        //let mut tasks = data_value.clone();
        log!("add");
        //let id = tasks.len()-1;
        //let mut task = tasks[id].clone();
        //let mut b = true;
        //let mut i = 0;
        //let s = clone_content.deref().clone();
        //let att = TaskAtt { t: s, v: "".to_string() };
        //let new_task_att = task.att.clone();
        //for task_a in &new_task_att {
        //    if task_a.t == att.t {
        //        task.att.remove(i.clone());
        //        task.att.insert(i.clone(), att.clone());
        //        b = false;
        //    }
        //    i += 1;
        //}
        //if b {
        //    task.att.push(att);
        //}
        //tasks.remove(id);
        //tasks.insert(id, task);
        //clone_data_value.set(tasks.clone());
    });

    let mut task: Task;
    let mut id= 0;
    let mut tasks = atom_data.tasks.clone();
    if !tasks.is_empty() {
        id = tasks.len()-1;
        task = tasks[id].clone();
    }else {
        task = Task::default()
    }

    let mut i = 0;
    let task_att = task.att.iter().map(|_: _| {
        i += 1;
        return html! { <TaskAttCopm task_id={id} att_index={i-1 as usize}/>}
    });
    task.name = task.att[0].v.clone();
    let name = task.name;

    let c = content.deref().clone();
    return html!{

         <div id="root-div">
                <div class="designer">

                    <h1>{name}</h1>

                    <div class="input-content">
                    <TextInputField
                        class=""
                        id=""
                        name=""
                        type_="text"
                        value={c}
                        placeholder="Input"
                        label="Input:"
                        set_value={write_callback.clone()}
                        open={true}
                        perma_open={true}
                    />
                    </div>

                    <div class="output-content">
                        {for task_att}
                    <div onclick={add.clone()} class="att-button"><h2>{"add"}</h2></div>
                    </div>

                    <div class="button-content">
                        <button class="button-Save" onclick={save_callback}> {"Save"} </button>
                        <button class="button-Delete" onclick={delete_callback}> {"Delete"} </button>
                    </div>
                </div>
            </div>

    }

}

#[derive(Properties, PartialEq)]
pub struct TaskAttProps{
    pub task_id: usize,
    pub att_index: usize,
}


#[function_component(TaskAttCopm)]
pub fn task_att(props: &TaskAttProps) -> Html {

        let task_index = props.task_id;
        let att_index = props.att_index;

        let day_tree_data = use_atom::<DayTreeData>();

        log!("len:", day_tree_data.tasks.len());

        let data_value_t = day_tree_data.clone();

        let data_value = day_tree_data.tasks.clone();
        let set_value = Callback::from(move |s: String| {
            log!("set_value");
            let mut data = data_value.clone();
            let task = data_value[task_index].clone();
            let mut att = task.att[att_index].clone();
            att.v = s;
            data.remove(task_index);
            data.insert(task_index, task);
            day_tree_data.set(DayTreeData{tasks:data, ..data_value_t.deref().clone()});

        });

        let day_tree_data = use_atom::<DayTreeData>();
        let data_value_t = day_tree_data.clone();
        let data_value = data_value_t.tasks.clone();
        let del = Callback::from(move |_| {
            log!("del");
            let mut task = data_value[task_index].clone();
            task.att.remove(att_index);
            day_tree_data.set(DayTreeData{
                tasks: data_value.clone(),
                ..day_tree_data.deref().clone()
            });
        });

        let day_tree_data = use_atom::<DayTreeData>();
        let data_value_t = day_tree_data.clone();
        let data_value = data_value_t.tasks.clone();
        log!(task_index, "task_att_index");
        let task = data_value[task_index].clone();
        let mut att = task.att[att_index].clone();
        let v = att.v;
        let t = att.t;

    return html!{
            <div class="TaskCard">
                            <TextInputField
                        class=""
                        id=""
                        name=""
                        type_="text"
                        value={v.clone()}
                        placeholder={t.clone()}
                        label={t.clone()+" : "}
                        set_value={set_value}
                        open={true}
                    />
                <div onclick={del.clone()} class="rem-button"><h2>{"Remove"}</h2></div>
            </div>
    }

}
*/
