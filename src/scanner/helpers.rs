use super::Scanner;

impl Scanner {
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
            if x == '\n' {
                self.line += 1;
            }
            cur_str.push(x);
        }
        cur_str
    }

    pub fn is_at_end(&mut self) -> bool {
        self.iter.peek().is_none()
    }

    pub fn check_next(&mut self, cmp: char) -> bool {
        self.iter.peek().is_some_and(|x| *x == cmp)
    }
}
