use crate::token;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;
enum State {
  Import,
  Export,
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
  state: State,
  token: String,
  escaped: bool,
}

impl<'a> Lexer<'a> {
  pub fn new(code: &'a str) -> Self {
    Lexer {
      chars: code.chars(),
      state: State::BlockStart,
      token: String::new(),
      escaped: false,
    }
  }

  fn next_token(&mut self) -> Option<token::Token> {
    self.token.clear();
    self.escaped = false;
    loop {
      if let Some(char) = self.chars.next() {
        match self.state {
          State::BlockStart => {
            if let Some(token) = self.further_block_start_token(char) {
              self.state = State::Block;
              return Some(token);
            }
          }
          State::Block => {
            if let Some(token) = self.further_block_token(char) {
              return Some(token);
            }
          }
          State::InlineJsx => {}
          State::BlockJsx => {}
          State::Import => {}
          State::Export => {}
        }
      } else if !self.token.is_empty() {
        return Some(token::get_token(self.token.clone()));
      } else {
        return None;
      }
    }
  }

  fn gen_token(&mut self) -> Option<token::Token> {
    if self.token.is_empty() {
      return None;
    }
    if self.escaped {
      Some(token::Token::Text(self.token.clone()))
    } else {
      Some(token::get_token(self.token.clone()))
    }
  }

  fn further_block_start_token(&mut self, char: char) -> Option<token::Token> {
    match char {
      ' ' => self.gen_token(),
      '\n' => self.gen_token(),
      _ => {
        self.token.push(char);
        return None;
      }
    }
  }

  fn further_block_token(&mut self, char: char) -> Option<token::Token> {
    match char {
      ' ' => self.gen_token(),
      '\n' => {
        self.state = State::BlockStart;
        self.gen_token()
      }
      _ => {
        self.token.push(char);
        None
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn lexer(code: &str) -> Vec<token::Token> {
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
        token::Token::Heading1,
        token::Token::Text("123".to_owned()),
        token::Token::Heading1,
        token::Token::Text("123".to_owned()),
      ]
    );
  }
}
