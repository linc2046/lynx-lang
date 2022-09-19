/*!
Lynx is an interpreter for lynx language specs derived from the noted `The Monkey Language` and the book ``.
It is an educational crate aimed to learn how programming languages are made. Also, it is made by rookie rustaceans like me.
The code in this library includes lots of comments as well as test cases.
The whole idea is from the notable book `The Monkey Language`. I also have the idea to extend it when I am available on
this project again.

In general, an interpreter consists of three major parts, as divided in separate file in `src` folder.

A Tokenizer will tokenize input text or stream into tokens.

Sometimes, a simple numeric can be a `Number` token;

```shell
1 // Token::Number
```

## lexer

```shell
std::iter::FromIterator;
std::iter::Peekable;
std::str::CharIndices;
```

- [lexing by incrementing vec index](https://github.com/mohitk05/monkey-rust/blob/master/src/lexer/mod.rs)
- [lexing by iterating](https://github.com/JayKickliter/monkey/blob/324172cb0afefcbe9e3b3e6a4e10b5b9bd7acf26/src/lexer/mod.rs)
- [nom-based lexing](https://github.com/Rydgel/monkey-rust/blob/master/lib/lexer/mod.rs)
https://blog.wadackel.me/2018/rs-monkey-lang/

## parser

- pratt parsing

```shell
std::slice::Chunks
std::ops::Deref;
```

## evaluator

```text
// let foo1 = 2;

// outer = Env {
//     store: {
//         print: fn (),
//         foo1: 2
//     },
//     outer: None
// }

// fn call_fn(foo) {
// let bar = 1;
// return foo + bar;
// }

// call_fn(foo1)

// ScopedEnv {
//     store: {
//         bar: 1
//     },
//     outer: outer
// }
```


## repl

- <https://arzg.github.io/lang/7/>

*/

#![allow(unused_must_use)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_comparisons)]
#![feature(array_windows)]
#![feature(exact_size_is_empty)]

pub mod ast;
pub mod builtin;
pub mod env;
pub mod evaluator;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod token;
pub mod util;
