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

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_ok(content: &str, name: &str, kind: &str) {
        let result = syn::parse_str::<AliasDsl>(content);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.name.name, name);
        assert_eq!(result.kind.kind, kind);
    }

    fn assert_err(content: &str) {
        let result = syn::parse_str::<AliasDsl>(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse() {
        assert_ok("something somekind", "something", "somekind");

        assert_ok("Something : somekind", "Something", "somekind");
        assert_ok("Something = somekind", "Something", "somekind");

        assert_ok("Something:somekind", "Something", "somekind");
        assert_ok("Something: somekind", "Something", "somekind");
        assert_ok("Something :somekind", "Something", "somekind");
        assert_ok("Something=somekind", "Something", "somekind");
        assert_ok("Something= somekind", "Something", "somekind");
        assert_ok("Something =somekind", "Something", "somekind");

        assert_err("123");
        assert_err("{}");
    }
}

