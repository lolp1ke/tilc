//! Token túriniń jeńildetilgen nusqasy

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
  /// &
  And,
  /// |
  Or,
  /// +
  Plus,
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
  EOF,
}

// TODO: create own prefix style
/// Sannyń negizi
pub enum Base {
  /// Munyń prefıksi: "0b"
  Binary,

  /// Munyń prefıksi: "0o"
  Octal,

  /// Munyń prefıksi joq
  Decimal,

  /// Munyń prefıksi: "0x"
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
