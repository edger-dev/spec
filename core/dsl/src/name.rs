use syn::parse::{Parse, ParseStream};
use syn::Result;

use edger_spec_schema::Name;

#[derive(Clone, Debug)]
pub struct NameDsl {
    pub name: Name,
}

impl Parse for NameDsl {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = crate::util::parse_str(input, "Name")?;
        let _ = crate::util::try_parse_separator(input)?;
        Ok(NameDsl{name})
    }
}

impl NameDsl {
    pub fn to_schema(&self) -> Name {
        self.name.clone()
    }
}
