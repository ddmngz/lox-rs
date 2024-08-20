use super::expr::*;

pub type LoxObject = LiteralValue;
pub struct Interpreter {}

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
    fn visit_binary(&self, expr: &Binary) -> LoxObject {
        use LiteralValue::{Float, Bool, String};
        use BinaryOperator::{PLUS,MINUS,STAR,SLASH,GREATER,GREATEREQUAL,LESS,LESSEQUAL,EQUALEQUAL,BANGEQUAL};

        let left = self.evaluate(&expr.left);
        let right = self.evaluate(&expr.right);


        let (left, right) = match (left,right){
            (Float(left),Float(right)) => (left,right),
            (String(left), String(right)) => return LoxObject::String(format!("{left}{right}")),
            (left,right) => {
                match expr.operator{
                    BANGEQUAL => return Bool(!is_equal(left,right)),
                    EQUALEQUAL => return Bool(is_equal(left,right)),
                    _ => panic!("expected 2 floats or 2 strings for binary expression {:?}", expr)
                }
            }
        };

        match expr.operator {
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
        }
    }

    fn visit_grouping(&self, expr: &Grouping) -> LoxObject {
        self.evaluate(&Expr::Grouping(expr.clone()))
    }
    fn visit_literal(&self, expr: &Literal) -> LoxObject {
        // could remove the clone soon
        expr.value.clone()
    }
    fn visit_unary(&self, expr: &Unary) -> LoxObject {
        let right = self.evaluate(&expr.right);
        match expr.operator {
            UnaryOperator::MINUS => negate(right),
            UnaryOperator::BANG => not_truthy(right),
        }
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
    fn cast_float(&self) -> Option<f64> {
        match self {
            LoxObject::Float(val) => Some(*val),
            _ => None,
        }
    }

    fn cast_float_checked(&self) -> f64{ 
        match self.cast_float(){
            Some(value) => value,
            None => panic!("expected float, received {:?}",self)
        }
    }
}

fn negate(value: LoxObject) -> LoxObject {
    LoxObject::Float(match value.cast_float() {
        Some(val) => val * -1.0,
        None => f64::NAN,
    })
}

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> LoxObject {
        walk_expr(self, expr)
    }
}
