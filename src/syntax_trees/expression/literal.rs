use super::Visitor;
use crate::syntax_trees::lox_object::LoxObject;
#[derive(Clone, Debug)]
pub struct Literal {
    pub value: LoxObject,
}

impl Visitor<String> for Literal {
    fn accept(&self) -> String {
        self.value.to_string()
    }
}

impl Visitor<LoxObject> for Literal {
    fn accept(&self) -> LoxObject {
        self.value.clone()
    }
}

impl Literal {
    pub fn float(value: f64) -> Self {
        Self {
            value: LoxObject::Float(value),
        }
    }

    pub fn nil() -> Self {
        Self {
            value: LoxObject::Nil,
        }
    }

    pub fn r#true() -> Self {
        Self {
            value: LoxObject::Bool(true),
        }
    }

    pub fn r#false() -> Self {
        Self {
            value: LoxObject::Bool(false),
        }
    }
}
