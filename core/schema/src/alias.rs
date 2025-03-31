use serde::{Deserialize, Serialize};

pub type Name = String;
pub type Kind = String;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub struct Alias {
    pub name: Name,
    pub kind: Kind,
}

impl super::DebugDisplay for Alias {}