use serde::{Deserialize, Serialize};

use super::alias::{Alias, Name};

pub type Field = Alias;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub struct Record {
    pub name: Name,
    pub fields: Vec<Field>,
}

impl super::DebugDisplay for Record {}