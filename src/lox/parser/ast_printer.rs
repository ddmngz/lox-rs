use super::expr::*;

pub struct AstPrinter;

fn parenthesize(name: &str, args: &[&Expr]) -> String {
    let mut str = String::from("(");
    str.push_str(name);
    for expr in args {
        str.push(' ');
        str.push_str(walk_expr::<String>(&mut AstPrinter, expr).as_str());
    }
    str.push(')');
    str
}

impl AstPrinter {
    #[allow(dead_code)]
    pub fn print(&mut self, expr: Expr) -> String {
        walk_expr(self, &expr)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary(&self, expr: &Binary) -> String {
        let args = [expr.left.as_ref(), expr.right.as_ref()];
        parenthesize(&expr.operator.to_string(), &args)
    }

    fn visit_grouping(&self, expr: &Grouping) -> String {
        parenthesize("group", &[&expr.expression])
    }

    fn visit_literal(&self, expr: &Literal) -> String {
        use LiteralValue::*;
        match &expr.value {
            Float(f) => f.to_string(),
            r#String(s) => s.clone(),
            Nil => "nil".to_string(),
            Bool(b) => b.to_string(),
        }
    }

    fn visit_unary(&self, expr: &Unary) -> String {
        parenthesize(&expr.operator.to_string(), &[&expr.right])
    }
}
