use crate::object::Object;
use std::collections::HashMap;

pub fn make_builtin() -> HashMap<String, Object> {
    let mut builtin_map = HashMap::new();

    builtin_map.insert("len".to_string(), Object::Builtin(len));
    builtin_map.insert("first".to_string(), Object::Builtin(first));
    builtin_map.insert("last".to_string(), Object::Builtin(last));
    builtin_map.insert("rest".to_string(), Object::Builtin(rest));
    builtin_map.insert("push".to_string(), Object::Builtin(push));
    builtin_map.insert("unshift".to_string(), Object::Builtin(unshift));
    builtin_map.insert("print".to_string(), Object::Builtin(print));

    builtin_map
}

fn len(params: Vec<Object>) -> Object {
    match params.first() {
        Some(Object::Array(arr)) => Object::Integer(arr.len()),
        _ => Object::Null,
    }
}

fn first(params: Vec<Object>) -> Object {
    match params.first() {
        Some(Object::Array(arr)) => arr.first().unwrap_or(&Object::Null).clone(),
        _ => Object::Null,
    }
}

fn last(params: Vec<Object>) -> Object {
    match params.first() {
        Some(Object::Array(arr)) => arr.last().unwrap_or(&Object::Null).clone(),
        _ => Object::Null,
    }
}

fn rest(params: Vec<Object>) -> Object {
    match params.first() {
        Some(Object::Array(arr)) => match arr.split_first() {
            Some((_, elements)) => Object::Array(elements.to_vec()),
            None => Object::Null,
        },
        _ => Object::Null,
    }
}

fn push(params: Vec<Object>) -> Object {
    match params.first() {
        Some(Object::Array(arr)) => {
            let mut vec = vec![];
            match params.get(1) {
                Some(obj) => {
                    vec.extend(arr.clone().into_iter());
                    vec.push(obj.clone());

                    Object::Array(vec)
                }
                _ => Object::Array(arr.to_vec()),
            }
        }
        _ => Object::Null,
    }
}

fn unshift(params: Vec<Object>) -> Object {
    match params.first() {
        Some(Object::Array(arr)) => {
            let mut vec = vec![];
            match params.get(1) {
                Some(obj) => {
                    vec.push(obj.clone());
                    vec.extend(arr.clone().into_iter());

                    Object::Array(vec)
                }
                _ => Object::Array(arr.to_vec()),
            }
        }
        _ => Object::Null,
    }
}

fn print(params: Vec<Object>) -> Object {
    match params.first() {
        Some(obj) => {
            println!("{:?}", obj);
        }
        None => {
            println!("nothing for print");
        }
    }

    Object::Null
}
