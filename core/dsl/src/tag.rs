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

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_ok(content: &str, expect: &str) {
        let result = syn::parse_str::<TagDsl>(content);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().tag, expect);
    }

    fn assert_err(content: &str) {
        let result = syn::parse_str::<TagDsl>(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse() {
        assert_ok("something", "something");
        assert_ok("Something,", "Something");
        assert_ok("someThing ;", "someThing");
        assert_ok("\"some thing\"", "`some thing`");

        assert_err("123");
        assert_err("{}");
    }
}
