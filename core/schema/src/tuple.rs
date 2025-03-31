use serde::{Deserialize, Serialize};

use super::alias::{Name, Kind};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub struct Tuple {
    pub name: Name,
    pub fields: Vec<Kind>,
}

impl super::DebugDisplay for Tuple {}