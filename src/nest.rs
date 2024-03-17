use serde::{Serialize, Deserialize};
use crate::leaf::Leaf;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Nest {
    Leaf(Leaf),
    V1(Vec<Leaf>),
    V2(Vec<Vec<Leaf>>),
    V3(Vec<Vec<Vec<Leaf>>>),
    V4(Vec<Vec<Vec<Vec<Leaf>>>>),
}
