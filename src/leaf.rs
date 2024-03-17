use serde::{Serialize, Deserialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Leaf {
    Bool(bool),
    String(String),
    Number(serde_json::Number),
}

impl Display for Leaf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Leaf::Bool(x) => {
                write!(f, "{}", x)
            },
            Leaf::Number(x) => {
                write!(f, "{}", x)
            },
            Leaf::String(x) => {
                write!(f, "{}", x)
            },
        }
    }
}
