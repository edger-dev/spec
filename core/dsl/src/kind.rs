use syn::parse::{Parse, ParseStream};
use syn::Result;

use edger_spec_schema::Kind;

#[derive(Clone, Debug)]
pub struct KindDsl {
    pub kind: Kind,
}

impl Parse for KindDsl {
    fn parse(input: ParseStream) -> Result<Self> {
        let kind = crate::util::parse_str(input, "Kind")?;
        let _ = crate::util::try_parse_separator(input)?;
        Ok(KindDsl{kind})
    }
}


impl KindDsl {
    pub fn to_schema(&self) -> Kind {
        self.kind.clone()
    }
}