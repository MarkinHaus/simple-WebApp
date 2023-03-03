use std::ops::DerefMut;

use gloo::console::log;
use serde::{Deserialize, Serialize};
use serde_json::{Error, json};
use stylist::Style;
use yew::Component;
use yew::html::Scope;
use yew::prelude::*;
use yew_router::history::History;
use yew_router::prelude::RouterScopeExt;
use crate::app::cloundm::{ResultCon, ServiceURL};
use chrono::{Datelike, Local, NaiveDate, Weekday};

use crate::app::daytree::{Task, TaskAtt};
use crate::app::router::Route;
use crate::util::text_input::TextInputField;

use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{DragEvent, Element, MouseEvent};
use web_sys::*;
use yew::{Callback, Context, Html, html, Properties};
use yew::html::ImplicitClone;
use yew::prelude::*;

use yew::prelude::*;



const STYLE: &str = include_str!("caleder.css");

#[derive(Debug, Serialize, Deserialize)]
pub struct DayTreeStateDesigner {
    pub task: Vec<Task>,
}

pub struct Designer {
    pub task: Task,
    pub edit_att: usize,
    pub content: String,
    pub info: String,
}

#[derive(Properties, PartialEq)]
pub struct DesignerProps {
    pub task: Task,
    pub token: String
}

pub enum DesignerMsg {
    SaveTask,
    DeleteTasks,
    DeleteTasksAtt,
    EditAtt(usize),
    Write(String),
    InputStream(String),
    SetInfo(String),
    AddAtt(TaskAtt),
}

impl Component for Designer {
    type Message = DesignerMsg;
    type Properties = DesignerProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            task: ctx.props().task.clone(),
            edit_att: 0,
            content: "".to_string(),
            info: "".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let history = ctx.link().history().unwrap();
        let link = ctx.link().clone();
        let token = ctx.props().token.clone();
        match msg {
            DesignerMsg::SaveTask => {
                let task = self.task.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let link = link.clone();

                    let mut service = ServiceURL::default();

                    log!(task.name.clone());

                    let res = service.post_request(json!({"task": task}), token, "daytree".to_string(),
                                                   "save_task_to_bucket".to_string(),
                                                   "".to_string()).await;
                    let result: ResultCon<String> = serde_json::from_str(&res).unwrap();
                    link.send_message(DesignerMsg::SetInfo(result.res));
                });
            }
            DesignerMsg::SetInfo(s) => {
                self.info = s;
            }
            DesignerMsg::DeleteTasks => {
                self.task = Task::default();
                link.send_message(DesignerMsg::SetInfo("Task delete".to_string()));
            }
            DesignerMsg::DeleteTasksAtt => {
                self.task.att.remove(self.edit_att);
                link.send_message(DesignerMsg::SetInfo("Task Att delete".to_string()));
            }
            DesignerMsg::EditAtt(index) => {
                self.edit_att = index;
                return false;
            }
            DesignerMsg::Write(s) => {
                if self.edit_att == 0 {
                    self.task.name = s.clone();
                }
                self.task.att[self.edit_att].v = s;
            }
            DesignerMsg::InputStream(s) => {
                self.content = s.clone();
                if s.ends_with(" ") {
                    wasm_bindgen_futures::spawn_local(async move {
                        let link = link.clone();
                        let clone_his = history.clone();
                        let mut service = ServiceURL::default();

                        let res = service.post_request(json!({"input": s}), token, "daytree".to_string(),
                                                       "designer_input".to_string(),
                                                       "".to_string()).await;
                        let result: Result<ResultCon<Vec<TaskAtt>>, _> = serde_json::from_str(&res);

                        match result {
                            Ok(res) => {
                                for task_att in res.res {
                                    link.send_message(DesignerMsg::AddAtt(task_att))
                                };
                            }
                            Err(_) => {
                                let result: ResultCon<String> = serde_json::from_str(&res).unwrap();
                                link.send_message(DesignerMsg::SetInfo(result.res));
                            }
                        }
                    });
                }
            }
            DesignerMsg::AddAtt(mut att) => {
                if att.t == "self.content" {
                    att.t = self.content.clone();
                }
                let mut b = true;
                let new_task_att = self.task.att.clone();
                let mut i = 0;
                for task_a in &new_task_att {
                    if task_a.t == att.t {
                        self.task.att.remove(i.clone());
                        self.task.att.insert(i.clone(), att.clone());
                        b = false;
                    }
                    i += 1;
                }
                if b {
                    self.task.att.push(att);
                }
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let clone_link = ctx.link().clone();
        let save_callback = Callback::from(move |_| {
            clone_link.send_message(DesignerMsg::SaveTask)
        });

        let clone_link = ctx.link().clone();
        let delete_callback = Callback::from(move |_| {
            clone_link.send_message(DesignerMsg::DeleteTasks)
        });

        let clone_link = ctx.link().clone();
        let write_callback = Callback::from(move |s: String| {
            clone_link.send_message(DesignerMsg::InputStream(s))
        });

        let clone_link = ctx.link().clone();
        let add = Callback::from(move |_| {
            clone_link.send_message(DesignerMsg::AddAtt(TaskAtt { t: "self.content".to_string(), v: "".to_string() }));
        });

        let mut i = 0;
        let task_att = self.task.att.iter().map(|_t| {
            i += 1;
            self.show_task_att(ctx.link(), i - 1)
        });

        let stylesheet = Style::new(STYLE).unwrap();

        html! {
            <div class={stylesheet} id="root-div-d">
            <h1>{self.info.clone()}</h1>
                <div class="designer">

                    <h1>{self.task.name.clone()}</h1>

                    <div class="input-content">
            <TextInputField
                        class=""
                        id=""
                        name=""
                        type_="text"
                        value={self.content.clone()}
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
}

impl Designer {
    pub fn show_task_att(&self, link: &Scope<Self>, index: usize) -> Html {
        let task_att = self.task.att[index.clone()].clone();

        let clone_link = link.clone();
        let set_value = Callback::from(move |s: String| {
            clone_link.send_message(DesignerMsg::EditAtt(index));
            clone_link.send_message(DesignerMsg::Write(s))
        });

        let clone_link = link.clone();
        let del = Callback::from(move |_| {
            clone_link.send_message(DesignerMsg::EditAtt(index));
            clone_link.send_message(DesignerMsg::DeleteTasksAtt)
        });

        let v = task_att.v;
        let t = task_att.t;

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
}

pub struct Calender2 {
    pub week_tasks: Vec<Vec<Task>>,
    pub infobox: String,
    pub if_drop_from: Vec<i32>,
}

pub enum CalenderMsg {
    SetTasks(Vec<Vec<Task>>),
    SaveTasks,
    RegisterDrag(usize, usize, Task),
    AddTaskO(usize, usize),
    Delete(usize, usize),
    NoTaskFound,
    ErrorFindigTasks,
    EditTask(Task)
}

#[derive(Properties, PartialEq)]
pub struct CalenderProps {
    pub edit_task: Callback<Task>,
    pub token: String
}


impl Component for Calender2 {
    type Message = CalenderMsg;
    type Properties = CalenderProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            week_tasks: vec![
                vec![]
                , vec![]
                , vec![]
                , vec![]
                , vec![]
                , vec![]
                , vec![]],
            infobox: "".to_string(),
            if_drop_from: vec![-1, -1],
        }
    }


    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        let history = ctx.link().history().unwrap();
        let callback = ctx.props().edit_task.clone();
        let token = ctx.props().token.clone();
        match msg {
            CalenderMsg::SetTasks(week_tasks) => {
                self.week_tasks = week_tasks.clone();
                return true;
            }
            CalenderMsg::EditTask(task) => {
                callback.emit(task);
                return true;
            }
            CalenderMsg::SaveTasks => {
                let task = self.week_tasks.clone();
                let clone_link = link.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let clone_his = history.clone();

                    let mut service = ServiceURL::default();

                    let data = service.post_request(json!({"week":task.clone()}), token, "dayTree".to_string(),
                                                    "save_task_week".to_string(),
                                                    "".to_string()).await;
                    let result: Result<ResultCon<Vec<Vec<Task>>>, _> = serde_json::from_str(&data);
                    match result {
                        Ok(res) => {
                            let tasks = res.res;
                            if tasks.is_empty() {
                                clone_link.send_message(CalenderMsg::NoTaskFound)
                            } else {
                                clone_link.send_message(CalenderMsg::SetTasks(tasks))
                            }
                        }
                        Err(_) => {
                            clone_link.send_message(CalenderMsg::ErrorFindigTasks)
                        }
                    }
                });

                return false;
            }
            CalenderMsg::RegisterDrag(index, day, task) => {
                if self.if_drop_from == vec![-1, -1] {
                    log!("RegisterDrag from ", self.if_drop_from[1], self.if_drop_from[0], " to ", day.clone(), index);
                    self.if_drop_from = vec![index as i32, day as i32];
                }
            }
            CalenderMsg::AddTaskO(index, day) => {
                if self.if_drop_from == vec![index as i32, day as i32] {
                    self.if_drop_from = vec![-1, -1];
                    return true;
                } else if self.if_drop_from != vec![-1, -1] {
                    log!("Appanding Index F ", self.if_drop_from[1], self.if_drop_from[0], " to ", day.clone(), index);
                    let mut week = self.week_tasks.clone();
                    let task = week[self.if_drop_from[1] as usize].remove(self.if_drop_from[0] as usize - 1);
                    //week[day].remove(index);
                    // week[day].insert(index - 1, task.clone());
                    week[day].insert(index, task.clone());
                    self.if_drop_from = vec![-1, -1];
                    link.send_message(CalenderMsg::SetTasks(week));
                } else {
                    log!("NO match found")
                }
                log!("TRY Index AddTaskO", self.if_drop_from[0], self.if_drop_from[1], " to ", day.clone(), index, self.week_tasks.len(), self.week_tasks[2].len());
            }
            CalenderMsg::NoTaskFound => { self.infobox = "NoTaskFound".to_string(); }
            CalenderMsg::ErrorFindigTasks => { self.infobox = "ErrorFindigTasks".to_string(); }
            CalenderMsg::Delete(index, day) => {
                let mut week = self.week_tasks.clone();
                week[day].remove(index);
                link.send_message(CalenderMsg::SetTasks(week));
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let edit_t = ctx.props().edit_task.clone();
        //let save_callback = Callback::from(move |_| {
        //    link.send_message(CalenderMsg::SaveTask)
        //});
        let mut i = 0;
        let mo = self.week_tasks[0].iter().map(|t| {
            i += 1;
            self.show_day(&link, i, t, 0, edit_t.clone())
        });

        let mut i = 0;
        let di = self.week_tasks[1].iter().map(|t| {
            i += 1;
            self.show_day(&link, i, t, 1, edit_t.clone())
        });
        let mut i = 0;
        let mi = self.week_tasks[2].iter().map(|t| {
            i += 1;
            self.show_day(&link, i, t, 2, edit_t.clone())
        });
        let mut i = 0;
        let do_ = self.week_tasks[3].iter().map(|t| {
            i += 1;
            self.show_day(&link, i, t, 3, edit_t.clone())
        });
        let mut i = 0;
        let fr = self.week_tasks[4].iter().map(|t| {
            i += 1;
            self.show_day(&link, i, t, 4, edit_t.clone())
        });
        let mut i = 0;
        let sa = self.week_tasks[5].iter().map(|t| {
            i += 1;
            self.show_day(&link, i, t, 5, edit_t.clone())
        });
        let mut i = 0;
        let so = self.week_tasks[6].iter().map(|t| {
            i += 1;
            self.show_day(&link, i, t, 6, edit_t.clone())
        });

        let clone_link = link.clone();
        let ondrop_mo = Callback::from(move |_| { clone_link.send_message(CalenderMsg::AddTaskO(0, 0)) });
        let clone_link = link.clone();
        let ondrop_di = Callback::from(move |_| { clone_link.send_message(CalenderMsg::AddTaskO(0, 1)) });
        let clone_link = link.clone();
        let ondrop_mi = Callback::from(move |_| { clone_link.send_message(CalenderMsg::AddTaskO(0, 2)) });
        let clone_link = link.clone();
        let ondrop_do = Callback::from(move |_| { clone_link.send_message(CalenderMsg::AddTaskO(0, 3)) });
        let clone_link = link.clone();
        let ondrop_fr = Callback::from(move |_| { clone_link.send_message(CalenderMsg::AddTaskO(0, 4)) });
        let clone_link = link.clone();
        let ondrop_sa = Callback::from(move |_| { clone_link.send_message(CalenderMsg::AddTaskO(0, 5)) });
        let clone_link = link.clone();
        let ondrop_so = Callback::from(move |_| { clone_link.send_message(CalenderMsg::AddTaskO(0, 6)) });
        let clone_link = link.clone();
        let save = Callback::from(move |_| { clone_link.send_message(CalenderMsg::SaveTasks) });

        let stylesheet = Style::new(STYLE).unwrap();

        html! {
            <div class={stylesheet} id="root-div-c">
                <button onclick={save}> {"Save"} </button>
                <div class="calender-container">
                    <h3>{&self.infobox}</h3>
                    <div class="calender-header">
                        <span>{"Mo."}</span>
                        <span>{"Di."}</span>
                        <span>{"Mi."}</span>
                        <span>{"Do."}</span>
                        <span>{"Fr."}</span>
                        <span>{"Sa."}</span>
                        <span>{"So."}</span>
                    </div>

                    <div class="calender-content">
                        <div class="divAE" >if self.if_drop_from != vec![-1 as i32,-1 as i32] {<div onclick={ondrop_mo.clone()}><h2>{"add"}</h2><h1>{"mo"}</h1></div>}{for mo}</div>
                        <div class="divAE" >if self.if_drop_from != vec![-1 as i32,-1 as i32] {<div onclick={ondrop_di.clone()}><h2>{"add"}</h2><h1>{"di"}</h1></div>}{for di}</div>
                        <div class="divAE" >if self.if_drop_from != vec![-1 as i32,-1 as i32] {<div onclick={ondrop_mi.clone()}><h2>{"add"}</h2><h1>{"mi"}</h1></div>}{for mi}</div>
                        <div class="divAE" >if self.if_drop_from != vec![-1 as i32,-1 as i32] {<div onclick={ondrop_do.clone()}><h2>{"add"}</h2><h1>{"do"}</h1></div>}{for do_}</div>
                        <div class="divAE" >if self.if_drop_from != vec![-1 as i32,-1 as i32] {<div onclick={ondrop_fr.clone()}><h2>{"add"}</h2><h1>{"fr"}</h1></div>}{for fr}</div>
                        <div class="divAE" >if self.if_drop_from != vec![-1 as i32,-1 as i32] {<div onclick={ondrop_sa.clone()}><h2>{"add"}</h2><h1>{"sa"}</h1></div>}{for sa}</div>
                        <div class="divAE" >if self.if_drop_from != vec![-1 as i32,-1 as i32] {<div onclick={ondrop_so.clone()}><h2>{"add"}</h2><h1>{"so"}</h1></div>}{for so}</div>
                    </div>

                </div>

            </div>
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let link = ctx.link().clone();

            let token = ctx.props().token.clone();


            wasm_bindgen_futures::spawn_local(async move {
                let clone_link = link.clone();

                let mut service = ServiceURL::default();

                let data = service.post_request(json!({}), token, "daytree".to_string(),
                                                "get_bucket_week".to_string(),
                                                "".to_string()).await;

                let result: Result<ResultCon<Vec<Vec<Task>>>, _> = serde_json::from_str(&data);
                log!("TASKFF");
                match result {
                    Ok(res) => {
                        let tasks = res.res;
                        if tasks.is_empty() {
                            clone_link.send_message(CalenderMsg::NoTaskFound)
                        } else {
                        log!("TASK DA");
                            clone_link.send_message(CalenderMsg::SetTasks(tasks))
                        }
                    }
                    Err(e) => {
                        clone_link.send_message(CalenderMsg::ErrorFindigTasks)
                    }
                }
            });
        }
    }
}

impl Calender2 {
    pub fn show_task_att(&self, link: &Scope<Self>, att: &TaskAtt) -> Html {
        let task_att = att;

        let link = link.clone();
        let set_value = Callback::from(move |s: String| {
            //link.send_message(DesignerMsg::EditAtt(index));
            //link.send_message(DesignerMsg::Write(s))
        });

        return html! {
            <div class="TaskCard">
                <h3>{task_att.t.clone()} {":"} {task_att.v.clone()}</h3>
            </div>
        };
    }
    pub fn show_day(&self, link: &Scope<Self>, index: usize, task: &Task, day: usize, edit: Callback<Task>) -> Html {
        let clone_link = link.clone();
        let clone_day = day.clone();
        let delete = Callback::from(move |_| {
            log!("delete");
            clone_link.send_message(CalenderMsg::Delete(index - 1, clone_day));
        });
        let clone_link = link.clone();
        let clone_task = task.clone();
        let clone_day = day.clone();
        let register_drag = Callback::from(move |e: MouseEvent| {
            log!(index.clone(), clone_day.clone(), "RegisterDrag");
            e.prevent_default();
            clone_link.send_message(CalenderMsg::RegisterDrag(index, clone_day, clone_task.clone()));
        });

        let clone_link = link.clone();
        let clone_day = day.clone();
        let ondrop = Callback::from(move |_| {
            log!(index.clone(), clone_day.clone(), "ondrop");
            clone_link.send_message(CalenderMsg::AddTaskO(index - 1, clone_day));
        });

        let clone_edit = edit.clone();
        let c_task = task.clone();
        let edit_task =Callback::from(move |_| {
            clone_edit.emit(c_task.clone());
        });

        // oc -> wer und o ich bin
        // on over -> wohin ich will
        // release ausfuühren
        log!("Sel:", self.if_drop_from[0]);
        let clone_task = task.clone();
        let clone_link = link.clone();
        let mut i = 0;
        let task_att = task.att.iter().map(move |a| {
            i += 1;
            self.show_task_att(&clone_link, a)
        });
        return html!(
            <div class="Task">
                <h3>{task.name.clone()}</h3>
            if self.if_drop_from == vec![-1 as i32,-1 as i32] {
                <button class={"ms-button"} onclick={register_drag.clone()}> {"Select"} </button>
                <button class={"ms-button"} onclick={delete}> {"Delete"} </button>
            }else if self.if_drop_from == vec![index as i32, clone_day as i32]  {
                <div class="TaskCard">
                    {for task_att}
                    <button class={"ms-button"} onclick={edit_task.clone()}> {"edit"} </button>
                    <button class={"ms-button"} onclick={ondrop.clone()}> {"save"} </button>
                    <button class={"ms-button"} onclick={delete}> {"Delete"} </button>
                </div>
            }
            else {
                <button class={"ms-button"} onclick={ondrop}> {"Paste"} </button>
            }
            </div>
        );
    }
}

struct Hour {
    hour: u8,
    tasks: Vec<Task>,
}
pub struct Calendar {
    link: Scope<Self>,
    date: NaiveDate,
    selected_day: Option<NaiveDate>,
    hours: Vec<Hour>,
    other_tasks: Vec<Task>,
}

pub enum CalendarMsg {
    SelectDay(NaiveDate),
    UnselectDay,
}
impl Calendar {
       pub fn show_days_calendar(&self) -> Vec<NaiveDate> {
        let mut days =Vec::new();
        let first_day_of_month = self.date.with_day(1).unwrap();
        let mut first_day_of_calendar = first_day_of_month;
        let mut day_of_week = first_day_of_month.weekday();

        while day_of_week != Weekday::Mon {
            days.push(first_day_of_calendar);
            first_day_of_calendar = first_day_of_calendar.pred_opt().unwrap();
            day_of_week = day_of_week.pred();
        }
        days.push(first_day_of_calendar);
        days.reverse();
        let mut day = first_day_of_month.succ_opt().unwrap();
        while day.month() == self.date.month() {
            days.push(day);
            day = day.succ_opt().unwrap();
        }

        let mut day = day;
        while days.len() % 7 != 0 {
            days.push(day);
            day = day.succ_opt().unwrap();
        }
           return days
    }
}

fn get_tasks_for_day(day: usize) -> Vec<Task> {
    vec![Task { name: "Task 1".to_string(), att: vec![TaskAtt{ t: "time".to_string(), v: "08:00".to_string() }] },
         Task { name: "Task 2".to_string(), att: vec![TaskAtt{ t: "time".to_string(), v: "09:00".to_string() }] },
         Task { name: "Task 3".to_string(), att: vec![TaskAtt{ t: "time".to_string(), v: "09:30".to_string() }] },
         Task { name: "Task 4".to_string(), att: vec![TaskAtt{ t: "time".to_string(), v: "10:00".to_string() }] },
         Task { name: "Task 5".to_string(), att: vec![] },
    ]
}

impl Component for Calendar {
    type Message = CalendarMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let now = Local::now();
        let mut date = now.date_naive();
        Self { link:ctx.link().clone(), date, selected_day: None, hours: Vec::new(), other_tasks: Vec::new()}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CalendarMsg::SelectDay(day) => {
                let d_m = day.month();
                let d_d = day.day();
                if d_m != self.date.month() {
                    return false;
                }
                if d_d < self.date.day() {
                    return false;
                }
                self.selected_day = Some(day);
                self.hours = Vec::new();
                self.other_tasks = Vec::new();

               for task in get_tasks_for_day(d_d as usize) {
                   let hour_task_attributes: Vec<&TaskAtt> = task.att.iter().filter(|a| a.t == "time").collect();
                   if !hour_task_attributes.is_empty() {
                       let hour = hour_task_attributes[0].v[..2].parse::<u8>().unwrap();
                       let mut hour_tasks = self.hours.iter_mut().find(|h| h.hour == hour);
                       match hour_tasks {
                           Some(hour_tasks) => hour_tasks.tasks.push(task),
                           None => {

                               self.hours.push(Hour { hour, tasks: vec![task] });

                           }
                       }
                   } else {
                       self.other_tasks.push(task);
                   }
               }
                log!(self.hours.len());
            self.hours.sort_by(|a, b| a.hour.cmp(&b.hour));
                log!("self.hours.len()");
                true
            }
            CalendarMsg::UnselectDay => {
                self.selected_day = None;
                true
            }
        }
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut days =self.show_days_calendar();
        let link = ctx.link().clone();


        let clone_link = link.clone();
        let select_day = |day: NaiveDate| Callback::from(move |_| {
            clone_link.send_message(CalendarMsg::SelectDay(day)) });
        let clone_link = link.clone();
        let unselect_day =  Callback::from(move |_| { clone_link.send_message(CalendarMsg::UnselectDay) });
        log!(self.hours.len());

        let stylesheet = Style::new(STYLE).unwrap();
        html! {
            <div class={stylesheet}>
            <h1 onclick={unselect_day}>{"Calendar"}</h1>
             if let Some(day) = &self.selected_day {
                    <div>
                        <h2>{"Tasks for day "}{day.day()}</h2>
                        <table>
                            {for self.hours.iter().map(|hour| {
                                html! {
                                    <tr>
                                        <td>{hour.hour}{":00"}</td>
                                        <td>
                                            {for hour.tasks.iter().map(|task| {
                                                html! {
                                                    <div>{task.name.clone()}</div>
                                                }
                                            })}
                                        </td>
                                    </tr>
                                }
                            })}
                        </table>
                        {if !self.other_tasks.is_empty() {
                            html! {
                                <div>
                                    <h3>{"Other tasks"}</h3>
                                    {for self.other_tasks.iter().map(|task| {
                                        html! {
                                            <div>{task.name.clone()}</div>
                                        }
                                    })}
                                </div>
                            }
                        } else {
                            html! {}
                        }}
                    </div>
            }else {
                <table>
                <thead>
                    <tr>
                        <th>{"Mo"}</th>
                        <th>{"Tu"}</th>
                        <th>{"We"}</th>
                        <th>{"Th"}</th>
                        <th>{"Fr"}</th>
                        <th>{"Sa"}</th>
                        <th>{"Su"}</th>
                    </tr>
                </thead>
                <tbody>
                    {for days.chunks(7).map(|week| {
                        html! {
                            <tr>
                                {for week.iter().map(|day| {
                                    let class1 = if day.day() < self.date.day(){
                                        "after-day"
                                    } else {
                                        ""
                                    };
                                    html! {
                                        <td class={classes!("td-day")}> <button onclick={select_day.clone()(day.clone())}>{day.day()}</button></td>
                                    }
                                })}
                            </tr>
                        }
                    })}
                </tbody>
            </table>
            }

            </div>
        }
    }
}



