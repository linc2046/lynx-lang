use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::object::Object;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Env {
    store: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn get(&mut self, key: String) -> Option<Object> {
        match self.store.get(&key) {
            Some(value) => Some(value.clone()),
            None => match &self.outer {
                Some(outer) => outer.borrow_mut().get(key),
                None => None,
            },
        }
    }

    pub fn set(&mut self, key: String, value: Object) {
        self.store.insert(key, value);
    }

    pub fn enclosed_outer_env(outer: Rc<RefCell<Env>>) -> Self {
        Env {
            store: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn from(builtin: HashMap<String, Object>) -> Self {
        Env {
            store: builtin,
            outer: None,
        }
    }
}
