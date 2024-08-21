use super::parser::expr::*;
use crate::error::LoxRuntimeError;

pub type LoxObject = LiteralValue;
pub struct Interpreter {}

pub type Result<T> = std::result::Result<T, LoxRuntimeError>;

fn is_equal(left:LoxObject, right:LoxObject) -> bool{
    use LiteralValue::{Float, Bool, String, Nil};
    match (left,right){
        (Nil, Nil) => true,
        (Bool(a), Bool(b)) => a == b,
        (String(a), String(b)) => a == b,
        // should be unreachable, might be worth reviewing control flow at some point
        (Float(_), Float(_)) => panic!("somehow ended up comparing two floats in is_equal"),
        (_,_) => false,
    }
}

// one could do this by instead implementing like Add, Subtract, Cmp, and PartialEq for LoxObject
impl Visitor<LoxObject> for Interpreter {
    fn visit_binary(&self, expr: &Binary) -> Result<LoxObject> {
        use LiteralValue::{Float, Bool, String};
        use BinaryOperator::{PLUS,MINUS,STAR,SLASH,GREATER,GREATEREQUAL,LESS,LESSEQUAL,EQUALEQUAL,BANGEQUAL};

        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;


        let (left, right) = match (left,right){
            (Float(left),Float(right)) => (left,right),
            (String(left), String(right)) => return Ok(LoxObject::String(format!("{left}{right}"))),
            (left,right) => {
                match expr.operator{
                    BANGEQUAL => return Ok(Bool(!is_equal(left,right))),
                    EQUALEQUAL => return Ok(Bool(is_equal(left,right))),
                    _ => return Err(LoxRuntimeError::InvalidOperand)
                }
            }
        };

        Ok(match expr.operator {
            PLUS => Float(left + right),
            MINUS => Float(left - right),
            STAR => Float(left * right),
            SLASH => Float(left / right),
            GREATER => Bool(left > right),
            GREATEREQUAL => Bool(left >=right),
            LESS => Bool(left < right),
            LESSEQUAL => Bool(left <=right),
            EQUALEQUAL => Bool(left == right),
            BANGEQUAL => Bool(left != right),
        })
    }

    fn visit_grouping(&self, expr: &Grouping) -> Result<LoxObject> {
        let expression = *expr.expression.clone();
        self.evaluate(&expression)
    }
    fn visit_literal(&self, expr: &Literal) -> Result<LoxObject> {
        // could remove the clone soon
        Ok(expr.value.clone())
    }
    fn visit_unary(&self, expr: &Unary) -> Result<LoxObject> {
        let right = self.evaluate(&expr.right)?;
        Ok(
        match expr.operator {
            UnaryOperator::MINUS => LoxObject::Float(-1.0 * right.cast_float()?),
            UnaryOperator::BANG => not_truthy(right),
        })
    }
}

fn not_truthy(value: LoxObject) -> LoxObject {
    LoxObject::Bool(!is_truthy(value))
}

fn is_truthy(value: LoxObject) -> bool {
    match value {
        LoxObject::Bool(b) => b,
        LoxObject::Nil => false,
        _ => true,
    }
}

impl LoxObject {
    fn cast_float(&self) -> Result<f64> {
        match self {
            LoxObject::Float(val) => Ok(*val),
            _ => Err(LoxRuntimeError::InvalidOperand),
        }
    }
}


impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<LoxObject> {
        walk_expr(self, expr)
    }
    pub fn interpret(self, expr:&Expr) -> Result<()>{
        let value = self.evaluate(expr)?;
        println!("{}",value);
        Ok(())
    }
}
