#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Variable {
    Normal {name: String},
    Slack  {name: String},
}

pub fn var(name: &'static str) -> Variable {
    Variable::Normal {
        name: name.to_string(),
    }
}


