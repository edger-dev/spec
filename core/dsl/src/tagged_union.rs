use syn::parse::{Parse, ParseStream};
use syn::{bracketed, Result};

use edger_spec_schema::Union;

use crate::NameDsl;
use crate::CaseDsl;

#[derive(Clone, Debug)]
pub struct UnionDsl {
    pub name: NameDsl,
    pub cases: Vec<CaseDsl>,
}

impl Parse for UnionDsl {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<NameDsl>()?;
        let _ = crate::util::try_parse_assigner(input)?;
        let cases_content;
        bracketed!(cases_content in input);
        let mut cases = vec![];
        while !cases_content.is_empty() {
            cases.push(cases_content.parse::<CaseDsl>()?);
        }
        let _ = crate::util::try_parse_separator(input)?;
        Ok(UnionDsl { name, cases })
    }
}

impl UnionDsl {
    pub fn to_schema(&self) -> Union {
        let cases = self.cases.iter().map(
            |x| x.to_schema()
        ).collect();

        Union {
            name: self.name.to_schema(),
            cases,
        }
    }
}


