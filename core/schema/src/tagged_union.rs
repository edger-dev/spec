use serde::{Deserialize, Serialize};

use crate::{Name, Case};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub struct Union {
    pub name: Name,
    pub cases: Vec<Case>,
}

impl crate::DebugDisplay for Union {}