use crate::block;

enum State {
  Normal,
  InlineJsx,
  BlockJsx,
}

struct Parser<'a> {
  chars: std::str::Chars<'a>,
  token_stack: Vec<String>,
  state: State,
}

impl<'a> Parser<'a> {
  pub fn new(code: &'a str) -> Self {
    Parser {
      chars: code.chars(),
      token_stack: vec![],
      state: State::Normal,
    }
  }

  fn next_token(&mut self) -> Option<String> {
    if self.token_stack.len() > 0 {
      let token = self.token_stack.pop().unwrap();
      return Some(token);
    }
    let mut token = String::new();
    loop {
      if let Some(char) = self.chars.next() {
        if self.further_token_normal(&mut token, char) == true {
          return Some(token);
        }
      } else {
        if token.is_empty() {
          return None;
        }
        return Some(token);
      }
    }
  }

  fn further_token_normal(&mut self, token: &mut String, char: char) -> bool {
    match char {
      ' ' => {
        if token.is_empty() {
          return false;
        }
        return true;
      }
      '\n' => {
        if token.is_empty() {
          token.push(char);
          return true;
        }
        let mut next_token = String::new();
        next_token.push(char);
        self.token_stack.push(next_token);
        return true;
      }
      _ => {
        token.push(char);
        return false;
      }
    }
  }
}

pub fn parse(code: &str) -> Vec<String> {
  let mut parser = Parser::new(code);
  let mut tokens = vec![];
  loop {
    if let Some(token) = parser.next_token() {
      tokens.push(token)
    } else {
      break;
    }
  }
  tokens
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_1() {
    assert_eq!(
      parse("# 123 #\n123"),
      vec![
        "#".to_owned(),
        "123".to_owned(),
        "#".to_owned(),
        "\n".to_owned(),
        "123".to_owned()
      ]
    );
  }
}
