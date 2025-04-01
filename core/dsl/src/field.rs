use syn::parse::{Parse, ParseStream};
use syn::Result;

use edger_spec_schema::Field;

use crate::NameDsl;
use crate::KindDsl;

#[derive(Clone, Debug)]
pub struct FieldDsl {
    pub name: NameDsl,
    pub kind: KindDsl,
}

impl Parse for FieldDsl {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<NameDsl>()?;
        let _ = crate::util::try_parse_assigner(input)?;
        let kind = input.parse::<KindDsl>()?;
        let _ = crate::util::try_parse_separator(input)?;
        Ok(FieldDsl { name, kind })
    }
}

impl FieldDsl {
    pub fn to_schema(&self) -> Field {
        Field {
            name: self.name.to_schema(),
            kind: self.kind.to_schema(),
        }
    }
}

