use serde::{Deserialize, Serialize};

use crate::{alias::{Alias, Name}, tuple::Tuple};

use super::record::Record;

pub type Tag = String;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub enum Case {
   Tag(Tag),
   Alias(Alias),
   Tuple(Tuple),
   Record(Record),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub struct Union {
    pub name: Name,
    pub cases: Vec<Case>,
}

impl super::DebugDisplay for Union {}