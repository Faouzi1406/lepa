pub mod ast;
pub mod compiler;
pub mod cursor;
pub mod errors;
pub mod lexer;
pub mod parser;

// Test the lexer with the given input
#[cfg(test)]
pub mod test_lexer {
    #[cfg(test)]
    pub mod test_lexer_operators {
        use crate::lexer::lexer::{Lexer, Operators, Token, TokenType};

        #[test]
        fn test_eq_operators() {
            let input = " = == ".to_string();
            let lex = Token::lex(input);

            // Verify that second first token is an Eq
            assert_eq!(lex[0].token_type, TokenType::Operator(Operators::Eq));
            // Verify that the 3 token is an EqEq
            assert_eq!(lex[1].token_type, TokenType::Operator(Operators::EqEq));
        }

        #[test]
        fn test_less() {
            let input = " <  <= ".to_string();
            let lex = Token::lex(input);

            // Verify that the first token is an Less
            assert_eq!(lex[0].token_type, TokenType::Operator(Operators::Less));
            // Verify that trhe second token is an LessEq
            assert_eq!(lex[1].token_type, TokenType::Operator(Operators::LessEq));
        }

        #[test]
        fn test_more() {
            let input = " >  >= ".to_string();
            let lex = Token::lex(input);

            // Verify thath the first token is an More
            assert_eq!(lex[0].token_type, TokenType::Operator(Operators::More));
            // Verify that the second token is an MoreEq
            assert_eq!(lex[1].token_type, TokenType::Operator(Operators::MoreEq));
        }
    }

    #[cfg(test)]
    pub mod test_strings {
        use crate::lexer::lexer::{Lexer, Token, TokenType};

        #[test]
        fn test_string() {
            let input = "\"Hello World\"".to_string();
            let lex = Token::lex(input);

            // Verify that the first token is a string
            assert_eq!(lex[0].token_type, TokenType::String,);
            // Verify that the value of the string is correct
            assert_eq!(lex[0].value, "Hello World")
        }
    }

    #[cfg(test)]
    pub mod test_numbers {
        use crate::lexer::lexer::{Lexer, Token, TokenType};

        #[test]
        fn test_numn() {
            let input = " 10 ".to_string();
            let lex = Token::lex(input);

            // Verify that the first token is a number
            assert_eq!(lex[0].token_type, TokenType::Number);
            // Verify that the value of the number is correct
            assert_eq!(lex[0].value.parse::<f64>().unwrap(), 10.);
        }
    }

    #[cfg(test)]
    pub mod test_identifiers {
        use crate::lexer::lexer::{Lexer, Token, TokenType};

        #[test]
        fn test_identifier() {
            let input = " hello world ".to_string();
            let lex = Token::lex(input);

            // Verify that the first token is an identifier
            assert_eq!(lex[0].token_type, TokenType::Identifier);
            // Verify that the value of the identifier is correct
            assert_eq!(lex[0].value, "hello");

            // Verify that the second token is an identifier
            assert_eq!(lex[1].token_type, TokenType::Identifier);
            // Verify that the value of the identifier is correct
            assert_eq!(lex[1].value, "world");
        }
    }

    #[cfg(test)]
    pub mod test_keywords {
        use crate::lexer::lexer::{KeyWords, Lexer, Token, TokenType};

        #[test]
        fn test_if_else() {
            let input = " if  else ".to_string();
            let lex = Token::lex(input);
            assert_eq!(lex[0].token_type, TokenType::Keyword(KeyWords::If));
            assert_eq!(lex[1].token_type, TokenType::Keyword(KeyWords::Else));
        }

        #[test]
        fn test_let() {
            let input = " let ";
            let lex = Token::lex(input.into());
            assert_eq!(lex[0].token_type, TokenType::Keyword(KeyWords::Let));
        }
    }
}

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
