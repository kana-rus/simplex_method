use std::{hash::{DefaultHasher, Hasher}, sync::{OnceLock, Mutex}};


#[derive(Clone)]
pub enum Variable {
    Normal { id: usize },
    Slack  { id: usize },
    Object,
}

pub fn var(literal: impl AsRef<str>) -> Variable {
    let mut hash = DefaultHasher::new();
    hash.write(literal.as_ref().as_bytes());
    // Needs no check because this library requires target pointer width be 64
    let id = hash.finish() as usize;

    Variable::Normal { id }
}

pub fn new_slack() -> Variable {
    static SLACK_ID: OnceLock<Mutex<usize>> = OnceLock::new();

    let mut latest_id = SLACK_ID.get_or_init(|| Mutex::new(0)).lock().unwrap();
    let new_slack_id = *latest_id + 1;
    *latest_id = new_slack_id;

    Variable::Slack { id: new_slack_id }
}
