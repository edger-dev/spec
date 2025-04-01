use serde::{Deserialize, Serialize};

use crate::{Name, Kind};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub struct Tuple {
    pub name: Name,
    pub fields: Vec<Kind>,
}

impl crate::util::DebugDisplay for Tuple {}