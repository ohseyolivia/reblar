use crate::lex::{self};

#[test]
pub fn lex_test() {
    let mode = lex::Mode::loose();
    let act_res = lex::Token::parse("hello14641\"hey friend\"", mode);
    let exp_res =  vec![
        lex::Token::Identifior("hello14641".to_string()),
        lex::Token::StrLiteral("hey friend".to_string())
    ];
    assert_eq!(Some(exp_res), act_res);
}
