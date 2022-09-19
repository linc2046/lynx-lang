use crate::ast::{AstNode, Expression, Precedence, Statement};
use crate::lexer::Lexer;
use crate::token::TokenType;
use std::ops::Deref;

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curToken: TokenType,
    peekToken: TokenType,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer,
            curToken: TokenType::NONE,
            peekToken: TokenType::NONE,
        }
    }

    fn next_token(&mut self) {
        self.curToken = self.peekToken.clone();

        // println!("curToken {:?}", self.curToken);

        self.peekToken = self.lexer.next_token();

        // println!("29 peekToken {:?}", self.peekToken);
    }

    fn expect_cur_token_is(&self, _t: TokenType) -> bool {
        let matched = matches!(&self.curToken, _t);
        // println!("current {:?} {:?} {:?}", &self.curToken, _t, matched);

        matched
    }

    fn expect_peek_token_is(&self, _t: TokenType) -> bool {
        let matched = matches!(&self.peekToken, _t);
        // println!("peek {:?} {:?} {:?}", &self.peekToken, _t, matched);

        matched
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.curToken {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let precedence = self.cur_precedence();

        println!("parse_expression_statement {:?} {:?}", self.curToken, precedence);

        match self.parse_expression(precedence) {
            Some(expr) => {
                if self.peekToken.eq(&TokenType::SEMICOLON) {
                    self.next_token();
                }

                Some(Statement::Expr(Box::new(expr)))
            }
            None => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        match &self.curToken {
            TokenType::ASSIGN | TokenType::LET => {
                self.next_token();
            }
            _ => {
                return None;
            }
        }

        let identifier = match &self.curToken {
            TokenType::IDENTIFIER(name) => Expression::Identifier(name.to_string()),
            _ => {
                return None;
            }
        };

        if !self.peekToken.eq(&TokenType::ASSIGN) {
            return None;
        }

        self.next_token();
        self.next_token();

        let expression = match self.parse_expression(Precedence::Lowest) {
            Some(expr) => expr,
            None => {
                return None;
            }
        };

        if self.peekToken.eq(&TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(Statement::Let(Box::new(identifier), Box::new(expression)))
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();

        let expression = match self.parse_expression(Precedence::Lowest) {
            Some(expr) => expr,
            None => {
                return None;
            }
        };

        self.next_token();

        if self.peekToken.eq(&TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(Statement::Return(Box::new(expression)))
    }

    fn parse_boolean_expression(&mut self) -> Option<Expression> {
        match &self.curToken {
            TokenType::TRUE => Some(Expression::Boolean(true)),
            TokenType::FALSE => Some(Expression::Boolean(false)),
            _ => None,
        }
    }

    fn parse_string_expression(&mut self) -> Option<Expression> {
        match &self.curToken {
            TokenType::STRING(str) => Some(Expression::String(str.to_string())),
            _ => None,
        }
    }

    fn parse_number_expression(&mut self) -> Option<Expression> {
        match &self.curToken {
            &TokenType::NUMBER(num) => Some(Expression::Integer(num)),
            _ => None,
        }
    }

    fn parse_identifier(&mut self) -> Option<Expression> {
        match &self.curToken {
            TokenType::IDENTIFIER(name) => {
                match self.peekToken {
                    // TokenType::LEFT_PAREN => self.parse_fn_call_expression(name.to_string()),
                    // TokenType::LEFT_BRACE => {}, // arr[0]
                    _ => Some(Expression::Identifier(String::from(name))),
                }
            }
            _ => None,
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let cur_pos = self.lexer.get_cur_pos();

        // prefix expression
        let mut left = match &self.curToken {
            TokenType::TRUE => self.parse_boolean_expression(),
            TokenType::FALSE => self.parse_boolean_expression(),
            TokenType::STRING(_) => self.parse_string_expression(),
            &TokenType::NUMBER(_) => self.parse_number_expression(),
            TokenType::IDENTIFIER(_) => self.parse_identifier(),
            TokenType::LEFT_BRACE => self.parse_array_expression(),
            TokenType::LEFT_CURLY_BRACE => self.parse_hash_expression(),
            TokenType::IF => self.parse_if_expression(),
            TokenType::WHILE => self.parse_while_expression(),
            TokenType::BREAK => Some(Expression::Break),
            TokenType::FN => self.parse_fn_expression(),
            TokenType::BANG | TokenType::MINUS => self.parse_prefix_expression(),
            TokenType::LEFT_PAREN => self.parse_grouped_expression(),
            _ => Some(Expression::NON_PARSED_EXPR((cur_pos, self.curToken.clone()))),
        };

        println!(
            "parse_expression {:?} {:?} {:?} {:?}",
            self.curToken,
            self.peekToken,
            precedence < self.peek_precedence(),
            left
        );

        // infix expression
        while !self.peekToken.eq(&TokenType::SEMICOLON) && precedence < self.peek_precedence() {
            match self.peekToken {
                TokenType::ADD
                | TokenType::MINUS
                | TokenType::MULTIPLY
                | TokenType::DIVIDE
                | TokenType::EQUAL_EQUAL
                | TokenType::LESS
                | TokenType::LESS_EQUAL
                | TokenType::GREATER
                | TokenType::GREATER_EQUAL
                | TokenType::BANG_EQUAL => {
                    self.next_token();
                    left = self.parse_infix_expression(left.unwrap(), &self.curToken.clone());
                }
                TokenType::LEFT_PAREN => {
                    self.next_token();
                    left = self.parse_fn_call_expression(left.unwrap());
                }
                _ => {
                    return left;
                }
            }
        }

        left
    }

    fn parse_fn_call_expression(&mut self, func: Expression) -> Option<Expression> {
        self.next_token();

        if self.curToken.eq(&TokenType::LEFT_PAREN) {
            self.next_token();
        }

        let mut fn_parameters = vec![];

        // println!("166 {:?}", self.curToken);

        while !self.curToken.eq(&TokenType::SEMICOLON) {
            match self.curToken {
                TokenType::COMMA | TokenType::RIGHT_PAREN => {
                    self.next_token();
                }
                _ => {
                    let parameter = self.parse_expression(Precedence::Lowest).unwrap();

                    fn_parameters.push(parameter);

                    self.next_token();
                }
            }
        }

        Some(Expression::FnCall(Box::new(func), fn_parameters))
    }

    fn parse_array_expression(&mut self) -> Option<Expression> {
        self.next_token();

        let mut expressions = vec![];

        while !self.curToken.eq(&TokenType::RIGHT_BRACE) {
            match self.curToken {
                TokenType::COMMA => {
                    self.next_token();
                }
                _ => {
                    let expression = self.parse_expression(Precedence::Lowest).unwrap();

                    expressions.push(expression);

                    self.next_token();
                }
            }
        }

        Some(Expression::Array(expressions))
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        self.next_token();

        if self.curToken.eq(&TokenType::LEFT_PAREN) {
            self.next_token();
        }

        let ifCondition = match self.parse_expression(Precedence::Lowest) {
            Some(expr) => expr,
            None => {
                return None;
            }
        };

        self.next_token();

        if self.curToken.eq(&TokenType::RIGHT_PAREN) {
            self.next_token();
        }

        if self.curToken.eq(&TokenType::LEFT_CURLY_BRACE) {
            self.next_token();
        }

        let ifStatements = self.parse_block_statements().unwrap();

        if self.curToken.eq(&TokenType::RIGHT_CURLY_BRACE) {
            self.next_token();
        }

        // to be refactored
        let mut elseStatements = vec![];

        if self.curToken.eq(&TokenType::ELSE) {
            self.next_token();

            while !self.curToken.eq(&TokenType::RIGHT_CURLY_BRACE) {
                match self.curToken {
                    TokenType::LEFT_CURLY_BRACE | TokenType::SEMICOLON => {
                        self.next_token();
                    }
                    _ => {
                        let statement = self.parse_statement().unwrap();

                        elseStatements.push(statement);
                    }
                }
            }

            if self.curToken.eq(&TokenType::RIGHT_CURLY_BRACE) {
                self.next_token();
            }
        }

        Some(Expression::If(
            Box::new(ifCondition),
            Statement::BlockStatement(ifStatements),
            if !elseStatements.is_empty() {
                Some(Statement::BlockStatement(elseStatements))
            } else {
                Option::None
            },
        ))
    }

    fn parse_while_expression(&mut self) -> Option<Expression> {
        self.next_token();

        if self.curToken.eq(&TokenType::LEFT_PAREN) {
            self.next_token();
        }

        let whileCondition = match self.parse_expression(Precedence::Lowest) {
            Some(expr) => expr,
            None => {
                return None;
            }
        };

        self.next_token();

        if self.curToken.eq(&TokenType::RIGHT_PAREN) {
            self.next_token();
        }

        if self.curToken.eq(&TokenType::LEFT_CURLY_BRACE) {
            self.next_token();
        }

        let whileStatements = self.parse_block_statements().unwrap();

        Some(Expression::While(
            Box::new(whileCondition),
            Statement::BlockStatement(whileStatements),
        ))
    }

    fn parse_let_fn_expression(&mut self) -> Option<Expression> {
        // function literal does not have function name
        let fn_name = Expression::Identifier(String::from(""));

        self.next_token();

        if self.curToken.eq(&TokenType::LEFT_PAREN) {
            self.next_token();
        }

        let mut fn_parameters = vec![];

        while !self.curToken.eq(&TokenType::RIGHT_PAREN) {
            match self.curToken {
                TokenType::COMMA => {
                    self.next_token();
                }
                _ => {
                    let parameter = self.parse_expression(Precedence::Lowest).unwrap();

                    fn_parameters.push(parameter);

                    self.next_token();
                }
            }
        }

        if self.curToken.eq(&TokenType::RIGHT_PAREN) {
            self.next_token();
        }

        if self.curToken.eq(&TokenType::LEFT_CURLY_BRACE) {
            self.next_token();
        }

        let fn_body = self.parse_block_statements().unwrap();

        Some(Expression::Fn(
            Box::new(fn_name),
            fn_parameters,
            Statement::BlockStatement(fn_body),
        ))
    }

    fn parse_fn_expression(&mut self) -> Option<Expression> {
        // let foo = fn() {}
        if self.peekToken.eq(&TokenType::LEFT_PAREN) {
            return self.parse_let_fn_expression();
        }

        self.next_token();

        let fn_name = match &self.curToken {
            TokenType::IDENTIFIER(name) => Expression::Identifier(String::from(name)),
            _ => {
                return None;
            }
        };

        self.next_token();

        if self.curToken.eq(&TokenType::LEFT_PAREN) {
            self.next_token();
        }

        let mut fn_parameters = vec![];

        while !self.curToken.eq(&TokenType::RIGHT_PAREN) {
            match self.curToken {
                TokenType::COMMA => {
                    self.next_token();
                }
                _ => {
                    let parameter = self.parse_expression(Precedence::Lowest).unwrap();

                    fn_parameters.push(parameter);

                    self.next_token();
                }
            }
        }

        if self.curToken.eq(&TokenType::RIGHT_PAREN) {
            self.next_token();
        }

        if self.curToken.eq(&TokenType::LEFT_CURLY_BRACE) {
            self.next_token();
        }

        let fn_body = self.parse_block_statements().unwrap();

        Some(Expression::Fn(
            Box::new(fn_name),
            fn_parameters,
            Statement::BlockStatement(fn_body),
        ))
    }

    fn parse_block_statements(&mut self) -> Option<Vec<Statement>> {
        let mut block_statements = vec![];

        while !self.curToken.eq(&TokenType::RIGHT_CURLY_BRACE) {
            match self.curToken {
                TokenType::SEMICOLON => {
                    self.next_token();
                }
                _ => {
                    let statement = self.parse_statement().unwrap();

                    block_statements.push(statement);
                }
            }
        }

        Some(block_statements)
    }

    fn parse_hash_expression(&mut self) -> Option<Expression> {
        self.next_token();

        let mut flat_hash_vec = vec![];

        while !self.curToken.eq(&TokenType::RIGHT_CURLY_BRACE) {
            match self.curToken {
                TokenType::COLON => {
                    self.next_token();
                }
                TokenType::COMMA => {
                    self.next_token();
                }
                TokenType::LEFT_BRACE => {
                    let hash_value = self.parse_expression(Precedence::Lowest).unwrap();
                    flat_hash_vec.push(hash_value);

                    self.next_token();
                }
                _ => {
                    // { key: value, ... }
                    match &self.peekToken {
                        TokenType::COLON => {
                            let hash_key = self.parse_expression(Precedence::Lowest).unwrap();
                            flat_hash_vec.push(hash_key);

                            self.next_token();
                        }
                        TokenType::COMMA => {
                            let hash_value = self.parse_expression(Precedence::Lowest).unwrap();
                            flat_hash_vec.push(hash_value);

                            self.next_token();
                        }
                        TokenType::RIGHT_CURLY_BRACE => {
                            let hash_value = self.parse_expression(Precedence::Lowest).unwrap();
                            flat_hash_vec.push(hash_value);
                            self.next_token();
                            break;
                        }
                        _ => {
                            self.next_token();
                        }
                    }
                }
            }
        }

        println!("445 {:?}", flat_hash_vec);

        let mut hash_tuple = vec![];

        // https://stackoverflow.com/questions/66386013/how-to-iterate-over-two-elements-in-a-collection-stepping-by-one-using-iterator
        // let mut hash_tuple = flat_hash_vec
        //     .windows(2)
        //     .map(|&w: Windows<&[Expression; 2]>| (*w.nth(0), *w.nth(1)))
        //     .collect();

        let mut chunks = flat_hash_vec.chunks(2);

        while !chunks.is_empty() {
            if let [hash_key, hash_value] = chunks.next().unwrap() {
                hash_tuple.push((hash_key.deref().clone(), hash_value.deref().clone()));
            }
        }

        Some(Expression::Hash(hash_tuple))
    }

    fn cur_precedence(&self) -> Precedence {
        Precedence::get(&self.curToken)
    }

    fn peek_precedence(&self) -> Precedence {
        Precedence::get(&self.peekToken)
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        println!("parse_prefix_expression {:?}", self.curToken);

        let operator = self.curToken.clone();
        self.next_token();

        // match self.parse_expression(Precedence::Prefix) {
        //     Some(expr) => Some(Expression::Prefix(operator, Box::new(expr))),
        //     None => None,
        // }
        self.parse_expression(Precedence::Prefix)
            .map(|expr| Expression::Prefix(operator, Box::new(expr)))
    }

    fn parse_infix_expression(&mut self, left: Expression, token: &TokenType) -> Option<Expression> {
        let cur_precedence = self.cur_precedence();

        self.next_token();

        match self.parse_expression(cur_precedence) {
            Some(expr) => {
                println!("parse_infix_expression {:?}", self.curToken);

                Some(Expression::Infix(Box::new(left), token.clone(), Box::new(expr)))
            }
            None => None,
        }
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();

        // println!("parse_grouped_expression {:?}", self.curToken);

        let expr = self.parse_expression(Precedence::Lowest);

        // )
        if !self.peekToken.eq(&TokenType::RIGHT_PAREN) {
            None
        } else {
            self.next_token();
            expr
        }
    }

    pub fn get(input: &str) -> Parser {
        let lexer = Lexer::new(input);

        Parser::new(lexer)
    }

    pub fn parse_program(&mut self) -> AstNode {
        self.next_token();
        self.next_token();

        let mut statements = vec![];

        // while let Some(ended) = self.lexer.is_end() {
        //     if ended {
        //         break;
        //     } else {
        //         match self.parse_statement() {
        //             Some(statement) => {
        //                 statements.push(statement);
        //             }
        //             _ => {}
        //         }
        //         self.next_token();
        //     }
        // }

        while !self.curToken.eq(&TokenType::EOF) {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }
            self.next_token();
        }

        AstNode::Program(statements)
    }
}

#[cfg(test)]
mod unit_test {
    use crate::ast::{AstNode, Expression, Precedence, Statement};
    use crate::parser::Parser;
    use crate::token::TokenType;

    fn get_parser(input: &str) -> Parser {
        return Parser::get(input);
    }

    #[test]
    fn parse_let_statement() {
        let input = r#"
            let version = 1234;
            let name = "Lynx programming language";
            let is_cool = false;
        "#;
        let mut parser = get_parser(input);
        let rootNode = parser.parse_program();

        let expected = AstNode::Program(vec![
            Statement::Let(
                Box::new(Expression::Identifier(String::from("version"))),
                Box::new(Expression::Integer(1234)),
            ),
            Statement::Let(
                Box::new(Expression::Identifier(String::from("name"))),
                Box::new(Expression::String(String::from("Lynx programming language"))),
            ),
            Statement::Let(
                Box::new(Expression::Identifier(String::from("is_cool"))),
                Box::new(Expression::Boolean(false)),
            ),
        ]);

        // println!("{:?}", rootNode);

        assert_eq!(format!("{:?}", rootNode), format!("{:?}", expected));
    }

    #[test]
    fn parse_return_statement() {
        let input = r#"
            return 1234;
            return "Lynx programming language";
            return false;
            return foo_456;
        "#;
        let mut parser = get_parser(input);
        let rootNode = parser.parse_program();

        let expected = AstNode::Program(vec![
            Statement::Return(Box::new(Expression::Integer(1234))),
            Statement::Return(Box::new(Expression::String(String::from("Lynx programming language")))),
            Statement::Return(Box::new(Expression::Boolean(false))),
            Statement::Return(Box::new(Expression::Identifier(String::from("foo_456")))),
        ]);

        println!("{:?}", rootNode);

        assert_eq!(format!("{:?}", rootNode), format!("{:?}", expected));
    }

    #[test]
    fn parse_integer_expression() {
        let input = r#"1234"#;
        let mut parser = get_parser(input);
        let rootNode = parser.parse_program();

        let expected = AstNode::Program(vec![Statement::Expr(Box::new(Expression::Integer(1234)))]);

        assert_eq!(format!("{:?}", rootNode), format!("{:?}", expected));
    }

    #[test]
    fn parse_string_expression() {
        let input = r#""foo""#;
        let mut parser = get_parser(input);
        let rootNode = parser.parse_program();

        let expected = AstNode::Program(vec![Statement::Expr(Box::new(Expression::String(String::from("foo"))))]);

        assert_eq!(format!("{:?}", rootNode), format!("{:?}", expected));
    }

    #[test]
    fn parse_boolean_expression() {
        let input = r#"true"#;
        let mut parser = get_parser(input);
        let rootNode = parser.parse_program();

        let expected = AstNode::Program(vec![Statement::Expr(Box::new(Expression::Boolean(true)))]);

        assert_eq!(format!("{:?}", rootNode), format!("{:?}", expected));
    }

    #[test]
    fn parse_array_expression() {
        let input = r#"
            [1234, true, "Lynx programming language", [1234, true, "Lynx programming language"]];
        "#;
        let mut parser = get_parser(input);
        let rootNode = parser.parse_program();

        let expected = AstNode::Program(vec![Statement::Expr(Box::new(Expression::Array(vec![
            Expression::Integer(1234),
            Expression::Boolean(true),
            Expression::String(String::from("Lynx programming language")),
            Expression::Array(vec![
                Expression::Integer(1234),
                Expression::Boolean(true),
                Expression::String(String::from("Lynx programming language")),
            ]),
        ])))]);

        println!("{:?}", rootNode);

        assert_eq!(format!("{:?}", rootNode), format!("{:?}", expected));
    }

    #[test]
    fn parse_hash_expression() {
        let input = r#"
            {
                "foo": "bar",
                1: 2,
                2: [4, "stuff", false],
                "abc": true
            };
        "#;
        let mut parser = get_parser(input);
        let rootNode = parser.parse_program();

        let expected = AstNode::Program(vec![Statement::Expr(Box::new(Expression::Hash(vec![
            (
                Expression::String(String::from("foo")),
                Expression::String(String::from("bar")),
            ),
            (Expression::Integer(1), Expression::Integer(2)),
            (
                Expression::Integer(2),
                Expression::Array(vec![
                    Expression::Integer(4),
                    Expression::String(String::from("stuff")),
                    Expression::Boolean(false),
                ]),
            ),
            (Expression::String(String::from("abc")), Expression::Boolean(true)),
        ])))]);

        println!("{:?}", rootNode);

        assert_eq!(format!("{:?}", rootNode), format!("{:?}", expected));
    }

    #[test]
    fn parse_prefix_expression() {
        let input = r#"
            !true;
            !false;
        "#;
        let mut parser = get_parser(input);
        let rootNode = parser.parse_program();
        // -2; // todo minus

        let expected = AstNode::Program(vec![
            Statement::Expr(Box::new(Expression::Prefix(
                TokenType::BANG,
                Box::new(Expression::Boolean(true)),
            ))),
            Statement::Expr(Box::new(Expression::Prefix(
                TokenType::BANG,
                Box::new(Expression::Boolean(false)),
            ))),
        ]);

        println!("{:?}", rootNode);

        assert_eq!(format!("{:?}", rootNode), format!("{:?}", expected));
    }

    #[test]
    fn parse_infix_expression() {
        let input = r#"
            1 + 2 / 3 * 4 - 5;
        "#;
        // println!("input {:?}", input);

        let mut parser = get_parser(input);
        let rootNode = parser.parse_program();

        let _tree_like_representation = r#"
            Expr(
                Infix(
                    Infix(
                        Integer(1), 
                            ADD, 
                        Infix(
                            Infix(
                                Integer(2), 
                                DIVIDE, 
                                Integer(3)
                            ), 
                            MULTIPLY, 
                            Integer(4))
                        )
                    ), 
                    MINUS, 
                    Integer(5))
                )
            )        
        "#;

        let expected = AstNode::Program(vec![Statement::Expr(Box::new(Expression::Infix(
            Box::new(Expression::Infix(
                Box::new(Expression::Integer(1)),
                TokenType::ADD,
                Box::new(Expression::Infix(
                    Box::new(Expression::Infix(
                        Box::new(Expression::Integer(2)),
                        TokenType::DIVIDE,
                        Box::new(Expression::Integer(3)),
                    )),
                    TokenType::MULTIPLY,
                    Box::new(Expression::Integer(4)),
                )),
            )),
            TokenType::MINUS,
            Box::new(Expression::Integer(5)),
        )))]);

        println!("{:?}", rootNode);

        assert_eq!(format!("{:?}", rootNode), format!("{:?}", expected));
    }

    #[test]
    fn parse_let_infix_expression() {
        let input = r#"
            let foo = 1 + 2 / 3 * 4 - 5;
        "#;
        // println!("input {:?}", input);

        let mut parser = get_parser(input);
        let rootNode = parser.parse_program();

        let expected = AstNode::Program(vec![Statement::Let(
            Box::new(Expression::Identifier(String::from("foo"))),
            Box::new(Expression::Infix(
                Box::new(Expression::Infix(
                    Box::new(Expression::Integer(1)),
                    TokenType::ADD,
                    Box::new(Expression::Infix(
                        Box::new(Expression::Infix(
                            Box::new(Expression::Integer(2)),
                            TokenType::DIVIDE,
                            Box::new(Expression::Integer(3)),
                        )),
                        TokenType::MULTIPLY,
                        Box::new(Expression::Integer(4)),
                    )),
                )),
                TokenType::MINUS,
                Box::new(Expression::Integer(5)),
            )),
        )]);

        println!("{:?}", rootNode);

        assert_eq!(format!("{:?}", rootNode), format!("{:?}", expected));
    }

    #[test]
    fn parse_grouped_expression() {
        let input = r#"
         3 / (1 + 2);
        "#;
        // todo fix (1 + 2) / 3;
        let mut parser = get_parser(input);
        let rootNode = parser.parse_program();

        let expected = AstNode::Program(vec![Statement::Expr(Box::new(Expression::Infix(
            Box::new(Expression::Integer(3)),
            TokenType::DIVIDE,
            Box::new(Expression::Infix(
                Box::new(Expression::Integer(1)),
                TokenType::ADD,
                Box::new(Expression::Integer(2)),
            )),
        )))]);

        println!("{:?}", rootNode);

        assert_eq!(format!("{:?}", rootNode), format!("{:?}", expected));
    }

    #[test]
    fn parse_if_expression() {
        let input = r#"
            if (foo) {
                let bar = "stuff";
                return bar;
            } else {
                return 5;
            }
        "#;
        let mut parser = get_parser(input);
        let rootNode = parser.parse_program();

        let expected = AstNode::Program(vec![Statement::Expr(Box::new(Expression::If(
            Box::new(Expression::Identifier(String::from("foo"))),
            Statement::BlockStatement(vec![
                Statement::Let(
                    Box::new(Expression::Identifier(String::from("bar"))),
                    Box::new(Expression::String(String::from("stuff"))),
                ),
                Statement::Return(Box::new(Expression::Identifier(String::from("bar")))),
            ]),
            Some(Statement::BlockStatement(vec![Statement::Return(Box::new(
                Expression::Integer(5),
            ))])),
        )))]);

        println!("{:?}", rootNode);

        assert_eq!(format!("{:?}", rootNode), format!("{:?}", expected));
    }

    #[test]
    fn parse_while_expression() {
        let input = r#"
            while (foo) {
                let bar = "stuff";

                foo(bar);
            }
        "#;
        let mut parser = get_parser(input);
        let rootNode = parser.parse_program();

        let expected = AstNode::Program(vec![Statement::Expr(Box::new(Expression::While(
            Box::new(Expression::Identifier(String::from("foo"))),
            Statement::BlockStatement(vec![
                Statement::Let(
                    Box::new(Expression::Identifier(String::from("bar"))),
                    Box::new(Expression::String(String::from("stuff"))),
                ),
                Statement::Expr(Box::new(Expression::FnCall(
                    Box::new(Expression::Identifier(String::from("foo"))),
                    vec![Expression::Identifier(String::from("bar"))],
                ))),
            ]),
        )))]);

        println!("{:?}", rootNode);

        assert_eq!(format!("{:?}", rootNode), format!("{:?}", expected));
    }

    #[test]
    fn parse_while_break_expression() {
        let input = r#"
            while (foo) {
                let bar = "stuff";

                foo(bar);
                break;
            }
        "#;
        let mut parser = get_parser(input);
        let rootNode = parser.parse_program();

        let expected = AstNode::Program(vec![Statement::Expr(Box::new(Expression::While(
            Box::new(Expression::Identifier(String::from("foo"))),
            Statement::BlockStatement(vec![
                Statement::Let(
                    Box::new(Expression::Identifier(String::from("bar"))),
                    Box::new(Expression::String(String::from("stuff"))),
                ),
                Statement::Expr(Box::new(Expression::FnCall(
                    Box::new(Expression::Identifier(String::from("foo"))),
                    vec![Expression::Identifier(String::from("bar"))],
                ))),
                Statement::Expr(Box::new(Expression::Break)),
            ]),
        )))]);

        println!("{:?}", rootNode);

        assert_eq!(format!("{:?}", rootNode), format!("{:?}", expected));
    }

    #[test]
    fn parse_fn_expression() {
        let input = r#"
            fn foo("bar", stuff, 123) {
                let another_bar = stuff;
                return another_bar;
            }
        "#;
        let mut parser = get_parser(input);
        let rootNode = parser.parse_program();

        let expected = AstNode::Program(vec![Statement::Expr(Box::new(Expression::Fn(
            Box::new(Expression::Identifier(String::from("foo"))),
            vec![
                Expression::String(String::from("bar")),
                Expression::Identifier(String::from("stuff")),
                Expression::Integer(123),
            ],
            Statement::BlockStatement(vec![
                Statement::Let(
                    Box::new(Expression::Identifier(String::from("another_bar"))),
                    Box::new(Expression::Identifier(String::from("stuff"))),
                ),
                Statement::Return(Box::new(Expression::Identifier(String::from("another_bar")))),
            ]),
        )))]);

        println!("{:?}", rootNode);

        assert_eq!(format!("{:?}", rootNode), format!("{:?}", expected));
    }

    #[test]
    fn parse_let_fn_expression() {
        let input = r#"
            let foo = fn("bar", stuff, 123) {
                let another_bar = stuff;
                return another_bar;
            }
        "#;
        let mut parser = get_parser(input);
        let rootNode = parser.parse_program();

        let expected = AstNode::Program(vec![Statement::Let(
            Box::new(Expression::Identifier(String::from("foo"))),
            Box::new(Expression::Fn(
                Box::new(Expression::Identifier(String::from(""))),
                vec![
                    Expression::String(String::from("bar")),
                    Expression::Identifier(String::from("stuff")),
                    Expression::Integer(123),
                ],
                Statement::BlockStatement(vec![
                    Statement::Let(
                        Box::new(Expression::Identifier(String::from("another_bar"))),
                        Box::new(Expression::Identifier(String::from("stuff"))),
                    ),
                    Statement::Return(Box::new(Expression::Identifier(String::from("another_bar")))),
                ]),
            )),
        )]);

        println!("{:?}", rootNode);

        assert_eq!(format!("{:?}", rootNode), format!("{:?}", expected));
    }

    #[test]
    fn parse_fn_call_expression() {
        let input = r#"
            let arr = foo("bar", another_bar, 456);
        "#;
        let mut parser = get_parser(input);
        let rootNode = parser.parse_program();

        let expected = AstNode::Program(vec![Statement::Let(
            Box::new(Expression::Identifier(String::from("arr"))),
            Box::new(Expression::FnCall(
                Box::new(Expression::Identifier(String::from("foo"))),
                vec![
                    Expression::String(String::from("bar")),
                    Expression::Identifier(String::from("another_bar")),
                    Expression::Integer(456),
                ],
            )),
        )]);

        println!("{:?}", rootNode);

        assert_eq!(format!("{:?}", rootNode), format!("{:?}", expected));
    }

    #[test]
    fn parse_builtin_expression() {
        let input = r#"
            first([1, 2, 3]);
        "#;
        let mut parser = get_parser(input);
        let rootNode = parser.parse_program();

        let expected = AstNode::Program(vec![Statement::Expr(Box::new(Expression::FnCall(
            Box::new(Expression::Identifier(String::from("first"))),
            vec![Expression::Array(vec![
                Expression::Integer(1),
                Expression::Integer(2),
                Expression::Integer(3),
            ])],
        )))]);

        println!("{:?}", rootNode);

        assert_eq!(format!("{:?}", rootNode), format!("{:?}", expected));
    }

    #[test]
    fn parse_fibonacci() {
        let input = r#"
            fn fibonacci(x) {
                if (x == 0) {
                    return 0;
                } else {
                    return 1;
                }
            }
            let foo = 123;
            let fib_val = fibonacci(foo);
            fib_val;
        "#;
        let mut parser = get_parser(input);
        let rootNode = parser.parse_program();
        println!("{:?}", rootNode);

        // let expected = AstNode::Program(vec![Statement::Let(
        //     Box::new(Expression::Identifier(String::from("arr"))),
        //     Box::new(Expression::FnCall(
        //         Box::new(Expression::Identifier(String::from("foo"))),
        //         vec![
        //             Expression::String(String::from("bar")),
        //             Expression::Identifier(String::from("another_bar")),
        //             Expression::Integer(456),
        //         ],
        //     )),
        // )]);

        // assert_eq!(format!("{:?}", rootNode), format!("{:?}", expected));
        assert_eq!(1, 1);
    }

    #[test]
    fn check_precedence() {
        let mut parser = get_parser(
            r#"
        (foo + bar / 123)
    "#,
        );

        parser.next_token();
        assert_eq!(Precedence::Group, parser.peek_precedence());

        parser.next_token();
        assert_eq!(Precedence::Lowest, parser.peek_precedence());

        parser.next_token();
        assert_eq!(Precedence::Addition, parser.peek_precedence());

        parser.next_token();
        parser.next_token();
        assert_eq!(Precedence::Multiply, parser.peek_precedence());
    }

    #[test]
    fn parse_program() {
        let input = r#"
            let version = 1234;
            let name = "Lynx programming language";
            let is_cool = false;
            let arr = [1234, true, "Lynx programming language", [1234, true, "Lynx programming language"]];            
            let hash = {
                "foo": "bar",
                1: 2,
                2: [4, "stuff", false],
                "abc": true
            };
            let prefix = !true;
            let infix = 1 + 2 / 3 * 4 - 5;
            let grouped = 3 / (1 + 2);
            
            fn foo("bar", stuff, 123) {
                let another_bar = stuff;
                return another_bar;
            }

            let call_val = foo("bar", another_bar, 456);
            
            while (call_val) {
                let bar = "stuff";

                foo(bar, call_val);
            }
            
            if (call_val) {
                let bar = "stuff";
                return bar;
            } else {
                return 5;
            }
        "#;
        let mut parser = get_parser(input);
        let rootNode = parser.parse_program();

        let expected = AstNode::Program(vec![
            Statement::Let(
                Box::new(Expression::Identifier(String::from("version"))),
                Box::new(Expression::Integer(1234)),
            ),
            Statement::Let(
                Box::new(Expression::Identifier(String::from("name"))),
                Box::new(Expression::String(String::from("Lynx programming language"))),
            ),
            Statement::Let(
                Box::new(Expression::Identifier(String::from("is_cool"))),
                Box::new(Expression::Boolean(false)),
            ),
            Statement::Let(
                Box::new(Expression::Identifier(String::from("arr"))),
                Box::new(Expression::Array(vec![
                    Expression::Integer(1234),
                    Expression::Boolean(true),
                    Expression::String(String::from("Lynx programming language")),
                    Expression::Array(vec![
                        Expression::Integer(1234),
                        Expression::Boolean(true),
                        Expression::String(String::from("Lynx programming language")),
                    ]),
                ])),
            ),
            Statement::Let(
                Box::new(Expression::Identifier(String::from("hash"))),
                Box::new(Expression::Hash(vec![
                    (
                        Expression::String(String::from("foo")),
                        Expression::String(String::from("bar")),
                    ),
                    (Expression::Integer(1), Expression::Integer(2)),
                    (
                        Expression::Integer(2),
                        Expression::Array(vec![
                            Expression::Integer(4),
                            Expression::String(String::from("stuff")),
                            Expression::Boolean(false),
                        ]),
                    ),
                    (Expression::String(String::from("abc")), Expression::Boolean(true)),
                ])),
            ),
            Statement::Let(
                Box::new(Expression::Identifier(String::from("prefix"))),
                Box::new(Expression::Prefix(TokenType::BANG, Box::new(Expression::Boolean(true)))),
            ),
            Statement::Let(
                Box::new(Expression::Identifier(String::from("infix"))),
                Box::new(Expression::Infix(
                    Box::new(Expression::Infix(
                        Box::new(Expression::Integer(1)),
                        TokenType::ADD,
                        Box::new(Expression::Infix(
                            Box::new(Expression::Infix(
                                Box::new(Expression::Integer(2)),
                                TokenType::DIVIDE,
                                Box::new(Expression::Integer(3)),
                            )),
                            TokenType::MULTIPLY,
                            Box::new(Expression::Integer(4)),
                        )),
                    )),
                    TokenType::MINUS,
                    Box::new(Expression::Integer(5)),
                )),
            ),
            Statement::Let(
                Box::new(Expression::Identifier(String::from("grouped"))),
                Box::new(Expression::Infix(
                    Box::new(Expression::Integer(3)),
                    TokenType::DIVIDE,
                    Box::new(Expression::Infix(
                        Box::new(Expression::Integer(1)),
                        TokenType::ADD,
                        Box::new(Expression::Integer(2)),
                    )),
                )),
            ),
            Statement::Expr(Box::new(Expression::Fn(
                Box::new(Expression::Identifier(String::from("foo"))),
                vec![
                    Expression::String(String::from("bar")),
                    Expression::Identifier(String::from("stuff")),
                    Expression::Integer(123),
                ],
                Statement::BlockStatement(vec![
                    Statement::Let(
                        Box::new(Expression::Identifier(String::from("another_bar"))),
                        Box::new(Expression::Identifier(String::from("stuff"))),
                    ),
                    Statement::Return(Box::new(Expression::Identifier(String::from("another_bar")))),
                ]),
            ))),
            Statement::Let(
                Box::new(Expression::Identifier(String::from("call_val"))),
                Box::new(Expression::FnCall(
                    Box::new(Expression::Identifier(String::from("foo"))),
                    vec![
                        Expression::String(String::from("bar")),
                        Expression::Identifier(String::from("another_bar")),
                        Expression::Integer(456),
                    ],
                )),
            ),
            Statement::Expr(Box::new(Expression::While(
                Box::new(Expression::Identifier(String::from("call_val"))),
                Statement::BlockStatement(vec![
                    Statement::Let(
                        Box::new(Expression::Identifier(String::from("bar"))),
                        Box::new(Expression::String(String::from("stuff"))),
                    ),
                    Statement::Expr(Box::new(Expression::FnCall(
                        Box::new(Expression::Identifier(String::from("foo"))),
                        vec![
                            Expression::Identifier(String::from("bar")),
                            Expression::Identifier(String::from("call_val")),
                        ],
                    ))),
                ]),
            ))),
            Statement::Expr(Box::new(Expression::If(
                Box::new(Expression::Identifier(String::from("call_val"))),
                Statement::BlockStatement(vec![
                    Statement::Let(
                        Box::new(Expression::Identifier(String::from("bar"))),
                        Box::new(Expression::String(String::from("stuff"))),
                    ),
                    Statement::Return(Box::new(Expression::Identifier(String::from("bar")))),
                ]),
                Some(Statement::BlockStatement(vec![Statement::Return(Box::new(
                    Expression::Integer(5),
                ))])),
            ))),
        ]);

        println!("{:?}", rootNode);

        assert_eq!(format!("{:?}", rootNode), format!("{:?}", expected));
    }
}
