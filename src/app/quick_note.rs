use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;
use bounce::*;
use gloo::console::log;
use serde_json::json;
use stylist::Style;

use yew::{Callback, Html, html, use_state, function_component, classes, Properties};
use yew_hooks::{use_async, use_effect_once, use_local_storage, UseAsyncHandle, UseLocalStorageHandle, UseSetHandle};
use yew_router::history::History;
use yew_router::hooks::use_history;
use crate::app::cloundm::{ResultCon, ServiceURL};
use crate::app::router::Route;
use crate::app::userdata::{QuickNoteData, UserData};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use yew_router::prelude::Link;
use web_sys::{Event, HtmlTextAreaElement, HtmlSelectElement, HtmlInputElement};
use crate::util::text_input::{TextInputField, TextInputStagesMsg};


const STYLE: &str = include_str!("quick_note.css");

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Note {
    pub id: String,
    pub note: String,
    pub r#type: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Types {
    pub name: String,
    pub sub_type_of: String,
    pub color: String,
}

#[derive(Clone, Debug)]
pub enum _LoadingStage {
    _Success,
    _Loading,
    _Error,
}

#[derive(Clone)]
pub struct DataEdit {
    pub id: usize,
    pub data: String,
}

#[function_component(QuickNoteApp)]
pub fn quick_note() -> Html {

    let userdata = use_atom::<UserData>();

    let quick_note_data = use_atom::<QuickNoteData>();

    let tabs_n = use_state(|| true);

    let clone_quick_note_data = quick_note_data.clone();
    let clone_userdata = userdata.clone();
    let set_atom_data = Callback::from(move | d :QuickNoteData| {
        let clone_qnd = clone_quick_note_data.clone();
        clone_qnd.set(d);
    });

    let his = use_history().unwrap();
    let clone_set_atom_data = set_atom_data.clone();
    let get_asy: UseAsyncHandle<String, _> = use_async(async move {
        let clone_his = his.clone();
        let clone_set_atom_data = clone_set_atom_data.clone();
        if clone_userdata.auth {
            let mut service = ServiceURL::default();
            let data_note = service.post_request(json!({}), clone_userdata.token.clone(), "quickNote".to_string(),
                                            "get_inbox_api".to_string(), "".to_string()).await;

            let result: Result<ResultCon<Vec<Note>>, _> = serde_json::from_str(&data_note);

            let mut data = QuickNoteData{notes: vec![],types:  vec![]};
            match result {
                    Ok(res) => {
                        data.notes = res.res;
                    }
                    Err(e) => {
                        log!(e.to_string());
                        return Err(e.to_string());
                    }
                };

            let data_types = service.post_request(json!({}), clone_userdata.token.clone(), "quickNote".to_string(),
                                            "get_types_api".to_string(), "".to_string()).await;

            let result: Result<ResultCon<Vec<Types>>, _> = serde_json::from_str(&data_types);
            match result {
                    Ok(res) => {
                        data.types = res.res;
                    }
                    Err(e) => {
                        log!(e.to_string());
                        return Err(e.to_string());
                }
            };
            clone_set_atom_data.emit(data);
            return  Ok("".to_string())
        }else {
            clone_his.push(Route::CloudMLogin);
        }
        return Err("???".to_string());
    });

    let clone_data = quick_note_data.clone();
    let clone_userdata = userdata.clone();
    let his = use_history().unwrap();
    let save_asy: UseAsyncHandle<String, _> = use_async(async move {
        let clone_his = his.clone();
        if clone_userdata.auth {
            let mut service = ServiceURL::default();
            let notes = clone_data.notes.clone();
            let _data = service.post_request(json!({"notes":notes}), clone_userdata.token.clone(), "quickNote".to_string(),
                                            "save_inbox_api".to_string(), "".to_string()).await;

            let types = clone_data.types.clone();
            let _data = service.post_request(json!({"types":types}), clone_userdata.token.clone(), "quickNote".to_string(),
                                            "save_types_api".to_string(), "".to_string()).await;

            return  Ok("".to_string())
        }else {
            clone_his.push(Route::CloudMLogin);
            return Err(())
        }

    });

    let clone_tabs_n = tabs_n.clone();
    let open_notes = Callback::from(move |_| {
        clone_tabs_n.set(true);
    });

    let clone_tabs_n = tabs_n.clone();
    let open_types = Callback::from(move |_| {
        clone_tabs_n.set(false);
    });

    let mut ac_clss_n = "gon";
    let mut ac_clss_t = "";
    if tabs_n.deref().clone() {
        ac_clss_n= "";
        ac_clss_t= "gon";
    }

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

    let stylesheet = Style::new(STYLE).unwrap();
    let clone_tabs_n = tabs_n.deref().clone();

    return html! {
        <div class={stylesheet}>
            <div class="QuickNote">
                <div class="login-bg">
                    <h1>{"QuickNote"}</h1>
                </div>
                <div class="header">
                    <div class={classes!("header-nav", ac_clss_n)} onclick={open_notes.clone()}> <h3>{"Go to Notes"}</h3></div>
                    <div class={classes!("header-nav", ac_clss_t)} onclick={open_types.clone()}> <h3>{"Go to Types"}</h3></div>
                </div>
        if let Some(_data) = &get_asy.data {
            <div>
            if clone_tabs_n {
                <NotesAPP/>
            }else {
                <TypesAPP/>
            }
            </div>
        } else {
            <div><h2>{"Sorry an error accord lock up the server logs, or contact an Admin"}</h2></div>
        }
        if let Some(error) = &get_asy.error {
            <div><h2>{"Error - loading Notes data "} {error}</h2></div>
        }
                <h3> { "Go "} <Link<Route> to={Route::Home}>{"Home "}</Link<Route>> </h3>
            </div>
        </div>
    }
}

#[function_component(NotesAPP)]
pub fn quick_note_app() -> Html {

    let quick_note_data = use_atom::<QuickNoteData>();

    let data_setter =quick_note_data.clone();
    let data_value = quick_note_data.notes.clone();
    let search = use_state(|| "".to_string());

    let clone_data_setter = data_setter.clone();
    let clone_data_value = data_value.clone();
    let add_note = Callback::from(move |_| {
        let mut data = clone_data_value.clone();
        let new_note = Note { note: "".to_string(), id: data.len().to_string(), r#type: "quickNotes/Inbox".to_string() }; // TODO Better ID system
        data.insert(0, new_note);
        clone_data_setter.set(QuickNoteData{
            notes: data.clone(),
            types: clone_data_setter.types.clone()
        });
    });

    let clone_data_setter = data_setter.clone();
    let clone_data_value = data_value.clone();
    let del_note = Callback::from(move |id: usize| {
        let mut data = clone_data_value.clone();
        data.remove(id);
        clone_data_setter.set(QuickNoteData{
            notes: data.clone(),
            types: clone_data_setter.types.clone()
        });
    });

    let clone_data_setter = data_setter.clone();
    let clone_data_value = data_value.clone();
    let set_note_type = Callback::from(move |data_e_n: DataEdit| {
        let mut data = clone_data_value.clone();
        let note = data.get(data_e_n.id).unwrap().clone();
        let new_note = Note { note: note.note, id: note.id, r#type: data_e_n.data }; // TODO Better ID system
        data.remove(data_e_n.id);
        data.insert(data_e_n.id, new_note);
        clone_data_setter.set(QuickNoteData{
            notes: data.clone(),
            types: clone_data_setter.types.clone()
        });
    });

    let clone_data_setter = data_setter.clone();
    let clone_data_value = data_value.clone();
    let set_note = Callback::from(move |data_e_n: DataEdit| {
        let mut data = clone_data_value.clone();
        let note = data.get(data_e_n.id).unwrap().clone();
        let new_note = Note { note: data_e_n.data, id: note.id, r#type: note.r#type }; // TODO Better ID system
        data.remove(data_e_n.id);
        data.insert(data_e_n.id, new_note);
        clone_data_setter.set(QuickNoteData{
            notes: data.clone(),
            types: clone_data_setter.types.clone()
        });
    });

    let clone_search = search.clone();
    let onchange = Callback::from(move |s: String| {
        clone_search.set(s);
        log!("SO")
    });

    let cards_n = rap_notes_to_card(Box::new(data_value.clone()), del_note, set_note, set_note_type, data_setter.types.clone(),  search.clone().to_string().as_str());
    let val_search_ = search.to_string();
    let val_search = String::from(val_search_);
    return html! {
        <div calss={classes!("NotesAPP", "content")}>
        <div class="note.input">
                <TextInputField
                    class=""
                    id="Search-Notes"
                    name="sn"
                    type_="text"
                    value={val_search.clone()}
                    placeholder="Search-Notes"
                    label=""
                    set_value={onchange.clone()}
                        open={true}
        perma_open={true}
                    />
         </div>
            <button onclick={add_note}> {"Add New Note"} </button>
            {cards_n}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct NoteVew {
    pub note: String,
    pub type_: String,
    pub id: String,
    pub index: usize,
    pub del_note: Callback<usize>,
    pub edit_note: Callback<DataEdit>,
    pub set_type: Callback<DataEdit>,
    pub types: Vec<Types>,
}

#[function_component(NoteKard)]
pub fn note_card(props: &NoteVew) -> Html {
    let toggle_edit = use_state(|| false);

    let clone_index = props.index;
    let clone_del = props.del_note.clone();
    let del = Callback::from(move |_| {
        clone_del.emit(clone_index - 1);
    });

    let clone_toggle_edit = toggle_edit.clone();
    let edit_note = Callback::from(move |_| {
        clone_toggle_edit.set(true);
    });

    let clone_index = props.index;
    let clone_set = props.set_type.clone();
    let set_type_note = Callback::from(move |event: Event| {
        let new_type = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlSelectElement>()
            .value();
        log!("new_type:", new_type.clone());
        let data = DataEdit { id: clone_index - 1, data: new_type };
        clone_set.emit(data);
    });

    let clone_toggle_edit = toggle_edit.clone();
    let save_close_note = Callback::from(move |_| {
        clone_toggle_edit.set(false);
    });

    let onchange = {
        let clone_id = props.index;
        let clone_edit = props.edit_note.clone();
        Callback::from(move |event: Event| {
            let note = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            clone_edit.emit(DataEdit { data: note, id: clone_id - 1 });
        })
    };


    let options = rap_types_to_option(Box::new(props.types.clone()));

    html! {
        <div class={"noteCarte"}>
            if toggle_edit.deref().clone() {
            <div>
                <label for="types">{"Choose a Type:"}</label>
                <select name="types" id="types" onchange={set_type_note.clone()}>
                    {options}
                    <option value={"quickNote/inbox"}>{"quickNote/inbox"}</option>
                    <option value={props.type_.clone()} selected={true} disabled={true} hidden={true}>{props.type_.clone()}</option>
                </select>
                <textarea
                  data-test={props.note.clone()}
                  value={props.note.clone()}
                {onchange}
                />
            </div>
              <button onclick={save_close_note} >{"save"}</button>
            }else {
            <div>
              <h3>{"type :"} {&props.type_}</h3>
              <h2>{&props.note.clone()}</h2>
            </div>
              <button onclick={edit_note} >{"Edit"}</button>
            }
            <button onclick={del}>{"Del"}</button>
        </div>
    }
}

fn rap_notes_to_card(notes: Box<Vec<Note>>, del: Callback<usize>, edit: Callback<DataEdit>, set_type: Callback<DataEdit>, types: Vec<Types>, search: &str) -> Vec<Html> {
    let mut i = 0usize;
    notes.iter().map(|note| {
        i += 1;
        if search.is_empty() {
            return html! {<NoteKard type_={note.r#type.clone()} id={note.id.clone()} index={i} note={note.note.clone()} del_note={del.clone()} edit_note={edit.clone()} set_type={set_type.clone()} types={types.clone()}/>}
        }else {
            if note.note.contains(search) || note.r#type.contains(search) {
            return html! {<NoteKard type_={note.r#type.clone()} id={note.id.clone()} index={i} note={note.note.clone()} del_note={del.clone()} edit_note={edit.clone()} set_type={set_type.clone()} types={types.clone()}/>}
            }
        }
        return  html! {}
    }).collect()
}

fn rap_types_to_option(types: Box<Vec<Types>>) -> Vec<Html> {
    let mut i = 0usize;

    types.iter().map(|t| {
        i += 1;
        let mut val: String = t.sub_type_of.clone();
        val.push("/".parse().unwrap());
        val = val + &t.name.clone();
        html! {
         <option value={val.clone()}>{val}</option>
    }
    }).collect()
}


#[derive(Properties, PartialEq)]
pub struct TypeVew {
    pub name: String,
    pub sub_type_of: String,
    pub index: usize,
    pub color: String,
    pub del_type: Callback<usize>,
    pub edit_type: Callback<DataEdit>,
    pub edit_color: Callback<DataEdit>,
    pub add_sub_type:Callback<(String, String)>,
}

#[function_component(TypesAPP)]
pub fn quick_type_app() -> Html {

    let quick_note_data = use_atom::<QuickNoteData>();

    let data_setter =quick_note_data.clone();
    let data_value = quick_note_data.types.clone();
    let search = use_state(|| "".to_string());

    let clone_data_setter = data_setter.clone();
    let clone_data_value = data_value.clone();
    let add_type = Callback::from(move |_| {
        let mut data = clone_data_value.clone();
        let new_type = Types {
            name: "".to_string(),
            sub_type_of: "".to_string(),
            color: "".to_string()
        }; // TODO Better ID system
        data.insert(0, new_type);
        clone_data_setter.set(QuickNoteData{
            types: data.clone(),
            notes: clone_data_setter.notes.clone()
        });
    });

    let clone_data_setter = data_setter.clone();
    let clone_data_value = data_value.clone();
    let del_type = Callback::from(move |id: usize| {
        let mut data = clone_data_value.clone();
        data.remove(id);
        clone_data_setter.set(QuickNoteData{
            types: data.clone(),
            notes: clone_data_setter.notes.clone()
        });
    });

    let clone_data_setter = data_setter.clone();
    let clone_data_value = data_value.clone();
    let set_type = Callback::from(move |data_e_n: DataEdit| {
        let mut data = clone_data_value.clone();
        let type_ = data.get(data_e_n.id).unwrap().clone();
        let new_type = Types { name: data_e_n.data, sub_type_of:type_.sub_type_of, color: "".to_string() }; // TODO Better ID system
        data.remove(data_e_n.id);
        data.insert(data_e_n.id, new_type);
        clone_data_setter.set(QuickNoteData{
            types: data.clone(),
            notes: clone_data_setter.notes.clone()
        });
    });

    let clone_data_setter = data_setter.clone();
    let clone_data_value = data_value.clone();
    let add_color_type = Callback::from(move |data_e_n: DataEdit| {
        let mut data = clone_data_value.clone();
        let type_ = data.get(data_e_n.id).unwrap().clone();
        let new_type = Types { name: type_.name, sub_type_of: type_.sub_type_of, color: data_e_n.data}; // TODO Better ID system
        data.remove(data_e_n.id);
        data.insert(data_e_n.id, new_type);
        clone_data_setter.set(QuickNoteData{
            types: data.clone(),
            notes: clone_data_setter.notes.clone()
        });
    });

    let clone_data_setter = data_setter.clone();
    let clone_data_value = data_value.clone();
    let add_sub_type = Callback::from(move |b:(String,String)| {
        let mut data = clone_data_value.clone();
        let s= b.0;
        let c = b.1;
        let new_type = Types { name: "#Edit#".to_string(), sub_type_of: s, color:c}; // TODO Better ID system
        data.insert(0, new_type);
        clone_data_setter.set(QuickNoteData{
            types: data.clone(),
            notes: clone_data_setter.notes.clone()
        });
    });

    let clone_search = search.clone();
    let onchange = Callback::from(move |s: String| {
        clone_search.set(s);
        log!("SO")
    });

    let clone_state = data_value.clone();

    let val_search_ = search.to_string();
    let cards_t = rap_types_to_card(Box::new(clone_state.clone()), del_type, set_type, add_color_type, add_sub_type);
    let val_search = String::from(val_search_);
    return html! {
        <div calss={classes!("NotesAPP", "content")}>
        <div class="note.input">
                        <TextInputField
                    class=""
                    id="Search-Types"
                    name="st"
                    type_="text"
                    value={val_search.clone()}
                    placeholder="Search-Types"
                    label=""
                    set_value={onchange.clone()}
                        open={true}
        perma_open={true}
                    />
         </div>
            <button onclick={add_type}> {"Add New Type"} </button>
            {cards_t}
        </div>
    }

}

#[function_component(TypeKard)]
pub fn type_card(props: &TypeVew) -> Html {
    let toggle_edit = use_state(|| false);

    let clone_index = props.index;
    let clone_del = props.del_type.clone();
    let del = Callback::from(move |_| {
        clone_del.emit(clone_index - 1);
    });

    let clone_toggle_edit = toggle_edit.clone();
    let edit_note = Callback::from(move |_| {
        clone_toggle_edit.set(true);
    });

    let clone_toggle_edit = toggle_edit.clone();
    let save_close_note = Callback::from(move |_| {
        clone_toggle_edit.set(false);
    });

    let mut glue_type = props.sub_type_of.clone();
    if !props.sub_type_of.is_empty() {
        glue_type.push_str("/");
    }
    glue_type += &props.name.clone();
    let color  = props.color.clone();
    let clone_add_set_sub = props.add_sub_type.clone();
    let clone_set_sub = Callback::from(move |_| {
        let glue_type = glue_type.clone();
        let color = color.clone();
        clone_add_set_sub.emit((glue_type, color));
    });


    let onchange = {
        let clone_id = props.index;
        let clone_edit = props.edit_type.clone();
        Callback::from(move |event: Event| {
            let note = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            clone_edit.emit(DataEdit { data: note, id: clone_id - 1 });
        })
    };

    html! {
        <div class={"noteCarte"}>
            if toggle_edit.deref().clone() {
                      <textarea
                        data-test={props.name.clone()}
                        value={props.name.clone()}
                      {onchange}
                      />
              <input type="color" />
              <button onclick={save_close_note} >{"save"}</button>
            }else {
                <div class={"header"} data-color={props.color.clone()}>
                    <h1>{&props.sub_type_of}{"/"}{&props.name}</h1>
                </div>
                <button onclick={edit_note} >{"Edit"}</button>
            }
            <button onclick={clone_set_sub}>{"Add sub Type"}</button>
            <button onclick={del}>{"Del"}</button>
        </div>
    }
}

fn rap_types_to_card(types: Box<Vec<Types>>, del: Callback<usize>, edit: Callback<DataEdit>, add_color: Callback<DataEdit>, add_sub_type: Callback<(String, String)>) -> Vec<Html> {
    let mut i = 0usize;
    types.iter().map(|t| {
        i += 1;
        html! {
            <TypeKard
            name={t.name.clone()}
            color={t.color.clone()}
            index={i}
            sub_type_of={t.sub_type_of.clone()}
            del_type={del.clone()}
            edit_type={edit.clone()}
            edit_color={add_color.clone()}
            add_sub_type={add_sub_type.clone()}
            />
        }
    }).collect()
}
