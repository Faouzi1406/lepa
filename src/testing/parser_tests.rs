#[cfg(test)]
pub mod test_parser {
    use crate::{
        lexer::lexer::{KeyWords, Lexer, Operators, Token, TokenType},
        parser::{Parser, WalkParser},
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
}
