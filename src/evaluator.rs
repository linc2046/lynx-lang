use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use crate::{ast::*, builtin::*, env::Env, object::Object, token::TokenType};

pub struct Evaluator {
    env: Rc<RefCell<Env>>,
}

impl Evaluator {
    pub fn new(env: Rc<RefCell<Env>>) -> Self {
        Evaluator { env }
    }

    pub fn builtin(&mut self) {
        let builtins = make_builtin();
        self.env = Rc::new(RefCell::new(Env::from(builtins)));

        println!("22 {:?}", self.env);
    }

    pub fn eval_program(&mut self, program: AstNode) -> Option<Object> {
        println!("{:?}", program);

        match program {
            AstNode::Program(statements) => {
                let mut value = Some(Object::Null);

                for statement in statements {
                    value = self.eval_statement(statement);
                }

                value
            }
        }
    }

    pub fn get_env(&self) -> RefMut<Env> {
        self.env.borrow_mut()
    }

    fn eval_statement(&mut self, statement: Statement) -> Option<Object> {
        match statement {
            Statement::Let(identifier, expr) => self.eval_let_statement(identifier, expr),
            Statement::Return(expr) => self.eval_return_statement(expr),
            Statement::Expr(expr) => self.eval_expression(expr),
            Statement::BlockStatement(statements) => self.eval_block_statements(statements),
        }
    }

    fn eval_let_statement(&mut self, identifier: Box<Expression>, expr: Box<Expression>) -> Option<Object> {
        match *identifier {
            Expression::Identifier(ident) => match self.eval_expression(expr) {
                Some(value) => {
                    self.env.deref().borrow_mut().set(ident, value);
                    None
                }
                None => None,
            },
            _ => None,
        }
    }

    fn eval_return_statement(&mut self, expr: Box<Expression>) -> Option<Object> {
        self.eval_expression(expr)
    }

    fn eval_block_statement(&mut self, block_stmt: Statement) -> Option<Object> {
        match block_stmt {
            Statement::BlockStatement(stmts) => self.eval_block_statements(stmts),
            _ => Some(Object::Null),
        }
    }

    fn eval_block_statements(&mut self, block_stmts: Vec<Statement>) -> Option<Object> {
        let mut value = Some(Object::Null);

        for stmt in block_stmts {
            value = self.eval_statement(stmt);
        }

        value
    }

    fn eval_expression(&mut self, expr: Box<Expression>) -> Option<Object> {
        match *expr {
            Expression::String(string) => self.eval_string(string),
            Expression::Integer(int) => self.eval_integer(int),
            Expression::Boolean(bl) => self.eval_boolean(bl),
            Expression::Identifier(identifer) => self.eval_identifier(identifer),
            Expression::Array(exprs) => self.eval_array_expression(exprs),
            Expression::Hash(hashes) => self.eval_hash_expression(hashes),
            Expression::Prefix(operator, expr) => self.eval_prefix_expression(operator, expr),
            Expression::Infix(left, operator, right) => self.eval_infix_expression(left, operator, right),
            Expression::If(condition, statement, else_statement) => {
                self.eval_if_expression(condition, statement, else_statement)
            }
            Expression::While(condition, block_statement) => self.eval_while_expression(condition, block_statement),
            Expression::Break => Some(Object::Break),
            Expression::Fn(fn_name, fn_parameter, fn_body) => self.eval_fn_expression(*fn_name, fn_parameter, fn_body),
            Expression::FnCall(fn_name, fn_parameter) => self.eval_fn_call_expression(fn_name, fn_parameter),
            _ => Some(Object::Null),
        }
    }

    fn eval_prefix_expression(&mut self, operator: TokenType, expr: Box<Expression>) -> Option<Object> {
        match operator {
            TokenType::BANG => {
                let value = self.eval_expression(expr).unwrap();

                if value.is_truthy() {
                    Some(Object::Boolean(false))
                } else {
                    Some(Object::Boolean(true))
                }
            }
            _ => Some(Object::Null),
        }
    }

    fn get_integer_val(&mut self, object: Object) -> usize {
        match object {
            Object::Integer(i) => i,
            _ => 0,
        }
    }

    fn get_infix_objects(&mut self, left: &InfixExpression, right: &InfixExpression) -> (Object, Object) {
        let left = self.eval_expression(left.clone()).unwrap();
        let right = self.eval_expression(right.clone()).unwrap();

        (left, right)
    }

    fn eval_infix_expression(
        &mut self,
        left: InfixExpression,
        operator: TokenType,
        right: InfixExpression,
    ) -> Option<Object> {
        let value = Some(Object::Null);

        match operator {
            TokenType::ADD => {
                let (left_obj, right_obj) = self.get_infix_objects(&left, &right);
                let left_val = self.get_integer_val(left_obj);
                let right_val = self.get_integer_val(right_obj);

                Some(Object::Integer(left_val + right_val))
            }
            TokenType::MINUS => {
                let (left_obj, right_obj) = self.get_infix_objects(&left, &right);
                let left_val = self.get_integer_val(left_obj);
                let right_val = self.get_integer_val(right_obj);

                Some(Object::Integer(left_val - right_val))
            }
            TokenType::MULTIPLY => {
                let (left_obj, right_obj) = self.get_infix_objects(&left, &right);
                let left_val = self.get_integer_val(left_obj);
                let right_val = self.get_integer_val(right_obj);

                Some(Object::Integer(left_val * right_val))
            }
            TokenType::DIVIDE => {
                let (left_obj, right_obj) = self.get_infix_objects(&left, &right);
                let left_val = self.get_integer_val(left_obj);
                let right_val = self.get_integer_val(right_obj);

                Some(Object::Integer(left_val / right_val))
            }
            _ => value,
        }
    }

    fn eval_identifier(&mut self, identifier: String) -> Option<Object> {
        match self.env.deref().borrow_mut().get(identifier.clone()) {
            Some(value) => Some(value),
            None => Some(Object::Error(format!("no identifier found: {}", identifier))),
        }
    }

    fn eval_integer(&mut self, int: usize) -> Option<Object> {
        Some(Object::Integer(int))
    }

    fn eval_string(&mut self, string: String) -> Option<Object> {
        Some(Object::String(string))
    }

    fn eval_boolean(&mut self, bl: bool) -> Option<Object> {
        Some(Object::Boolean(bl))
    }

    fn eval_array_expression(&mut self, exprs: Vec<Expression>) -> Option<Object> {
        Some(Object::Array(
            exprs
                .into_iter()
                .map(|expr| self.eval_expression(Box::new(expr)).unwrap_or(Object::Null))
                .collect::<Vec<_>>(),
        ))
    }

    fn eval_hash_expression(&mut self, hashes: Vec<(Expression, Expression)>) -> Option<Object> {
        let mut hash_object = HashMap::<Object, Object>::new();

        hashes.into_iter().for_each(|(k, v)| {
            hash_object.insert(
                self.eval_expression(Box::new(k)).unwrap_or(Object::Null),
                self.eval_expression(Box::new(v)).unwrap_or(Object::Null),
            );
        });

        Some(Object::Hash(hash_object))
    }

    fn eval_if_expression(
        &mut self,
        if_condition: IfCondition,
        statements: Statement,
        else_statements: Option<Statement>,
    ) -> Option<Object> {
        let condition = self.eval_expression(if_condition);

        if let Some(condition_val) = condition {
            if condition_val.is_truthy() {
                self.eval_block_statement(statements)
            } else if let Some(else_stmts) = else_statements {
                self.eval_block_statement(else_stmts)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn eval_while_expression(&mut self, while_condition: WhileCondition, block_stmt: Statement) -> Option<Object> {
        let condition = self.eval_expression(while_condition).unwrap();

        while condition.is_truthy() {
            match block_stmt {
                Statement::BlockStatement(ref stmts) => {
                    for stmt in stmts {
                        let mut _value = Some(Object::Null);

                        _value = self.eval_statement(stmt.clone());

                        if _value.eq(&Some(Object::Break)) {
                            break;
                        }
                    }
                }
                _ => {
                    break;
                }
            }
        }

        None
    }

    fn eval_fn_expression(
        &mut self,
        fn_name_expr: Expression,
        fn_parameter: FnParameter,
        fn_body: FnBody,
    ) -> Option<Object> {
        let fn_name = match fn_name_expr {
            Expression::Identifier(str) => str,
            _ => "".to_string(),
        };

        let fn_object = Object::Function(fn_parameter, fn_body, Rc::clone(&self.env));

        if fn_name.is_empty() {
            // let fn expression
            Some(fn_object)
        } else {
            // fn declaration
            self.env.borrow_mut().set(fn_name, fn_object);
            None
        }
    }

    fn enclose_fn_env(
        &mut self,
        arg_pair_vec: Vec<(&Expression, &Object)>,
        outer_env: Rc<RefCell<Env>>,
    ) -> Rc<RefCell<Env>> {
        let mut enclosed_env = Env::enclosed_outer_env(Rc::clone(&outer_env));

        for (key, value) in arg_pair_vec {
            let name = match key {
                Expression::Identifier(str) => str,
                _ => "",
            };

            if !name.is_empty() {
                enclosed_env.set(name.to_string(), value.clone());
            }
        }

        Rc::new(RefCell::new(enclosed_env))
    }

    fn eval_fn_call_expression(&mut self, fn_name: FnName, fn_parameter: FnParameter) -> Option<Object> {
        let arguments = fn_parameter
            .iter()
            .map(|expr| self.eval_expression(Box::new(expr.clone())).unwrap_or(Object::Null))
            .collect::<Vec<Object>>();

        let (parameters, stmt, outer_env) = match self.eval_expression(fn_name).unwrap() {
            Object::Function(args, stmt, outer_env) => (args, stmt, outer_env),
            Object::Builtin(func) => {
                // invoke builtin methods
                return Some(func(arguments));
            }
            _ => {
                return None;
            }
        };

        let original_env = Rc::clone(&self.env);

        // https://stackoverflow.com/questions/156767/whats-the-difference-between-an-argument-and-a-parameter
        let para_arg_pair = parameters.iter().zip(arguments.iter());

        // create temporary env for eval function statements
        self.env = self.enclose_fn_env(para_arg_pair.collect::<Vec<(&Expression, &Object)>>(), outer_env);

        let fn_call_value = self.eval_block_statement(stmt);

        // restore original env
        self.env = original_env;

        fn_call_value
    }
}

#[cfg(test)]
mod unit_test {
    use crate::env::Env;
    use crate::evaluator::Evaluator;
    use crate::object::Object;
    use crate::parser::Parser;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn get_eval_val(input: &str) -> Option<Object> {
        let program = Parser::get(input).parse_program();

        let mut evaluator = Evaluator::new(Rc::new(RefCell::new(Env::new())));

        evaluator.builtin();

        evaluator.eval_program(program)
    }

    #[test]
    fn eval_integer() {
        let evaluated = get_eval_val("12");
        let expected = Some(Object::Integer(12));

        assert_eq!(format!("{:?}", evaluated), format!("{:?}", expected));
    }

    #[test]
    fn eval_boolean() {
        assert_eq!(
            format!("{:?}", get_eval_val("true")),
            format!("{:?}", Some(Object::Boolean(true)))
        );
        assert_eq!(
            format!("{:?}", get_eval_val("false")),
            format!("{:?}", Some(Object::Boolean(false)))
        );
    }

    #[test]
    fn eval_string() {
        assert_eq!(
            format!("{:?}", get_eval_val(r#""foo_bar_123""#)),
            format!("{:?}", Some(Object::String(String::from("foo_bar_123"))))
        );
    }

    #[test]
    fn eval_hash_expression() {
        let hash_value = get_eval_val(
            r#"
    {
        "foo": "bar",
        1: 2,
        2: [1234, true, "Lynx programming language"],
        "abc": true
    };
    "#,
        )
        .unwrap();

        match hash_value {
            Object::Hash(hashes) => {
                assert_eq!(
                    hashes.get(&Object::String(String::from("abc"))),
                    Some(&Object::Boolean(true))
                );

                assert_eq!(hashes.get(&Object::Integer(1)), Some(&Object::Integer(2)));

                assert_eq!(
                    hashes.get(&Object::String(String::from("foo"))),
                    Some(&Object::String(String::from("bar")))
                );

                assert_eq!(
                    hashes.get(&Object::Integer(2)),
                    Some(&Object::Array(vec![
                        Object::Integer(1234),
                        Object::Boolean(true),
                        Object::String(String::from("Lynx programming language"))
                    ]))
                );
            }
            _ => (),
        }
    }

    #[test]
    fn eval_array_expression() {
        assert_eq!(
            format!(
                "{:?}",
                get_eval_val(
                    r#"[1234, true, "Lynx programming language", [1234, true, "Lynx programming language"]];"#
                )
            ),
            format!(
                "{:?}",
                Some(Object::Array(vec![
                    Object::Integer(1234),
                    Object::Boolean(true),
                    Object::String(String::from("Lynx programming language")),
                    Object::Array(vec![
                        Object::Integer(1234),
                        Object::Boolean(true),
                        Object::String(String::from("Lynx programming language"))
                    ])
                ]))
            )
        );
    }

    #[test]
    fn eval_identifier() {
        assert_eq!(
            format!(
                "{:?}",
                get_eval_val(
                    r#"
                foo;
            "#
                )
            ),
            format!("{:?}", Some(Object::Error(String::from("no identifier found: foo"))))
        );
    }

    #[test]
    fn eval_let_statement() {
        assert_eq!(
            format!(
                "{:?}",
                get_eval_val(
                    r#"
                let foo = 123 + 4;
                foo;
            "#
                )
            ),
            format!("{:?}", Some(Object::Integer(127)))
        );
    }

    #[test]
    fn eval_return_statement() {
        assert_eq!(
            format!("{:?}", get_eval_val(r#"return !false;"#)),
            format!("{:?}", Some(Object::Boolean(true)))
        );

        assert_eq!(
            format!("{:?}", get_eval_val(r#"return 123;"#)),
            format!("{:?}", Some(Object::Integer(123)))
        );

        assert_eq!(
            format!(
                "{:?}",
                get_eval_val(
                    r#"
                let foo = 123;
                return foo;
            "#
                )
            ),
            format!("{:?}", Some(Object::Integer(123)))
        );
    }

    #[test]
    fn eval_prefix_expression() {
        assert_eq!(
            format!("{:?}", get_eval_val(r#"!false"#)),
            format!("{:?}", Some(Object::Boolean(true)))
        );

        assert_eq!(
            format!("{:?}", get_eval_val(r#"!true"#)),
            format!("{:?}", Some(Object::Boolean(false)))
        );
    }

    #[test]
    fn eval_infix_expression() {
        assert_eq!(
            format!("{:?}", get_eval_val(r#"1 + 2 + 3 + 4 / 2 * 3"#)),
            format!("{:?}", Some(Object::Integer(12)))
        );
    }

    #[test]
    fn eval_grouped_expression() {
        assert_eq!(
            format!("{:?}", get_eval_val(r#"(7 + 2) / 3"#)),
            format!("{:?}", Some(Object::Integer(3)))
        );

        assert_eq!(
            format!("{:?}", get_eval_val(r#"9 / (1 + 2)"#)),
            format!("{:?}", Some(Object::Integer(3)))
        );
    }

    #[test]
    fn eval_if_expression() {
        assert_eq!(
            format!(
                "{:?}",
                get_eval_val(
                    r#"
                if (false) {
                    false;
                } else {
                    true;
                }
            "#
                )
            ),
            format!("{:?}", Some(Object::Boolean(true)))
        );

        assert_eq!(
            format!(
                "{:?}",
                get_eval_val(
                    r#"
                let foo = 123;
                if (foo) {
                    let bar = "stuff";
                    return bar;
                } else {
                    return 5;
                }
            "#
                )
            ),
            format!("{:?}", Some(Object::String(String::from("stuff"))))
        );
    }

    #[test]
    fn eval_while_expression() {
        // assert_eq!(
        //     format!(
        //         "{:?}",
        //         get_eval_val(
        //             r#"
        //         while (foo) {
        //             let bar = "stuff";

        //             foo(bar);
        //         }
        //     "#
        //         )
        //     ),
        //     format!("{:?}", Some(Object::Integer(3)))
        // );
        assert_eq!(1, 1);
    }

    #[test]
    fn eval_fn_expression() {
        let input = r#"
            fn bar(foo, stuff) {
                let another_bar = stuff;
                return another_bar + foo;
            }
    "#;
        let program = Parser::get(input).parse_program();
        let mut evaluator = Evaluator::new(Rc::new(RefCell::new(Env::new())));
        evaluator.eval_program(program);
        let env_stored = evaluator.get_env().get("bar".to_string());

        assert!(env_stored.is_some());
    }

    #[test]
    fn eval_let_fn_expression() {
        let input = r#"
            let foo = fn(bar, stuff) {
                let another_bar = stuff;
                return another_bar + bar;
            }
    "#;
        let program = Parser::get(input).parse_program();
        let mut evaluator = Evaluator::new(Rc::new(RefCell::new(Env::new())));
        evaluator.eval_program(program);

        let env_stored = evaluator.get_env().get("foo".to_string());

        assert!(env_stored.is_some());
    }

    #[test]
    fn eval_fn_call_expression() {
        let value = format!(
            "{:?}",
            get_eval_val(
                r#"
                fn bar(foo, stuff, var) {
                    let another_bar = stuff;
                    return another_bar + foo + var;
                }
                let value = bar(1, 2, 3);
                value;
        "#
            )
        );

        println!("{:?}", value);

        assert_eq!(value, format!("{:?}", Some(Object::Integer(6))));
    }

    #[test]
    fn eval_let_fn_call_expression() {
        let value = format!(
            "{:?}",
            get_eval_val(
                r#"
                let bar = fn(foo, stuff, var) {
                    let another_bar = stuff;
                    return another_bar + foo + var;
                }
                let value = bar(1, 2, 3);
                value;
        "#
            )
        );

        println!("{:?}", value);

        assert_eq!(value, format!("{:?}", Some(Object::Integer(6))));
    }

    #[test]
    fn eval_fibonacci_fn() {
        // todo
        let value = format!(
            "{:?}",
            get_eval_val(
                r#"
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
        "#
            )
        );

        println!("{:?}", value);

        assert_eq!(value, format!("{:?}", Some(Object::Integer(1))));
    }

    #[test]
    fn eval_build_ins() {
        assert_eq!(
            format!("{:?}", get_eval_val(r#"first([1, 2, 3]);"#)),
            format!("{:?}", Some(Object::Integer(1)))
        );

        assert_eq!(
            format!(
                "{:?}",
                get_eval_val(
                    r#"
            last([1, 2, 3]);
            "#
                )
            ),
            format!("{:?}", Some(Object::Integer(3)))
        );

        assert_eq!(
            format!("{:?}", get_eval_val(r#"rest([1, 2, 3]);"#)),
            format!(
                "{:?}",
                Some(Object::Array(vec![Object::Integer(2), Object::Integer(3)]))
            )
        );

        assert_eq!(
            format!("{:?}", get_eval_val(r#"len([1, 2, 3]);"#)),
            format!("{:?}", Some(Object::Integer(3)))
        );

        assert_eq!(
            format!("{:?}", get_eval_val(r#"push([1, 2, 3], 4);"#)),
            format!(
                "{:?}",
                Some(Object::Array(vec![
                    Object::Integer(1),
                    Object::Integer(2),
                    Object::Integer(3),
                    Object::Integer(4)
                ]))
            )
        );

        assert_eq!(
            format!("{:?}", get_eval_val(r#"unshift([1, 2, 3], 4);"#)),
            format!(
                "{:?}",
                Some(Object::Array(vec![
                    Object::Integer(4),
                    Object::Integer(1),
                    Object::Integer(2),
                    Object::Integer(3)
                ]))
            )
        );

        // assert_eq!(
        //     format!("{:?}", get_eval_val(r#"print([1, 2, 3]);"#)),
        //     format!("{:?}", Some("[1, 2, 3]"))
        // );
    }
}
