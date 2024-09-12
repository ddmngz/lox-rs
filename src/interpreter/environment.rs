use crate::interpreter::error::RuntimeError;
use crate::interpreter::Result;
use crate::syntax_trees::lox_object::LoxObject;
use crate::token::SmartString;
use std::collections::HashMap;

use std::collections::hash_map::Entry;

/*
#[derive(Default)]
pub struct Environment{
    environment: HashMap<SmartString, Option<LoxObject>>,
    outer: Option<Box<Environment>>
}*/

#[derive(Clone)]
pub enum Environment {
    Global(HashMap<SmartString, Option<LoxObject>>),
    Local {
        environment: HashMap<SmartString, Option<LoxObject>>,
        outer: Box<Environment>,
    },
}

impl Default for Environment {
    fn default() -> Self {
        Self::Global(HashMap::new())
    }
}

impl Environment {
    pub fn new(outer: Environment) -> Self {
        Self::Local {
            environment: HashMap::new(),
            outer: Box::new(outer),
        }
    }

    pub fn get(&self, key: &str) -> Result<&Option<LoxObject>> {
        match self {
            Self::Global(environment) => environment
                .get(key)
                .ok_or_else(|| RuntimeError::Undefined(SmartString::from(key))),
            Self::Local { environment, outer } => match environment.get(key) {
                Some(value) => Ok(value),
                None => outer.get(key),
            },
        }
    }

    pub fn define(&mut self, key: SmartString, value: Option<LoxObject>) {
        let environment = self.get_environment_mut();
        environment.insert(key, value);
    }

    pub fn assign(&mut self, key: SmartString, value: LoxObject) -> Result<()> {
        let environment = self.get_environment_mut();

        if let Entry::Occupied(mut variable) = environment.entry(key.clone()) {
            *variable.get_mut() = Some(value);
            Ok(())
        } else {
            match self {
                Self::Global(_) => Err(RuntimeError::Undefined(key)),
                Self::Local { outer, .. } => outer.assign(key, value),
            }
        }
    }

    fn get_environment_mut(&mut self) -> &mut HashMap<SmartString, Option<LoxObject>> {
        match self {
            Self::Global(environment) => environment,
            Self::Local { environment, .. } => environment,
        }
    }
}
