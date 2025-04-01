use syn::parse::{Parse, ParseStream};
use syn::Result;

use edger_spec_schema::Tag;

#[derive(Clone, Debug)]
pub struct TagDsl {
    pub tag: Tag,
}

impl Parse for TagDsl {
    fn parse(input: ParseStream) -> Result<Self> {
        let tag = crate::util::parse_str(input, "Tag")?;
        let _ = crate::util::try_parse_separator(input)?;
        Ok(TagDsl{tag})
    }
}

impl TagDsl {
    pub fn to_schema(&self) -> Tag {
        self.tag.clone()
    }
}