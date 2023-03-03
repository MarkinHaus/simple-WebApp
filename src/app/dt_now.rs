// TODO : Get Imports

// function_component DayTreeNow
// TODO : Get , Save Date
// TODO : show TaskCards - in sections Morning(6-10) before noon(10-12) at noon(12-14) afternoon(14-18) evening(18-23) late evening(23-6)
use now::DateTimeNow;
use std::ops::Deref;
use bounce::use_atom;
use gloo::console::log;
use serde_json::json;
use stylist::Style;
use crate::app::daytree::{Task, TaskAtt};
use yew::{Callback, Html, html, use_state, function_component, classes, Properties};
use yew_hooks::{use_async, use_effect_once, UseAsyncHandle};
use yew_router::history::History;
use yew_router::prelude::use_history;
use crate::app::cloundm::{ResultCon, ServiceURL};
use crate::app::router::Route;
use crate::app::userdata::UserData;

const STYLE: &str = include_str!("dt_now.css");
#[derive(Properties, PartialEq)]
pub struct DayTreeNowProps{
    pub token: String
}

#[function_component(DayTreeNow)]
pub fn now_dt(props : &DayTreeNowProps)-> Html{
let stylesheet = Style::new(STYLE).unwrap();
    let tasks = use_state(|| vec![]);
    let userdata = use_atom::<UserData>();
    let token = props.token.clone();

    let c_token = token.clone();
    let clone_userdata = userdata.clone();
    let his = use_history().unwrap();
    let clone_tasks = tasks.clone();
    let get_asy: UseAsyncHandle<String, _> = use_async(async move {
        let clone_his = his.clone();
        let clone_tasks = clone_tasks.clone();
        if clone_userdata.auth {
            let mut service = ServiceURL::default();
            let data = service.post_request(json!({}), c_token, "daytree".to_string(),
                                            "get_bucket_today".to_string(),
                                            "".to_string()).await;
            let result: Result<ResultCon<Vec<Task>>, _> = serde_json::from_str(&data);

            match result {
                    Ok(res) => {
                        clone_tasks.set(res.res);
                    }
                    Err(e) => {
                        log!(e.to_string());
                        return Err(e.to_string());
                    }
                };
            return  Ok("".to_string())
        }else {
            clone_his.push(Route::CloudMLogin);
        }
        return Err("???".to_string());
    });
    let c_token = token.clone();
    let clone_userdata = userdata.clone();
    let his = use_history().unwrap();
    let clone_tasks = tasks.clone();
    let save_asy: UseAsyncHandle<String, _> = use_async(async move {
        let clone_his = his.clone();
        let clone_tasks = clone_tasks.clone();
        if clone_userdata.auth {
            let mut service = ServiceURL::default();
            let tasks_ = clone_tasks.deref().clone();
            let data = service.post_request(json!({"task":tasks_}), c_token, "daytree".to_string(),
                                            "save_task_day".to_string(),
                                            "".to_string()).await;
            let result: Result<ResultCon<Vec<Task>>, _> = serde_json::from_str(&data);

            match result {
                    Ok(res) => {
                        clone_tasks.set(res.res);
                    }
                    Err(e) => {
                        log!(e.to_string());
                        return Err(e.to_string());
                    }
                };
            return  Ok("".to_string())
        }else {
            clone_his.push(Route::CloudMLogin);
        }
        return Err("???".to_string());
    });



    let clone_get_asy = get_asy.clone();
    let clone_save_asy = save_asy.clone();
    use_effect_once(move || {
        {
        log!("Get Data");
        clone_get_asy.run();
        } move||{
        log!("save data");
        clone_save_asy.run();
        }
    });

    let clone_save_asy = save_asy.clone();
    let callback_refresh = Callback::from(move | _:_| {
        log!("Refresh");
         clone_save_asy.run();
    });
    let callback_action = Callback::from(move | index:usize| {
        log!("Action", index);
    });
    let c_tasks = tasks.clone();
    let callback_done = Callback::from(move | index:usize| {
        log!("Done", index);
        let mut t = c_tasks.deref().clone();
        t.remove(index);
        c_tasks.set(t);
    });
    let c_tasks = tasks.clone();
    let callback_delay = Callback::from(move |index:usize| {
        log!("Delay", index);
        let mut t = c_tasks.deref().clone();
        let task = t.remove(index);
        t.insert(c_tasks.len()-1, task);
        c_tasks.set(t);
    });
    let c_tasks = tasks.clone();
    let callback_delete = Callback::from(move | index:usize| {
        log!("Delete", index);
        let mut t = c_tasks.deref().clone();
        t.remove(index);
        c_tasks.set(t);
    });
    let tasks_vec = tasks.deref().clone();
    let mut i = 0;
    let task_cards =tasks_vec.iter().map(|t| {
        i += 1;
        return html!{
        <DayTreeTaskCard t={t.clone()} index={i} callback_action={callback_action.clone()} callback_done={callback_done.clone()} callback_delay={callback_delay.clone()} callback_delete={callback_delete.clone()}/>
    }
    });

    let sums: usize = tasks.len();
    //let now_ = Utc::now();
    //let now = String::from(now_.beginning_of_minute);
    return html!{
<div class={stylesheet} id="root-div">
              <button onclick={callback_refresh}> {"Refresh"} </button>
    <div class="container">
        <div class="timeline">
            <div class="timeline-month">
{":"}
                <span>{sums}{" Entries"}</span>
            </div>
            <div class="timeline-section">
                <div class="row">
                    <div class="col-sm-4">
                        {for task_cards}
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>
    }
}

// function_component DayTreeTaskCard
// TODO : show TaskCards
// icon | Name
// description att
// action button?
// footer | Dateline -> higher key
#[derive(Properties, PartialEq)]
pub struct TaskCardProps{
    pub t: Task,
    pub index: usize,
    pub callback_action: Callback<usize>,
    pub callback_done: Callback<usize>,
    pub callback_delay: Callback<usize>,
    pub callback_delete: Callback<usize>,
}

#[function_component(DayTreeTaskCard)]
pub fn task_card(props: &TaskCardProps) -> Html {
    let index = props.index.clone();
    let clone_call = props.callback_action.clone();
    let action = Callback::from(move |_| {
        log!("action");
        clone_call.emit(index-1);
    });
    let clone_call = props.callback_done.clone();
    let done = Callback::from(move |_| {
        log!("done");
        clone_call.emit(index-1);
    });
    let clone_call = props.callback_delay.clone();
    let delay = Callback::from(move |_| {
        log!("delay");
        clone_call.emit(index-1);
    });
    let clone_call = props.callback_delete.clone();
    let delete = Callback::from(move |_| {
        log!("delete");
        clone_call.emit(index-1);
    });

    let mut i = 0;
    let att = props.t.att.clone();
    let att_html =att.iter().map(|t| {
        i += 1;
        return html!{
            <DayTreeTaskCardAtt a={t.clone()} index={i}/>
        }
    });

    let mut do_att = false;
    if att.len() >= 2 {
        do_att = att[1].t == "A";
    }

    let task = props.t.clone();
    return html!{
          <div class="timeline-box">
            <div class="box-title">
              <i class="fa fa-asterisk text-success" aria-hidden="true"></i> {task.name}
            </div>
            <div class="box-content">
                if do_att {
                    <button onclick={action}> {"Acton"} </button>
                }
              {for att_html}
            </div>
            <div class="box-footer">
                <div class="buttons">
                    <button onclick={done}> {"Done"}</button><button onclick={delay}>{"Delay"}</button><button onclick={delete}>{"Delete"}</button>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TaskAttProps{
    pub a: TaskAtt,
    pub index: usize,
}
#[function_component(DayTreeTaskCardAtt)]
pub fn task_card_att(props: &TaskAttProps) -> Html {
    let att = props.a.clone();
    let state = match &att.t as &str{
        "TaskType" => "",
        "Dateline" => "",
        "check" => "check",
        "Name" => "None",
        _ => {
            "default"
        }
    };
    return html!{
          <div class="task-att">
            if state == "None"{
            <></>
            }
            if state == "default"{
              <div class="box-item"><strong>{att.t}</strong>{" : "}{att.v}</div>
            }
        </div>
    }
}