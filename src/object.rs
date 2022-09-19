#![warn(clippy::derive_hash_xor_eq)]

use crate::ast::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

use crate::env::Env;

// https://stackoverflow.com/questions/64298245/in-rust-what-is-fn
pub type FuncType = fn(Vec<Object>) -> Object;

#[derive(PartialEq, Debug, Clone)]
pub enum Object {
    Integer(usize),
    Boolean(bool),
    String(String),
    Array(Vec<Object>),
    Hash(HashMap<Object, Object>),
    Null,
    ReturnValue(Rc<Object>),
    Function(Vec<Expression>, Statement, Rc<RefCell<Env>>),
    Builtin(FuncType),
    Break,
    Error(String),
}

impl Eq for Object {}

// https://doc.rust-lang.org/std/hash/trait.Hash.html
impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Object::Integer(int) => int.hash(state),
            Object::String(str) => str.hash(state),
            Object::Boolean(bl) => bl.hash(state),
            _ => "".hash(state),
        }
    }
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match *self {
            Object::Boolean(bl) => bl,
            Object::Null => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Builtin {
    Len,
    First,
    Last,
    Rest,
    Push,
    UnShift,
    Print,
}
