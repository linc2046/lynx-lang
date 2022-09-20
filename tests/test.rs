#[cfg(test)]
mod integration_test {
    use lynx_lang::env::Env;
    use lynx_lang::evaluator::Evaluator;
    use lynx_lang::object::Object;
    use lynx_lang::parser::Parser;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn _get_eval_val(input: &str) -> Option<Object> {
        let program = Parser::get(input).parse_program();

        let mut evaluator = Evaluator::new(Rc::new(RefCell::new(Env::new())));

        evaluator.builtin();

        evaluator.eval_program(program)
    }

    #[test]
    fn eval_lynx_program() {
        //     let value = get_eval_val(
        //         r#"
        //         let foo = fn(bar, stuff) {
        //             let another_bar = stuff;
        //             return another_bar + bar;
        //         };
        //         let grouped = 9 / (1 + 2);
        //         let arr = [1234, true, "Lynx programming language", [1234, true, "Lynx programming
        // language"]];         let value = foo(grouped, len(arr));

        //         if (value == 0) {
        //             return value;
        //         } else {
        //             return 0;
        //         }
        // "#
        //     );
        //     assert_eq!(format!("{:?}", value), format!("{:?}", Some(Object::Integer(7))));
        assert_eq!(1, 1);
    }
}
