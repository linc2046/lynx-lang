#![allow(clippy::upper_case_acronyms)]

// https://quizlet.com/157448303/characters-and-punctuation-marks-in-programming-language-flash-cards/

use std::fmt;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum TokenType {
    // None
    NONE,

    // Single-character tokens.
    LEFT_PAREN,        // (
    RIGHT_PAREN,       // )
    LEFT_CURLY_BRACE,  // {
    RIGHT_CURLY_BRACE, // }
    LEFT_BRACE,        // [
    RIGHT_BRACE,       // ]
    COMMA,             // ,
    DOT,               // .
    COLON,             // :
    MINUS,             // -
    ADD,               // +
    SEMICOLON,         // ;
    DIVIDE,            // /
    MULTIPLY,          // *

    // One or two character tokens.
    BANG,          // !
    BANG_EQUAL,    // !=
    ASSIGN,        // =
    EQUAL_EQUAL,   // ==
    GREATER,       // >
    GREATER_EQUAL, // >=
    LESS,          // <
    LESS_EQUAL,    // <=

    // Literals
    IDENTIFIER(String),
    STRING(String),
    NUMBER(usize),

    // Keywords.
    TRUE,
    FALSE,
    LET,
    FN,
    IF,
    ELSE,
    WHILE,
    BREAK,
    RETURN,

    // Build in Functions
    // LEN,
    // FIRST,
    // LAST,
    // REST,
    // PUSH,
    // UNSHIFT,
    // PRINT,

    // End of File
    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IDENTIFIER(name) => {
                write!(f, "identifier {}", name)
            }
            Self::ADD => {
                write!(f, "+")
            }
            Self::MINUS => {
                write!(f, "-")
            }
            Self::DIVIDE => {
                write!(f, "/")
            }
            Self::MULTIPLY => {
                write!(f, "*")
            }
            _ => {
                write!(f, "NONE")
            }
        }
    }
}
