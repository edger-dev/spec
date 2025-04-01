pub mod util;
pub mod parser;

mod name;
mod kind;
mod alias;
mod field;
mod record;
mod tuple;
mod tag;
mod case;
mod tagged_union;


pub use crate::name::NameDsl;
pub use crate::kind::KindDsl;
pub use crate::alias::AliasDsl;
pub use crate::field::FieldDsl;
pub use crate::record::RecordDsl;
pub use crate::tuple::TupleDsl;
pub use crate::tag::TagDsl;
pub use crate::case::CaseDsl;
pub use crate::tagged_union::UnionDsl;