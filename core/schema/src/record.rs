use serde::{Deserialize, Serialize};

use crate::{Alias, Name};

pub type Field = Alias;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub struct Record {
    pub name: Name,
    pub fields: Vec<Field>,
}

impl crate::DebugDisplay for Record {}