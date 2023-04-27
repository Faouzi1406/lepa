#[cfg(test)]
pub mod test_parser {
    use crate::{
        ast::{AstVar, TypeVar, Type},
        errors::logger::{Log, LogLevels, Logger},
        lexer::lexer::{KeyWords, Lexer, Operators, Token, TokenType},
        parser::{Parse, Parser, WalkParser},
    };

    #[test]
    fn test_peak_nth_tokens() {
        let input = " if ==  else ".to_string();
        let tokens = Token::lex(input);
        let mut parse = Parser::new(tokens);
        let tokens = parse.peak_nth(2);

        assert_eq!(
            tokens,
            Some(Token {
                token_type: TokenType::Keyword(KeyWords::Else),
                value: "else".into(),
                line: 0
            })
        )
    }

    #[test]
    fn test_peak_amount_nth_all() {
        let input = " if ==  else ".to_string();
        let tokens = Token::lex(input);
        let mut parse = Parser::new(tokens);
        let tokens = parse.peak_nth_all(2);

        assert_eq!(
            tokens,
            Some(vec![
                Token {
                    token_type: TokenType::Keyword(KeyWords::If),
                    value: "if".into(),
                    line: 0
                },
                Token {
                    token_type: TokenType::Operator(Operators::EqEq),
                    value: "==".into(),
                    line: 0
                },
            ])
        )
    }

    #[test]
    fn test_up_until_token() {
        let input = " if ==  else ".to_string();
        let tokens = Token::lex(input);
        let mut parse = Parser::new(tokens);
        let parse_until =
            parse.up_until_token(crate::lexer::lexer::TokenType::Operator(Operators::EqEq));

        // Check the amount of tokens
        assert_eq!(parse_until.clone().unwrap().len(), 2);
        // Check if tokens are correct
        assert_eq!(
            parse_until.clone().unwrap(),
            vec![
                Token {
                    token_type: TokenType::Keyword(KeyWords::If),
                    value: "if".into(),
                    line: 0
                },
                Token {
                    token_type: TokenType::Operator(Operators::EqEq),
                    value: "==".into(),
                    line: 0
                }
            ]
        );
    }

    #[test]
    // Tests for parsing variables look at file: sample_code/testing/var.lp for the code being
    // tested
    fn parsing_vars() {
        let lexer = Token::lex(include_str!("../../sample_code/testing/var.lp").to_string());
        let parse = Parser::new(lexer).parse();

        let var_1 = &parse.as_ref().unwrap().body[0];
        assert_eq!(var_1.var_name(), Some("some".into()));
        assert_eq!(
            var_1.var_value(),
            Some(TypeVar::String("Wow this works!".into()))
        );

        let var_2 = &parse.as_ref().unwrap().body[1];
        assert_eq!(var_2.var_name(), Some("number".into()));
        assert_eq!(var_2.var_value(), Some(TypeVar::Number(20)));
    }

    #[test]
    // Tests for parsing blocks look at file: sample_code/testing/parse_blocks.lp for the code being
    // tested
    fn parsing_blocks() {
        let lexer = Token::lex(include_str!("../../sample_code/testing/parse_blocks.lp").to_string());
        let parse = Parser::new(lexer).parse();

        let block = &parse.as_ref().unwrap().body[0];
        assert_eq!(block.type_, Type::Block);

        let var_in_block = &block.body[0];
        assert_eq!(var_in_block.var_name(), Some("hello".into()));

        let block_in_block = &block.body[1];
        assert_eq!(block_in_block.type_, Type::Block);
        
        let var_in_block_in_block = &block_in_block.body[0];
        assert_eq!(var_in_block_in_block.var_name(), Some("magic_recursion".into()));

        let outer_var = &parse.as_ref().unwrap().body[1];
        assert_eq!(outer_var.var_name(), Some("whut".into()));
    }
}
