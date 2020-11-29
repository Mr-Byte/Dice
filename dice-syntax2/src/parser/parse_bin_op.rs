use crate::Parser;

impl<'a> Parser<'a> {
    pub(super) fn parse_bin_op(&mut self, _: bool) {
        self.bump();
        self.skip_trivia();
    }
}
