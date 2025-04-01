use syn::parse::{End, Parse, ParseStream};
use syn::{Result, Error, token, Token};

use edger_spec_schema::Case;

use crate::AliasDsl;
use crate::TupleDsl;
use crate::TagDsl;
use crate::RecordDsl;

#[derive(Clone, Debug)]
pub enum CaseDsl {
    Tag(TagDsl),
    Alias(AliasDsl),
    Tuple(TupleDsl),
    Record(RecordDsl),
}

impl Parse for CaseDsl {
    fn parse(input: ParseStream) -> Result<Self> {
        let case = if
                crate::util::peek2_separator(input)
                || input.peek2(End) {
            CaseDsl::Tag(input.parse::<TagDsl>()?)
        } else if crate::util::peek2_str(input) {
            CaseDsl::Alias(input.parse::<AliasDsl>()?)
        } else if input.peek2(token::Paren) {
            CaseDsl::Tuple(input.parse::<TupleDsl>()?)
        } else if input.peek2(token::Brace) {
            CaseDsl::Record(input.parse::<RecordDsl>()?)
        } else {
            return Result::Err(Error::new(input.span(), format!("Invalid Case")));
        };

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        Ok(case)
    }
}

impl CaseDsl {
    pub fn to_schema(&self) -> Case {
        match self {
            CaseDsl::Tag(tag) =>
                Case::Tag(tag.to_schema()),
            CaseDsl::Alias(alias) =>
                Case::Alias(alias.to_schema()),
            CaseDsl::Tuple(tuple) =>
                Case::Tuple(tuple.to_schema()),
            CaseDsl::Record(record) =>
                Case::Record(record.to_schema()),

        }
    }
}


