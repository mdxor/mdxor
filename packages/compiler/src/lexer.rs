use lazy_static::lazy_static;
use std::collections::HashSet;
enum State {
  BlockStart,
  Block,
  InlineJsx,
  BlockJsx,
}

lazy_static! {
  static ref KEYCHARSET: HashSet<&'static char> = {
    let keyCharSet: HashSet<&'static char> = ['#', '\\', '`'].into_iter().collect();
    keyCharSet
  };
}

struct Lexer<'a> {
  chars: std::str::Chars<'a>,
  token_stack: Vec<String>,
  state: State,
}

impl<'a> Lexer<'a> {
  pub fn new(code: &'a str) -> Self {
    Lexer {
      chars: code.chars(),
      token_stack: vec![],
      state: State::BlockStart,
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
        match self.state {
          State::BlockStart => {
            if self.further_block_start_token(&mut token, char) == true {
              self.state = State::Block;
              return Some(token);
            }
          }
          State::Block => {
            if self.further_block_token(&mut token, char) == true {
              return Some(token);
            }
          }
          State::InlineJsx => {}
          State::BlockJsx => {}
        }
      } else {
        if token.is_empty() {
          return None;
        }
        return Some(token);
      }
    }
  }

  fn further_block_start_token(&mut self, token: &mut String, char: char) -> bool {
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

  fn further_block_token(&mut self, token: &mut String, char: char) -> bool {
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

#[cfg(test)]
mod tests {
  use super::*;

  fn lexer(code: &str) -> Vec<String> {
    let mut parser = Lexer::new(code);
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
  #[test]
  fn parse_1() {
    assert_eq!(
      lexer("# 123 #\n123"),
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
