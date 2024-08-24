use super::Scanner;

impl<'a> Scanner<'a> {
    pub fn advance_while<F>(&mut self, f: F)
    where
        F: Fn(&char) -> bool,
    {
        while self.iter.next_if(&f).is_some() {}
    }

    pub fn advance_and_get_literal<F>(&mut self, f: F) -> String
    where
        F: Fn(&char) -> bool,
    {
        let mut cur_str = String::new();
        while let Some(x) = self.iter.next_if(&f) {
            println!("{x} != \"");
            if x == '\n' {
                self.line += 1;
            }
            cur_str.push(x);
        }
        cur_str
    }
}
