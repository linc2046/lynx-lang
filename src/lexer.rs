#![warn(clippy::clone_double_ref)]
#![allow(clippy::len_zero)]

use crate::token::TokenType;
use crate::util::{is_identifier, is_white_space};
use std::iter::FromIterator;
use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<CharIndices<'a>>,
    current_pos: Option<usize>,
    len: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.char_indices().peekable(),
            current_pos: Some(0),
            len: input.chars().count(),
        }
    }

    pub fn get_cur_pos(&self) -> usize {
        self.current_pos.unwrap()
    }

    pub fn is_end(&self) -> Option<bool> {
        let pos = self.current_pos.unwrap();

        Some(pos >= self.len - 1)
    }

    pub fn is_peekable(&mut self) -> bool {
        self.input.next_if(|(_, x)| x.len_utf8() > 0).is_some()
    }

    pub fn skip_white_space(&mut self) {
        if let Some((_, c)) = self.peek_char() {
            if is_white_space(*c) {
                self.next_char();
            }
        }
    }

    pub fn next_char(&mut self) -> Option<(usize, char)> {
        let (pos, c) = self.input.next().unwrap();
        self.current_pos = Some(pos);

        // println!("debug char {:?}", c);

        Some((pos, c))
    }

    pub fn peek_char(&mut self) -> Option<&(usize, char)> {
        self.input.peek()
    }

    pub fn read_identifier(&mut self, c: char) -> Vec<char> {
        let mut identifier = vec![];

        if is_identifier(c) {
            identifier.push(c);
        }

        // https://caniuse.rs/features/let_chains
        // unstable let chains
        // while let Some(&(_, name)) = self.peek_char() && is_identifier(name) {
        //     identifier.push(name);
        //     self.next_char();
        // }

        while let Some(&(_, name)) = self.peek_char() {
            if is_identifier(name) {
                identifier.push(name);
                self.next_char();
            } else {
                break;
            }
        }

        identifier
    }

    pub fn read_keyword(&mut self, c: char) -> TokenType {
        let ident_name = self.read_identifier(c);

        if ident_name.len() > 0 {
            let name = String::from_iter(ident_name.into_iter());
            match name.as_str() {
                "true" => TokenType::TRUE,
                "false" => TokenType::FALSE,
                "let" => TokenType::LET,
                "fn" => TokenType::FN,
                "if" => TokenType::IF,
                "else" => TokenType::ELSE,
                "while" => TokenType::WHILE,
                "break" => TokenType::BREAK,
                "return" => TokenType::RETURN,
                // "print" => TokenType::PRINT,
                // "len" => TokenType::LEN,
                // "first" => TokenType::FIRST,
                // "last" => TokenType::LAST,
                // "rest" => TokenType::REST,
                // "push" => TokenType::PUSH,
                _ => TokenType::IDENTIFIER(name),
            }
        } else {
            // TokenType::NONE
            self.next_token()
        }
    }

    pub fn read_string(&mut self, c: char) -> Option<TokenType> {
        let mut chars = vec![];

        if c == '"' {
            while let Some(&(_, p)) = self.peek_char() {
                if p != '"' {
                    // println!("102 {:?}", p);

                    chars.push(p);
                    self.next_char();
                } else {
                    self.next_char();
                    break;
                }
            }

            let slices = chars.into_iter();
            Some(TokenType::STRING(String::from_iter(slices)))
        } else {
            None
        }
    }

    pub fn read_number(&mut self, c: char) -> Option<TokenType> {
        let mut chars = vec![];
        if c.is_numeric() {
            chars.push(c);
        }

        while let Some(&(_, p)) = self.peek_char() {
            if p.is_numeric() {
                chars.push(p);
                self.next_char();
            } else {
                break;
            }
        }

        let is_all_number = chars.clone().into_iter().all(|c| c.is_numeric());

        if is_all_number && !chars.is_empty() {
            let num_str = String::from_iter(chars.into_iter());
            Some(TokenType::NUMBER(num_str.parse::<usize>().unwrap()))
        } else {
            None
        }
    }

    pub fn next_token(&mut self) -> TokenType {
        self.skip_white_space();

        if self.is_end().unwrap() {
            return TokenType::EOF;
        }

        if let Some((_, c)) = self.next_char() {
            // println!("102 {:?}", c);

            match c {
                '[' => TokenType::LEFT_BRACE,
                ']' => TokenType::RIGHT_BRACE,
                '{' => TokenType::LEFT_CURLY_BRACE,
                '}' => TokenType::RIGHT_CURLY_BRACE,
                '(' => TokenType::LEFT_PAREN,
                ')' => TokenType::RIGHT_PAREN,
                ',' => TokenType::COMMA,
                ';' => TokenType::SEMICOLON,
                '.' => TokenType::DOT,
                ':' => TokenType::COLON,
                '+' => TokenType::ADD,
                '-' => TokenType::MINUS,
                '*' => TokenType::MULTIPLY,
                '/' => TokenType::DIVIDE,
                '!' => {
                    // reference whole tuple
                    let &(_, eq) = self.peek_char().unwrap();

                    if eq == '=' {
                        self.next_char();
                        TokenType::BANG_EQUAL
                    } else {
                        TokenType::BANG
                    }
                }
                '=' => {
                    let (_, eq) = self.peek_char().unwrap();

                    // or dereference eq
                    if *eq == '=' {
                        self.next_char();
                        TokenType::EQUAL_EQUAL
                    } else {
                        TokenType::ASSIGN
                    }
                }
                '>' => {
                    let &(_, eq) = self.peek_char().unwrap();

                    if eq == '=' {
                        self.next_char();
                        TokenType::GREATER_EQUAL
                    } else {
                        TokenType::GREATER
                    }
                }
                '<' => {
                    let &(_, eq) = self.peek_char().unwrap();

                    if eq == '=' {
                        self.next_char();
                        TokenType::LESS_EQUAL
                    } else {
                        TokenType::LESS
                    }
                }
                _ => {
                    let numbers = self.read_number(c);
                    let strings = self.read_string(c);

                    if numbers.is_some() {
                        numbers.unwrap()
                    } else if strings.is_some() {
                        strings.unwrap()
                    } else {
                        self.read_keyword(c)
                    }
                }
            }
        } else {
            // TokenType::NONE
            self.next_token()
        }
    }
}

#[cfg(test)]
mod unit_test {
    use crate::lexer::Lexer;
    use crate::token::TokenType;

    fn get_tokens(input: &str) -> Vec<TokenType> {
        let mut tokens = vec![];
        let mut lexer = Lexer::new(input);

        while let Some(is_end) = lexer.is_end() {
            if !is_end {
                let token = lexer.next_token();

                // println!("{:?}", token);
                tokens.push(token);
            } else {
                break;
            }
        }

        tokens.pop(); // remove EOF

        tokens
    }

    #[test]
    fn tokenize_basic_data_types() {
        let expected = vec![
            TokenType::LET,
            TokenType::IDENTIFIER(String::from("version")),
            TokenType::ASSIGN,
            TokenType::NUMBER(1234),
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENTIFIER(String::from("name")),
            TokenType::ASSIGN,
            TokenType::STRING(String::from("Lynx programming language")),
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENTIFIER(String::from("is_cool")),
            TokenType::ASSIGN,
            TokenType::TRUE,
            TokenType::SEMICOLON,
        ];
        let parsed = get_tokens(
            r#"
            let version = 1234;
            let name = "Lynx programming language";
            let is_cool = true;
        "#,
        );

        assert_eq!(expected, parsed);
    }

    #[test]
    fn tokenize_hash_type() {
        let parsed = get_tokens(
            r#"
                    let hash = {
                        "key": "foo",
                        "value": "bar",
                        1: ["arbitrary value"],
                        2: {
                        "child": ["arbitrary value"],
                        },
                    };
            "#,
        );

        let expected = vec![
            TokenType::LET,
            TokenType::IDENTIFIER(String::from("hash")),
            TokenType::ASSIGN,
            TokenType::LEFT_CURLY_BRACE,
            TokenType::STRING(String::from("key")),
            TokenType::COLON,
            TokenType::STRING(String::from("foo")),
            TokenType::COMMA,
            TokenType::STRING(String::from("value")),
            TokenType::COLON,
            TokenType::STRING(String::from("bar")),
            TokenType::COMMA,
            TokenType::NUMBER(1),
            TokenType::COLON,
            TokenType::LEFT_BRACE,
            TokenType::STRING(String::from("arbitrary value")),
            TokenType::RIGHT_BRACE,
            TokenType::COMMA,
            TokenType::NUMBER(2),
            TokenType::COLON,
            TokenType::LEFT_CURLY_BRACE,
            TokenType::STRING(String::from("child")),
            TokenType::COLON,
            TokenType::LEFT_BRACE,
            TokenType::STRING(String::from("arbitrary value")),
            TokenType::RIGHT_BRACE,
            TokenType::COMMA,
            TokenType::RIGHT_CURLY_BRACE,
            TokenType::COMMA,
            TokenType::RIGHT_CURLY_BRACE,
            TokenType::SEMICOLON,
        ];

        assert_eq!(expected, parsed);
    }

    #[test]
    fn tokenize_arr_type() {
        let parsed = get_tokens(
            r#"
            let arr = [1, 22, 3];
            let people = [{"name": "Anna", "age": 24}];
            let arr_with_values = [11 + 11, 22 * 2, 3];
        "#,
        );
        let expected = vec![
            TokenType::LET,
            TokenType::IDENTIFIER(String::from("arr")),
            TokenType::ASSIGN,
            TokenType::LEFT_BRACE,
            TokenType::NUMBER(1),
            TokenType::COMMA,
            TokenType::NUMBER(22),
            TokenType::COMMA,
            TokenType::NUMBER(3),
            TokenType::RIGHT_BRACE,
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENTIFIER(String::from("people")),
            TokenType::ASSIGN,
            TokenType::LEFT_BRACE,
            TokenType::LEFT_CURLY_BRACE,
            TokenType::STRING(String::from("name")),
            TokenType::COLON,
            TokenType::STRING(String::from("Anna")),
            TokenType::COMMA,
            TokenType::STRING(String::from("age")),
            TokenType::COLON,
            TokenType::NUMBER(24),
            TokenType::RIGHT_CURLY_BRACE,
            TokenType::RIGHT_BRACE,
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENTIFIER(String::from("arr_with_values")),
            TokenType::ASSIGN,
            TokenType::LEFT_BRACE,
            TokenType::NUMBER(11),
            TokenType::ADD,
            TokenType::NUMBER(11),
            TokenType::COMMA,
            TokenType::NUMBER(22),
            TokenType::MULTIPLY,
            TokenType::NUMBER(2),
            TokenType::COMMA,
            TokenType::NUMBER(3),
            TokenType::RIGHT_BRACE,
            TokenType::SEMICOLON,
        ];

        assert_eq!(expected, parsed);
    }

    #[test]
    fn tokenize_fn() {
        let parsed = get_tokens(
            r#"
            let return_stuff = fn(foo) {
                return foo;
              };
              
            return_stuff("Bar");
        "#,
        );

        let expected = vec![
            TokenType::LET,
            TokenType::IDENTIFIER(String::from("return_stuff")),
            TokenType::ASSIGN,
            TokenType::FN,
            TokenType::LEFT_PAREN,
            TokenType::IDENTIFIER(String::from("foo")),
            TokenType::RIGHT_PAREN,
            TokenType::LEFT_CURLY_BRACE,
            TokenType::RETURN,
            TokenType::IDENTIFIER(String::from("foo")),
            TokenType::SEMICOLON,
            TokenType::RIGHT_CURLY_BRACE,
            TokenType::SEMICOLON,
            TokenType::IDENTIFIER(String::from("return_stuff")),
            TokenType::LEFT_PAREN,
            TokenType::STRING(String::from("Bar")),
            TokenType::RIGHT_PAREN,
            TokenType::SEMICOLON,
        ];

        assert_eq!(expected, parsed);
    }

    #[test]
    fn tokenize_arithmetic_operators() {
        let expected = vec![
            // TokenType::LET,
            // TokenType::IDENTIFIER(String::from("bar")),
            // TokenType::ASSIGN,
            TokenType::LEFT_PAREN,
            TokenType::IDENTIFIER(String::from("foo_123")),
            TokenType::ADD,
            TokenType::IDENTIFIER(String::from("bb")),
            TokenType::RIGHT_PAREN,
            TokenType::MULTIPLY,
            TokenType::LEFT_PAREN,
            TokenType::IDENTIFIER(String::from("ccc")),
            TokenType::MINUS,
            TokenType::IDENTIFIER(String::from("d")),
            TokenType::RIGHT_PAREN,
            TokenType::DIVIDE,
            TokenType::IDENTIFIER(String::from("e")),
            TokenType::SEMICOLON,
        ];
        let parsed = get_tokens(
            r#"
            (foo_123 + bb) * (ccc - d) / e;
        "#,
        );

        assert_eq!(expected, parsed);
    }

    #[test]
    fn tokenize_boolean_logical_operator() {
        let expected = vec![
            TokenType::BANG,
            TokenType::TRUE,
            TokenType::SEMICOLON,
            TokenType::BANG,
            TokenType::FALSE,
            TokenType::SEMICOLON,
        ];
        let parsed = get_tokens(
            r#"
            !true;
            !false;
        "#,
        );

        assert_eq!(expected, parsed);
    }

    #[test]
    fn tokenize_unary_operators() {
        let expected = vec![
            TokenType::ADD,
            TokenType::NUMBER(10),
            TokenType::SEMICOLON,
            TokenType::MINUS,
            TokenType::NUMBER(5),
            TokenType::SEMICOLON,
            TokenType::STRING(String::from("Foo")),
            TokenType::ADD,
            TokenType::STRING(String::from(" ")),
            TokenType::ADD,
            TokenType::STRING(String::from("Bar")),
            TokenType::SEMICOLON,
        ];

        let parsed = get_tokens(
            r#"
            +10;
            -5;
            "Foo" + " " + "Bar";
        "#,
        );

        assert_eq!(expected, parsed);
    }

    #[test]
    fn tokenize_comparison_operator() {
        let expected = vec![
            TokenType::NUMBER(2),
            TokenType::GREATER,
            TokenType::NUMBER(5),
            TokenType::SEMICOLON,
            TokenType::NUMBER(1),
            TokenType::GREATER_EQUAL,
            TokenType::NUMBER(3),
            TokenType::SEMICOLON,
            TokenType::NUMBER(6),
            TokenType::LESS,
            TokenType::NUMBER(10),
            TokenType::SEMICOLON,
            TokenType::NUMBER(8),
            TokenType::LESS_EQUAL,
            TokenType::NUMBER(9),
            TokenType::SEMICOLON,
            TokenType::NUMBER(9),
            TokenType::BANG_EQUAL,
            TokenType::NUMBER(9),
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENTIFIER(String::from("isEqual")),
            TokenType::ASSIGN,
            TokenType::NUMBER(6),
            TokenType::EQUAL_EQUAL,
            TokenType::NUMBER(7),
            TokenType::SEMICOLON,
        ];

        let parsed = get_tokens(
            r#"
            2 > 5;
            1 >= 3;
            6 < 10;
            8 <= 9;
            9 != 9;
            let isEqual = 6 == 7;
        "#,
        );

        assert_eq!(expected, parsed);
    }

    #[test]
    fn tokenize_foc_if() {
        let parsed = get_tokens(
            r#"
            if (foo_123) {
                return 10;
              } else {
                return 5;
              }
        "#,
        );

        let expected = vec![
            TokenType::IF,
            TokenType::LEFT_PAREN,
            TokenType::IDENTIFIER(String::from("foo_123")),
            TokenType::RIGHT_PAREN,
            TokenType::LEFT_CURLY_BRACE,
            TokenType::RETURN,
            TokenType::NUMBER(10),
            TokenType::SEMICOLON,
            TokenType::RIGHT_CURLY_BRACE,
            TokenType::ELSE,
            TokenType::LEFT_CURLY_BRACE,
            TokenType::RETURN,
            TokenType::NUMBER(5),
            TokenType::SEMICOLON,
            TokenType::RIGHT_CURLY_BRACE,
        ];

        println!("{:?}", parsed);

        assert_eq!(expected, parsed);
    }

    #[test]
    fn tokenize_foc_while() {
        let parsed = get_tokens(
            r#"
            while (true) {
                print("looping...");
              }
        "#,
        );
        let expected = vec![
            TokenType::WHILE,
            TokenType::LEFT_PAREN,
            TokenType::TRUE,
            TokenType::RIGHT_PAREN,
            TokenType::LEFT_CURLY_BRACE,
            TokenType::IDENTIFIER(String::from("print")),
            TokenType::LEFT_PAREN,
            TokenType::STRING(String::from("looping...")),
            TokenType::RIGHT_PAREN,
            TokenType::SEMICOLON,
            TokenType::RIGHT_CURLY_BRACE,
        ];

        assert_eq!(expected, parsed);
    }

    #[test]
    fn tokenize_return() {
        let parsed = get_tokens(
            r#"
            if (true) {
                return;
              }
        "#,
        );
        let expected = vec![
            TokenType::IF,
            TokenType::LEFT_PAREN,
            TokenType::TRUE,
            TokenType::RIGHT_PAREN,
            TokenType::LEFT_CURLY_BRACE,
            TokenType::RETURN,
            TokenType::SEMICOLON,
            TokenType::RIGHT_CURLY_BRACE,
        ];

        assert_eq!(expected, parsed);
    }

    #[test]
    fn tokenize_build_in_function() {
        let parsed = get_tokens(
            r#"
            print("Hello");
            len("Lynx");
            len([0, 1, 2]);
            first([0, 1, 2]);
            last([0, 1, 2]);
            rest([0, 1, 2]);
            push([0, 1], 2);
        "#,
        );
        let expected = vec![
            TokenType::IDENTIFIER(String::from("print")),
            TokenType::LEFT_PAREN,
            TokenType::STRING(String::from("Hello")),
            TokenType::RIGHT_PAREN,
            TokenType::SEMICOLON,
            TokenType::IDENTIFIER(String::from("len")),
            TokenType::LEFT_PAREN,
            TokenType::STRING(String::from("Lynx")),
            TokenType::RIGHT_PAREN,
            TokenType::SEMICOLON,
            TokenType::IDENTIFIER(String::from("len")),
            TokenType::LEFT_PAREN,
            TokenType::LEFT_BRACE,
            TokenType::NUMBER(0),
            TokenType::COMMA,
            TokenType::NUMBER(1),
            TokenType::COMMA,
            TokenType::NUMBER(2),
            TokenType::RIGHT_BRACE,
            TokenType::RIGHT_PAREN,
            TokenType::SEMICOLON,
            TokenType::IDENTIFIER(String::from("first")),
            TokenType::LEFT_PAREN,
            TokenType::LEFT_BRACE,
            TokenType::NUMBER(0),
            TokenType::COMMA,
            TokenType::NUMBER(1),
            TokenType::COMMA,
            TokenType::NUMBER(2),
            TokenType::RIGHT_BRACE,
            TokenType::RIGHT_PAREN,
            TokenType::SEMICOLON,
            TokenType::IDENTIFIER(String::from("last")),
            TokenType::LEFT_PAREN,
            TokenType::LEFT_BRACE,
            TokenType::NUMBER(0),
            TokenType::COMMA,
            TokenType::NUMBER(1),
            TokenType::COMMA,
            TokenType::NUMBER(2),
            TokenType::RIGHT_BRACE,
            TokenType::RIGHT_PAREN,
            TokenType::SEMICOLON,
            TokenType::IDENTIFIER(String::from("rest")),
            TokenType::LEFT_PAREN,
            TokenType::LEFT_BRACE,
            TokenType::NUMBER(0),
            TokenType::COMMA,
            TokenType::NUMBER(1),
            TokenType::COMMA,
            TokenType::NUMBER(2),
            TokenType::RIGHT_BRACE,
            TokenType::RIGHT_PAREN,
            TokenType::SEMICOLON,
            TokenType::IDENTIFIER(String::from("push")),
            TokenType::LEFT_PAREN,
            TokenType::LEFT_BRACE,
            TokenType::NUMBER(0),
            TokenType::COMMA,
            TokenType::NUMBER(1),
            TokenType::RIGHT_BRACE,
            TokenType::COMMA,
            TokenType::NUMBER(2),
            TokenType::RIGHT_PAREN,
            TokenType::SEMICOLON,
        ];

        assert_eq!(expected, parsed);
    }

    #[test]
    fn tokenize_fibonacci() {
        let input = get_tokens(
            r#"
            fn fibonacci(x) {
                if (x == 0) {
                    return 0;
                } else {
                    if (x == 1) {
                        return 1;
                    } else {
                        return fibonacci(x - 1) + fibonacci(x - 2);
                    }
                }
            }
            let foo = 123;
            let fib_val = fibonacci(foo);
            print(fib_val);
        "#,
        );
        println!("{:?}", input);
    }

    #[test]
    fn tokenize_program() {
        let parsed = get_tokens(
            r#"
            let foo = 123;
            let fibonacci = fn(x) {
                if (x == 0) {
                  0
                } else {
                  if (x == 1) {
                    return 1;
                  } else {
                    fibonacci(x - 1) + fibonacci(x - 2);
                  }
                }
            };
            let fib_val = fibonacci(foo);
            print(fib_val);
        "#,
        );
        let expected = vec![
            TokenType::LET,
            TokenType::IDENTIFIER(String::from("foo")),
            TokenType::ASSIGN,
            TokenType::NUMBER(123),
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENTIFIER(String::from("fibonacci")),
            TokenType::ASSIGN,
            TokenType::FN,
            TokenType::LEFT_PAREN,
            TokenType::IDENTIFIER(String::from("x")),
            TokenType::RIGHT_PAREN,
            TokenType::LEFT_CURLY_BRACE,
            TokenType::IF,
            TokenType::LEFT_PAREN,
            TokenType::IDENTIFIER(String::from("x")),
            TokenType::EQUAL_EQUAL,
            TokenType::NUMBER(0),
            TokenType::RIGHT_PAREN,
            TokenType::LEFT_CURLY_BRACE,
            TokenType::NUMBER(0),
            TokenType::RIGHT_CURLY_BRACE,
            TokenType::ELSE,
            TokenType::LEFT_CURLY_BRACE,
            TokenType::IF,
            TokenType::LEFT_PAREN,
            TokenType::IDENTIFIER(String::from("x")),
            TokenType::EQUAL_EQUAL,
            TokenType::NUMBER(1),
            TokenType::RIGHT_PAREN,
            TokenType::LEFT_CURLY_BRACE,
            TokenType::RETURN,
            TokenType::NUMBER(1),
            TokenType::SEMICOLON,
            TokenType::RIGHT_CURLY_BRACE,
            TokenType::ELSE,
            TokenType::LEFT_CURLY_BRACE,
            TokenType::IDENTIFIER(String::from("fibonacci")),
            TokenType::LEFT_PAREN,
            TokenType::IDENTIFIER(String::from("x")),
            TokenType::MINUS,
            TokenType::NUMBER(1),
            TokenType::RIGHT_PAREN,
            TokenType::ADD,
            TokenType::IDENTIFIER(String::from("fibonacci")),
            TokenType::LEFT_PAREN,
            TokenType::IDENTIFIER(String::from("x")),
            TokenType::MINUS,
            TokenType::NUMBER(2),
            TokenType::RIGHT_PAREN,
            TokenType::SEMICOLON,
            TokenType::RIGHT_CURLY_BRACE,
            TokenType::RIGHT_CURLY_BRACE,
            TokenType::RIGHT_CURLY_BRACE,
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENTIFIER(String::from("fib_val")),
            TokenType::ASSIGN,
            TokenType::IDENTIFIER(String::from("fibonacci")),
            TokenType::LEFT_PAREN,
            TokenType::IDENTIFIER(String::from("foo")),
            TokenType::RIGHT_PAREN,
            TokenType::SEMICOLON,
            TokenType::IDENTIFIER(String::from("print")),
            TokenType::LEFT_PAREN,
            TokenType::IDENTIFIER(String::from("fib_val")),
            TokenType::RIGHT_PAREN,
            TokenType::SEMICOLON,
        ];

        assert_eq!(expected, parsed);
    }
}
