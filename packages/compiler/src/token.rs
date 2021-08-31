pub enum TokenKind {
  Heading1,
  Heading2,
  Heading3,
  Heading4,
  Heading5,
  Heading6,
  Text,
  InlineCode,
}

pub struct Token {
  pub kind: TokenKind,
  pub value: String,
}

pub fn get_token_kind(token: &str) -> TokenKind {
  match token {
    "#" => TokenKind::Heading1,
    "##" => TokenKind::Heading2,
    "###" => TokenKind::Heading3,
    "####" => TokenKind::Heading4,
    "#####" => TokenKind::Heading5,
    "######" => TokenKind::Heading6,
    _ => TokenKind::Text,
  }
}
