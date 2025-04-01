use syn::parse::{Parse, ParseStream};
use syn::{Result, parenthesized};

use edger_spec_schema::Tuple;

use crate::NameDsl;
use crate::KindDsl;

#[derive(Clone, Debug)]
pub struct TupleDsl {
    pub name: NameDsl,
    pub fields: Vec<KindDsl>,
}

impl Parse for TupleDsl {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<NameDsl>()?;
        let _ = crate::util::try_parse_assigner(input)?;
        let fields_content;
        parenthesized!(fields_content in input);
        let mut fields = vec![];
        while !fields_content.is_empty() {
            fields.push(fields_content.parse::<KindDsl>()?);
        }
        let _ = crate::util::try_parse_separator(input)?;
        Ok(TupleDsl { name, fields })
    }
}

impl TupleDsl {
    pub fn to_schema(&self) -> Tuple {
        let fields = self.fields.iter().map(
            |x| x.to_schema()
        ).collect();

        Tuple {
            name: self.name.to_schema(),
            fields,
        }
    }
}


