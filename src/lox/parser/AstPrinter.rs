use super::structs::*;

struct AstPrinter;

fn parenthesize(name: &str, args: &[&Box<Expr>]) -> String {
    let mut str = String::from("(");
    str.push_str(name);
    for expr in args {
        str.push(' ');
        str.push_str(walk_expr::<String>(&mut AstPrinter, &expr).as_str());
    }
    str.push(')');
    str
}

impl Visitor<String> for AstPrinter {
    fn visit_binary(&self, expr: &Binary) -> String {
        let args = [&expr.left, &expr.right];
        parenthesize(expr.operator.lexeme, &args)
    }
    fn visit_grouping(&self, expr: &Grouping) -> String {
        parenthesize("group", &[&expr.expression])
    }
    fn visit_literal(&self, expr: &Literal) -> String {
        match &expr.value {
            Some(LiteralEnum::Float(f)) => f.to_string(),
            Some(LiteralEnum::String(s)) => s.clone(),
            None => String::from("nil"),
        }
    }
    fn visit_unary(&self, expr: &Unary) -> String {
        parenthesize(expr.operator.lexeme, &[&expr.right])
    }
}
