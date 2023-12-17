use crate::{Expression, Scalor, Variable, new_slack};


pub enum Condition {
    Equation(Equation),
    Inequality(Inequality),
} impl Condition {
    pub fn into_standarized(self) -> (Self, Option<Variable>) {
        match self {
            Self::Equation(e) => (
                Self::Equation(e), None
            ),
            Self::Inequality(Inequality { left, sign:Sign::GE, right }) => (
                Self::Inequality(Inequality { left, sign:Sign::GE, right }), None
            ),
            Self::Inequality(Inequality { left, sign:Sign::LE, right }) => {
                let mut terms = left.terms;
                let slack = new_slack();
                terms.push((1., slack.clone()));

                (Self::Equation(equation(terms, right)), Some(slack))
            }
        }
    }

    pub fn left(&self) -> &Expression {
        match self {
            Self::Equation(e)   => &e.left,
            Self::Inequality(i) => &i.left,
        }
    }
    pub fn right(&self) -> &Scalor {
        match self {
            Self::Equation(e)   => &e.right,
            Self::Inequality(i) => &i.right,
        }
    }
}

pub struct Equation {
    pub left:  Expression,
    pub right: Scalor,
}

pub struct Inequality {
    pub left:  Expression,
    pub sign:  Sign,
    pub right: Scalor,
}
pub enum Sign {
    GE,
    LE,
}


pub fn equation(left: Vec<(Scalor, Variable)>, right: Scalor) -> Equation {
    Equation { left: Expression::from_iter(left), right:right.into() }
}
pub fn inequality(left: Vec<(Scalor, Variable)>, sign: Sign, right: Scalor) -> Inequality {
    Inequality { left:Expression::from_iter(left), sign, right:right.into() }
}
