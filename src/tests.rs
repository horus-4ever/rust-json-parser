#[cfg(test)]
mod lexer_tests {
    use crate::token::*;
    use crate::lexer::*;

    #[test]
    fn lexer_simple_ok1()
    {
        let code = r#"
            {
                "name": "yes"
            }
        "#;
        assert_eq!(Lexer::new(code).tokenize(), Some(vec![
            Token{ kind: TokenKind::Syntax(SyntaxKind::LBracket) },
            Token{ kind: TokenKind::Type(TypeKind::Str("name".to_owned())) },
            Token{ kind: TokenKind::Syntax(SyntaxKind::Colon) },
            Token{ kind: TokenKind::Type(TypeKind::Str("yes".to_owned())) },
            Token{ kind: TokenKind::Syntax(SyntaxKind::RBracket) }
        ]));
    }

    #[test]
    fn lexer_simple_ok2()
    {
        let code = r#"
            {
                "name":
                [
                    20,
                    true
                ]
            }
        "#;
        assert_eq!(Lexer::new(code).tokenize(), Some(vec![
            Token{ kind: TokenKind::Syntax(SyntaxKind::LBracket) },
            Token{ kind: TokenKind::Type(TypeKind::Str("name".to_owned())) },
            Token{ kind: TokenKind::Syntax(SyntaxKind::Colon) },
            Token{ kind: TokenKind::Syntax(SyntaxKind::LSBracket) },
            Token{ kind: TokenKind::Type(TypeKind::Numeric(20)) },
            Token{ kind: TokenKind::Syntax(SyntaxKind::Comma) },
            Token{ kind: TokenKind::Type(TypeKind::Boolean(true)) },
            Token{ kind: TokenKind::Syntax(SyntaxKind::RSBracket) },
            Token{ kind: TokenKind::Syntax(SyntaxKind::RBracket) }
        ]));
    }

    #[test]
    fn lexer_simple_fail1() {
        let code = r#"
            {
                "name": a
            }
        "#;
        assert_eq!(Lexer::new(code).tokenize(), None);
    }

    #[test]
    fn lexer_simple_fail2() {
        let code = r#"
            {
                "name": "hello\y"
            }
        "#;
        assert_eq!(Lexer::new(code).tokenize(), None);
    }
}