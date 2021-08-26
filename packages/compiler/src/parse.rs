use crate::block;

enum State {
  None,
  InlineJsx,
  BlockJsx,
}

struct Parser<'a> {
  chars: std::str::Chars<'a>,
  tokenStack: Vec<String>,
  state: State,
}

impl<'a> Parser<'a> {
  fn new(code: &'a str) -> Self {
    Parser {
      chars: code.chars(),
      tokenStack: vec![],
      state: State::None,
    }
  }

  fn next_token() -> String {
    String::from("")
  }
}

pub fn parse(code: &str) -> String {
  let parser = Parser::new(code);
  "".to_owned()
}
