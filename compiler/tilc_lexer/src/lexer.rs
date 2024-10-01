use std::str::Chars;

use crate::token::{
  Base, LiteralKind, Token,
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
  fn nth(&mut self, n: u32) -> char {
    debug_assert_ne!(n, 0);
    // let n: u32 = n + 1;
    for _ in 0..n - 1 {
      self.chars.next();
    }

    return self.step().unwrap_or(EOF);
  }
  fn step(&mut self) -> Option<char> {
    return self.chars.next();
  }


  fn is_digit(&mut self) -> bool {
    let mut containt_digits: bool = false;
    loop {
      match self.peek() {
        '_' => {
          self.step();
        }
        '0'..='9' => {
          containt_digits = true;
          self.step();
        }

        '.' | 'E' => {}

        _ => break,
      };
    }

    return containt_digits;
  }
  fn is_whitespace(&mut self) -> bool {
    return match self.chars.next() {
      Some(char) => char.is_whitespace(),
      None => true,
    };
  }
  fn number_kind(&mut self, current_char: char) -> LiteralKind {
    let mut base: Base = Base::Decimal;


    if current_char == '0' {
      match self.peek() {
        'e' => {
          base = Base::Binary;
          self.step();
          if !self.is_digit() {
            return LiteralKind::Int { base };
          };
        }
        's' => {
          base = Base::Octal;
          self.step();
          if !self.is_digit() {
            return LiteralKind::Int { base };
          };
        }

        '0'..='9' | '_' => {
          self.is_digit();
        }

        _ => return LiteralKind::Int { base },
      };
    } else {
      self.is_digit();
    };

    // TODO: Handle flaoting point number
    return match self.peek() {
      _ => LiteralKind::Int {
        base: Base::Decimal,
      },
    };
  }
  fn line_comment(&mut self) -> TokenKind {
    todo!();
  }
  fn block_comment(&mut self) -> TokenKind {
    todo!();
  }
  fn char_literal_or_lifetime(&mut self) -> TokenKind {
    debug_assert_eq!(self.peek(), '\'');
    todo!();
  }


  fn char_to_token(&mut self) -> Token {
    let current_char: char = match self.step() {
      Some(char) => char,
      None => return Token::new(Eof, 0),
    };

    let token_kind: TokenKind = match current_char {
      char if self.is_whitespace() => TokenKind::Whitespace,

      '/' => match self.peek() {
        '/' => self.line_comment(),
        '*' => self.block_comment(),

        _ => TokenKind::Slash,
      },

      char @ '0'..='9' => {
        let kind: LiteralKind = self.number_kind(char);

        todo!();
      }

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
      '^' => Caret,
      '%' => Percent,

      _ => Unknown,
    };

    todo!()
  }
}


#[cfg(test)]
mod test {
  use crate::token::{Base, LiteralKind};

  use super::Lexer;


  #[test]
  fn peek() {
    assert_eq!(Lexer::new("a").peek(), 'a');
    assert_eq!(Lexer::new("1").peek(), '1');
    assert_eq!(Lexer::new("?").peek(), '?');
    assert_eq!(Lexer::new("\\").peek(), '\\');
    assert_eq!(Lexer::new("").peek(), '\0');
  }
  #[test]
  fn nth() {
    let mut l: Lexer<'_> = Lexer::new("fx basty()");


    assert_eq!(l.nth(2), 'x');
    assert_eq!(l.nth(3), 'a');
    assert_eq!(l.nth(4), '(');
    assert_eq!(l.nth(5), '\0');
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

  // #[test]
  // fn is_digit() {
  //   let mut l: Lexer<'_> = Lexer::new("1");

  //   assert_eq!(
  //     l.number_kind(l.peek()),
  //     LiteralKind::Int {
  //       base: Base::Decimal
  //     }
  //   );
  //   assert_eq!(Lexer::new("1_000").is_digit(), true);
  //   assert_eq!(Lexer::new("0e101").is_digit(), true);
  //   assert_eq!(Lexer::new("0s71").is_digit(), true);
  //   assert_eq!(Lexer::new("0o1A1").is_digit(), true);
  // }
}
