use std::str::Chars;

use crate::token::{
  Token,
  TokenKind::{self, *},
};


const EOF: char = '\0';

pub struct Lexer<'a> {
  chars: Chars<'a>,
  len_left: usize,
}
impl<'a> Lexer<'a> {
  pub fn new(str: &'a str) -> Self {
    return Self {
      chars: str.chars(),
      len_left: str.len(),
    };
  }


  fn peek(&self) -> char {
    return match self.chars.clone().peekable().peek() {
      Some(char) => *char,
      None => EOF,
    };
  }
  fn step(&mut self) -> Option<char> {
    return self.chars.next();
  }

  fn char_literal_or_lifetime(&mut self) -> TokenKind {
    debug_assert_eq!(self.peek(), '\'');
    todo!();
  }

  fn char_to_token(&mut self) -> Token {
    let first_char: char = match self.step() {
      Some(char) => char,
      None => return Token::new(Eof, 0),
    };

    let token_kind: TokenKind = match first_char {
      '\'' => self.char_literal_or_lifetime(),

      ';' => Semicolon,
      ':' => Colon,
      ',' => Comma,
      '.' => Dot,
      '(' => OpenParen,
      ')' => CloseParen,
      '{' => OpenBrace,
      '}' => CloseBrace,
      '[' => OpenBracket,
      ']' => CloseBracket,
      '@' => At,
      '#' => Hashtag,
      '?' => Question,
      '$' => Dollar,
      '=' => Eq,
      '!' => Bang,
      '<' => Lt,
      '>' => Gt,
      '-' => Minus,
      '+' => Plus,
      '&' => And,
      '|' => Or,
      '/' => Slash,
      '^' => Caret,
      '%' => Percent,

      _ => Unknown,
    };

    todo!()
  }
}


#[cfg(test)]
mod test {
  use super::Lexer;


  #[test]
  fn peek() {
    assert_eq!(Lexer::new("?").peek(), '?');
    assert_eq!(Lexer::new("\\").peek(), '\\');
    assert_eq!(Lexer::new("a").peek(), 'a');
    assert_eq!(Lexer::new("1").peek(), '1');
    assert_eq!(Lexer::new("").peek(), '\0');
  }
  #[test]
  fn step() {
    let mut l: Lexer<'_> = Lexer::new("fx basty()");

    assert_eq!(l.step(), Some('f'));
    assert_eq!(l.step(), Some('x'));
    assert_eq!(l.step(), Some(' '));
    assert_eq!(l.step(), Some('b'));
    assert_eq!(l.step(), Some('a'));
    assert_eq!(l.step(), Some('s'));
    assert_eq!(l.step(), Some('t'));
    assert_eq!(l.step(), Some('y'));
    assert_eq!(l.step(), Some('('));
    assert_eq!(l.step(), Some(')'));
    assert_eq!(l.step(), None);
  }
}
