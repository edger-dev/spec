use serde::{Deserialize, Serialize};

use crate::{Alias, Tuple, Record};

pub type Tag = String;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub enum Case {
   Tag(Tag),
   Alias(Alias),
   Tuple(Tuple),
   Record(Record),
}

impl crate::DebugDisplay for Case {}