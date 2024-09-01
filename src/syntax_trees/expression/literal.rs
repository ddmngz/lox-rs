use super::Visitor;
use crate::syntax_trees::lox_object::LoxObject;
use crate::token::SmartString;

#[derive(Clone, Debug)]
pub struct Literal {
    pub value: LoxObject,
}

impl Visitor<String> for Literal {
    fn accept(&self) -> String {
        format!("LITERAL ({})",self.value)
    }
}

impl Visitor<LoxObject> for Literal {
    fn accept(&self) -> LoxObject {
        self.value.clone()
    }
}

impl Literal {

    pub fn new(inner:LoxObject) -> Self{
        Self{
            value: inner
        }
    }

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

    pub fn string(value:SmartString) -> Self{
        Self{
            value:LoxObject::String(value)
        }
    }
}
