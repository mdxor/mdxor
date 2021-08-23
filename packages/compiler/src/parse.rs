use crate::block;

struct Parser<'a> {
  code: &'a str,
  chars: std::str::Chars<'a>,
}

impl<'a> Parser<'a> {
  fn new(code: &'a str) -> Self {
    Parser {
      code,
      chars: code.chars(),
    }
  }
}

pub fn parse(code: &str) -> String {
  let parser = Parser::new(code);
  "".to_owned()
}
