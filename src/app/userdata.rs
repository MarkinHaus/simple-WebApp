use std::fmt;
use bounce::*;
use serde_json::json;
use yew_hooks::{use_local_storage, UseLocalStorageHandle};
use crate::app::quick_note::{Note, Types};
use serde::{Deserialize, Serialize};
use crate::app::daytree::Task;

#[derive(PartialEq, Atom, Eq, Clone)]
pub struct UserData {
    pub token: String,
    pub auth: bool,
}
impl From<String> for UserData {
    fn from(token: String) -> Self {
        Self {token, auth: false }
    }
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            token: "".to_string(),
            auth: false
        }
    }
}

impl fmt::Display for UserData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "auth: {}", self.auth)
    }
}

impl UserData {
    pub fn init_withe_token(s: &str) -> Self {
        Self {
            token: s.to_string(),
            auth: true,
        }
    }
}


#[derive(PartialEq, Atom, Eq, Clone, Deserialize, Serialize)]
pub struct QuickNoteData {
    pub notes: Vec<Note>,
    pub types: Vec<Types>,
}

impl From<String> for QuickNoteData {
    fn from(data: String) -> Self {
        let ob: QuickNoteData = serde_json::from_str(&data.as_str()).unwrap();
        ob
    }
}

impl Default for QuickNoteData {
    fn default() -> Self {
        Self {
        notes:vec![],
        types: vec![],
        }
    }
}

impl fmt::Display for QuickNoteData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = json!(self);
        write!(f, " {}", out)
    }
}


#[derive(PartialEq, Atom, Eq, Clone, Deserialize, Serialize)]
pub struct DayTreeData {
    pub notes: Vec<Note>,
    pub types: Vec<Types>,
    pub tasks: Vec<Task>,
}

impl From<String> for DayTreeData {
    fn from(data: String) -> Self {
        let ob: DayTreeData = serde_json::from_str(&data.as_str()).unwrap();
        ob
    }
}

impl Default for DayTreeData {
    fn default() -> Self {
        Self {
            notes:vec![],
            types: vec![],
            tasks: vec![],
        }
    }
}

impl fmt::Display for DayTreeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = json!(self);
        write!(f, " {}", out)
    }
}