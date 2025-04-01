use syn::{Result, parse_str};

use edger_spec_schema::{Alias, Record, Tuple, Union};

use crate::{AliasDsl, RecordDsl, TupleDsl, UnionDsl};

pub fn parse_alias(content: &str) -> Result<Alias> {
    let alias = parse_str::<AliasDsl>(content)?;
    //println!("Tab: T:{}, S:{}", tab.tracks.len(), tab.sections.len());
    Ok(alias.to_schema())
}

pub fn parse_tuple(content: &str) -> Result<Tuple> {
    let tuple = parse_str::<TupleDsl>(content)?;
    //println!("Tab: T:{}, S:{}", tab.tracks.len(), tab.sections.len());
    Ok(tuple.to_schema())
}

pub fn parse_record(content: &str) -> Result<Record> {
    let record = parse_str::<RecordDsl>(content)?;
    //println!("Tab: T:{}, S:{}", tab.tracks.len(), tab.sections.len());
    Ok(record.to_schema())
}

pub fn parse_union(content: &str) -> Result<Union> {
    let union = parse_str::<UnionDsl>(content)?;
    //println!("Tab: T:{}, S:{}", tab.tracks.len(), tab.sections.len());
    Ok(union.to_schema())
}

