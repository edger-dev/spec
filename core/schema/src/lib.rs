use std::fmt::{Debug, Display};

pub mod alias;
pub mod record;
pub mod tuple;
pub mod tagged_union;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::alias::{Alias, Name, Kind};

    #[doc(hidden)]
    pub use crate::record::{Record, Field};

    #[doc(hidden)]
    pub use crate::tuple::Tuple;

    #[doc(hidden)]
    pub use crate::tagged_union::{Union, Tag};
}

pub trait DebugDisplay : Debug {
}

impl Display for dyn DebugDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}