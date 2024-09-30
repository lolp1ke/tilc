//! Token túriniń jeńildetilgen nusqasy


// Temporary version of the Token before parsing
pub struct Token {
  kind: TokenKind,
  pos: usize,
}
impl Token {
  pub fn new(kind: TokenKind, pos: usize) -> Self {
    return Self { kind, pos };
  }
}


pub enum TokenKind {
  /// Bos oryn tańbasy
  Whitespace,
  // Single line Comment
  /// Túsinikteme
  LineComment,
  // Multiple line Comment
  /// Birneshe joldyq túsinikteme
  BlockComment,

  /// Identıfıkatorlar jáne Kilt Sózder
  Identifier,
  /// Shıki Identıfıkator, mysaly: s#ainymaly
  ///
  /// s tańbasy "Shıkı"degendi bildiredi
  RawIdentifier,
  /// Jaramsyz Identıfıkator
  InvalidIdentifier,

  // TODO: choose more suitable definition for the "Literal"
  /// Sózbe-Sóz
  Literal { kind: LiteralKind },

  /// Ómir súrý uzaqtyǵy
  Lifetime,

  // Single char tokens
  /// ;
  Semicolon,
  /// :
  Colon,
  /// ,
  Comma,
  /// .
  Dot,
  /// (
  OpenParen,
  /// )
  CloseParen,
  /// {
  OpenBrace,
  /// }
  CloseBrace,
  /// [
  OpenBracket,
  /// ]
  CloseBracket,
  /// @
  At,
  /// #
  Hashtag,
  /// ~
  Tilde,
  /// ?
  Question,
  /// $
  Dollar,
  /// =
  Eq,
  /// !
  Bang,
  /// <
  Lt,
  /// >
  Gt,
  /// -
  Minus,
  /// +
  Plus,
  /// &
  And,
  /// |
  Or,
  /// *
  Star,
  /// /
  Slash,
  /// ^
  Caret,
  /// %
  Percent,

  /// Belgisiz belgi, mysaly: '№'
  Unknown,
  /// Faıldyń sońy
  Eof,
}


// TODO: create own prefix style
/// Sannyń negizi
pub enum Base {
  /// Munyń prefıksi: "0e"
  Binary,

  /// Munyń prefıksi: "0s"
  Octal,

  /// Munyń prefıksi joq
  Decimal,

  /// Munyń prefıksi: "0o"
  Hexidecimal,
}
pub enum LiteralKind {
  /// Bútin san
  Int {
    base: Base,
  },
  Float,
  Char,
  Byte,
  String,
}
