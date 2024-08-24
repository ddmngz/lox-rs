use super::ast::expression::*;
#[derive(Default)]
pub struct AstPrinter;

fn parenthesize(name: &str, args: &[&Expr]) -> String {
    let mut str = String::from("(");
    str.push_str(name);
    for expr in args {
        str.push(' ');
        str.push_str(walk_expr::<String>(&AstPrinter, expr).unwrap().as_str());
    }
    str.push(')');
    str
}

impl AstPrinter {
    #[allow(dead_code)]
    pub fn print(&mut self, expr: Expr) -> String {
        walk_expr(self, &expr).unwrap()
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl AstPrinter {
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
            r#String(s) => format!("{s}"),
            Nil => "nil".to_string(),
            Bool(b) => b.to_string(),
        }
    }

    fn visit_unary(&self, expr: &Unary) -> String {
        parenthesize(&expr.operator.to_string(), &[&expr.right])
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary(&self, expr: &Binary) -> Result<String> {
        Ok(self.visit_binary(expr))
    }
    fn visit_grouping(&self, expr: &Grouping) -> Result<String> {
        Ok(self.visit_grouping(expr))
    }

    fn visit_literal(&self, expr: &Literal) -> Result<String> {
        Ok(self.visit_literal(expr))
    }

    fn visit_unary(&self, expr: &Unary) -> Result<String> {
        Ok(self.visit_unary(expr))
    }
}
