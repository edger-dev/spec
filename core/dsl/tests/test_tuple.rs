use edger_spec_dsl::parser;
use edger_spec_schema::Tuple;


fn assert_ok(content: &str, expected: &Tuple) {
    let result = parser::parse_tuple(content);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected.clone());
}

#[test]
fn test_tuple() {
    let expected = Tuple {
        name: "Name".to_string(),
        fields: vec![
            "Int".to_string(),
            "String".to_string(),
        ] 
    };
    assert_ok("Name = (Int, String)", &expected);
    assert_ok("Name = (Int, String, )", &expected);
    assert_ok("Name: (Int; String; )", &expected);
}
