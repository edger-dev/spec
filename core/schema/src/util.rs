use std::fmt::{Debug, Display};

pub trait DebugDisplay : Debug {
}

impl Display for dyn DebugDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}