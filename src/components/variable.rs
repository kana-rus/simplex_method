#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Variable {
    Normal {name: String},
    Slack  {id:   usize},
} impl Variable {
    pub fn is_normal(&self) -> bool {
        matches!(self, Variable::Normal {..})
    }
    pub fn is_slack(&self) -> bool {
        matches!(self, Variable::Slack {..})
    }
}

pub fn var(name: &'static str) -> Variable {
    Variable::Normal {
        name: name.to_string(),
    }
}

#[cfg(test)] pub(crate) fn slack(id: usize) -> Variable {
    Variable::Slack { id }
}

const _: () = {
    impl std::fmt::Debug for Variable {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Normal { name } => f.write_str(name),
                Self::Slack  {  id  } => f.write_str(&format!("slack#{id}")),
            }
        }
    }
};
