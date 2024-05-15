use crate::lox::Scanner;

impl<'a> Scanner<'a> {
    pub fn advance_while<F>(&mut self, f: F)
    where
        F: Fn(&(usize, char)) -> bool,
    {
        while self.iter.next_if(&f).is_some() {}
    }

    pub fn get_pos(&mut self) -> usize {
        if let Some((pos, _)) = self.iter.peek() {
            *pos
        } else {
            self.source.len()
        }
    }

    pub fn get_lexeme(&mut self) -> &'a str {
        let pos = self.get_pos();
        self.source[self.lex_start..pos].as_ref()
    }

    pub fn is_at_end(&mut self) -> bool {
        self.iter.peek().is_none()
    }

    pub fn check_next(&mut self, cmp: char) -> bool {
        self.iter.peek().is_some_and(|(_, x)| *x == cmp)
    }
}
