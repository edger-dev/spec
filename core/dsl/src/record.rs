use syn::parse::{Parse, ParseStream};
use syn::{braced, Result};

use edger_spec_schema::Record;

use crate::NameDsl;
use crate::FieldDsl;

#[derive(Clone, Debug)]
pub struct RecordDsl {
    pub name: NameDsl,
    pub fields: Vec<FieldDsl>,
}

impl Parse for RecordDsl {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<NameDsl>()?;
        let _ = crate::util::try_parse_assigner(input)?;
        let fields_content;
        braced!(fields_content in input);
        let mut fields = vec![];
        while !fields_content.is_empty() {
            fields.push(fields_content.parse::<FieldDsl>()?);
        }
        let _ = crate::util::try_parse_separator(input)?;
        Ok(RecordDsl { name, fields })
    }
}

impl RecordDsl {
    pub fn to_schema(&self) -> Record {
        let fields = self.fields.iter().map(
            |x| x.to_schema()
        ).collect();

        Record {
            name: self.name.to_schema(),
            fields,
        }
    }
}


