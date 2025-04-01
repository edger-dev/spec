use syn::parse::{Parse, ParseStream};
use syn::Result;

use edger_spec_schema::Alias;

use crate::NameDsl;
use crate::KindDsl;

#[derive(Clone, Debug)]
pub struct AliasDsl {
    pub name: NameDsl,
    pub kind: KindDsl,
}

impl Parse for AliasDsl {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<NameDsl>()?;
        let _ = crate::util::try_parse_assigner(input)?;
        let kind = input.parse::<KindDsl>()?;
        let _ = crate::util::try_parse_separator(input)?;
        Ok(AliasDsl { name, kind })
    }
}

impl AliasDsl {
    pub fn to_schema(&self) -> Alias {
        Alias {
            name: self.name.to_schema(),
            kind: self.kind.to_schema(),
        }
    }
}

