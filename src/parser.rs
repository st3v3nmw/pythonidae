use insta::assert_debug_snapshot;

lalrpop_mod!(pub grammar);

#[test]
fn raw_expressions() {
    assert_debug_snapshot!(grammar::ProgramParser::new().parse("42"));
    assert_debug_snapshot!(grammar::ProgramParser::new().parse("(42)"));
    assert_debug_snapshot!(grammar::ProgramParser::new().parse("((((42))))"));
    assert_debug_snapshot!(grammar::ProgramParser::new().parse("((42)"));
    assert_debug_snapshot!(grammar::ProgramParser::new().parse("2 * 4 - 3"));
    assert_debug_snapshot!(grammar::ProgramParser::new().parse("(2 * 4) - 3"));
    assert_debug_snapshot!(grammar::ProgramParser::new().parse("(2-4) /3"));
}

#[test]
fn assignments() {
    assert_debug_snapshot!(grammar::ProgramParser::new().parse(
        r#"
            foo=2 * 4 - 3
            bar = (2-4) /3
        "#
    ));
}
