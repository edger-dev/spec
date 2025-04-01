use syn::parse::ParseStream;
use syn::{Result, Error, Ident, LitStr, Token};

pub fn peek_str(input: ParseStream) -> bool {
    input.peek(Ident) || input.peek(LitStr)
}

pub fn peek2_str(input: ParseStream) -> bool {
    input.peek2(Ident) || input.peek2(LitStr)
}

pub fn parse_str(input: ParseStream, kind: &str) -> Result<String> {
    let name = if input.peek(Ident) {
        input.parse::<Ident>()?.to_string()
    } else if input.peek(LitStr) {
        let text = input.parse::<LitStr>()?.value();
        format!("`{text}`")
    } else {
        return Result::Err(Error::new(input.span(), format!("Invalid {kind}")));
    };
    Ok(name)
}

pub fn peek_assigner(input: ParseStream) -> bool {
    input.peek(Token![=]) || input.peek(Token![:])
}

pub fn peek2_assigner(input: ParseStream) -> bool {
    input.peek2(Token![=]) || input.peek2(Token![:])
}

pub fn try_parse_assigner(input: ParseStream) -> Result<bool>{
    let mut found = true;
    if input.peek(Token![=]) {
        input.parse::<Token![=]>()?;
    } else if input.peek(Token![:]) {
        input.parse::<Token![:]>()?;
    } else {
        found = false;
    }
    Ok(found)
}

pub fn peek_separator(input: ParseStream) -> bool {
    input.peek(Token![,]) || input.peek(Token![;])
}

pub fn peek2_separator(input: ParseStream) -> bool {
    input.peek2(Token![,]) || input.peek2(Token![;])
}

pub fn try_parse_separator(input: ParseStream) -> Result<bool>{
    let mut found = false;
    if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
    } else if input.peek(Token![;]) {
        input.parse::<Token![;]>()?;
    } else {
        found = false;
    }
    Ok(found)
}