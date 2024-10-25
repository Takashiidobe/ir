use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Hash, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tokenizer {
    line: usize,
    col: usize,
    index: usize,
    program: String,
    tokens: Vec<Token>,
    chars: Vec<char>,
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self {
            line: 1,
            col: 1,
            index: 0,
            program: String::new(),
            tokens: vec![],
            chars: vec![],
        }
    }
}

impl Tokenizer {
    pub fn tokenize(&mut self, program: &str) -> Vec<Token> {
        self.program.push_str(program);
        self.chars.extend(program.chars());
        let mut new_tokens = vec![];
        let chars: Vec<char> = self.program.chars().collect();

        let dict = HashMap::from([
            ('(', TokenType::LeftParen),
            (')', TokenType::RightParen),
            ('[', TokenType::LeftSquare),
            (']', TokenType::RightSquare),
            ('{', TokenType::LeftSquiggly),
            ('}', TokenType::RightSquiggly),
            (';', TokenType::Semicolon),
            (',', TokenType::Comma),
            (':', TokenType::Colon),
        ]);

        while self.index < chars.len() {
            let c = chars[self.index];

            if dict.contains_key(&c) {
                new_tokens.push(Token {
                    loc: self.loc(),
                    token: dict.get(&c).unwrap().clone(),
                });
                self.index += 1;
                continue;
            }

            match c {
                '0'..='9' => new_tokens.push(self.number()),
                '"' => new_tokens.push(self.string()),
                '+' => new_tokens.push(self.rel_op(TokenType::Plus, TokenType::AddAssign)),
                '-' => new_tokens.push(self.rel_op(TokenType::Minus, TokenType::SubAssign)),
                '*' => new_tokens.push(self.rel_op(TokenType::Star, TokenType::MulAssign)),
                '/' => new_tokens.push(self.rel_op(TokenType::Backslash, TokenType::DivAssign)),
                '%' => new_tokens.push(self.rel_op(TokenType::Percent, TokenType::ModAssign)),
                '!' => new_tokens.push(self.rel_op(TokenType::Bang, TokenType::NotEqual)),
                '=' => new_tokens.push(self.rel_op(TokenType::Equal, TokenType::EqualEqual)),
                '<' => new_tokens.push(self.rel_op(TokenType::LeftAngle, TokenType::LessThanEqual)),
                '>' => {
                    new_tokens.push(self.rel_op(TokenType::RightAngle, TokenType::GreaterThanEqual))
                }
                '&' => new_tokens.push(self.binary_char('&', TokenType::Ampersand, TokenType::And)),
                '|' => new_tokens.push(self.binary_char('|', TokenType::Pipe, TokenType::Or)),
                '\n' => {
                    self.line += 1;
                    self.col = 1;
                    self.index += 1;
                }
                ' ' | '\t' => {
                    self.col = 1;
                    self.index += 1;
                }
                '_' | 'a'..='z' | 'A'..='Z' => new_tokens.push(self.ident_or_keyword()),
                _ => {}
            }
        }

        self.tokens.extend(new_tokens.clone());
        new_tokens
    }

    fn ident_or_keyword(&mut self) -> Token {
        let mut s = String::new();
        let loc = self.loc();

        while self.index < self.chars.len() {
            let c = self.chars[self.index];
            match c {
                '_' | 'a'..='z' | 'A'..='Z' => {
                    s.push(c);
                    self.col += 1;
                    self.index += 1;
                }
                _ => {
                    self.col += 1;
                    self.index += 1;
                    break;
                }
            }
        }

        let keywords = HashMap::from([
            ("let", TokenType::Keyword(Keyword::Let)),
            ("fn", TokenType::Keyword(Keyword::Fn)),
            ("if", TokenType::Keyword(Keyword::If)),
            ("elif", TokenType::Keyword(Keyword::ElseIf)),
            ("else", TokenType::Keyword(Keyword::Else)),
            ("while", TokenType::Keyword(Keyword::While)),
            ("for", TokenType::Keyword(Keyword::For)),
            ("print", TokenType::Keyword(Keyword::Print)),
        ]);

        Token {
            loc,
            token: if keywords.contains_key(&s.as_str()) {
                keywords.get(&s.as_str()).unwrap().clone()
            } else {
                TokenType::Identifier(s)
            },
        }
    }

    fn rel_op(&mut self, short_token: TokenType, long_token: TokenType) -> Token {
        self.binary_char('=', short_token, long_token)
    }

    fn binary_char(
        &mut self,
        second_char: char,
        short_token: TokenType,
        long_token: TokenType,
    ) -> Token {
        if self.peek() == Some(second_char) {
            let token = Token {
                loc: self.loc(),
                token: long_token,
            };
            self.index += 2;
            token
        } else {
            let token = Token {
                loc: self.loc(),
                token: short_token,
            };
            self.index += 1;
            token
        }
    }

    fn string(&mut self) -> Token {
        let mut s = String::new();
        let loc = self.loc();
        self.index += 1;

        while self.index < self.chars.len() {
            let c = self.chars[self.index];
            match c {
                '"' => {
                    self.col += 1;
                    self.index += 1;
                    break;
                }
                _ => {
                    s.push(c);
                    self.col += 1;
                    self.index += 1;
                }
            }
        }

        Token {
            loc,
            token: TokenType::String(s),
        }
    }

    fn number(&mut self) -> Token {
        let mut s = String::new();
        let loc = self.loc();
        while self.index < self.chars.len() {
            let c = self.chars[self.index];
            match c {
                '0'..='9' => {
                    s.push(c);
                    self.col += 1;
                    self.index += 1;
                }
                _ => break,
            }
        }

        Token {
            loc,
            token: TokenType::Number(s.parse().unwrap()),
        }
    }

    fn peek(&self) -> Option<char> {
        if self.index + 1 < self.chars.len() {
            Some(self.chars[self.index + 1])
        } else {
            None
        }
    }

    fn loc(&self) -> SourceLocation {
        SourceLocation {
            line: self.line,
            col: self.col,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Hash, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    loc: SourceLocation,
    token: TokenType,
}

#[derive(
    Serialize, Deserialize, Default, Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct SourceLocation {
    line: usize,
    col: usize,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    #[default]
    Eof,
    Number(i64),
    String(String),
    Identifier(String),
    Plus,
    Minus,
    Star,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    Comma,
    LeftParen,
    RightParen,
    LeftSquiggly,
    RightSquiggly,
    LeftSquare,
    RightSquare,
    Equal,
    Bang,
    EqualEqual,
    NotEqual,
    LeftAngle,
    LessThanEqual,
    RightAngle,
    GreaterThanEqual,
    Semicolon,
    And,
    Or,
    Ampersand,
    Pipe,
    Percent,
    Backslash,
    Nil,
    False,
    True,
    Colon,
    Keyword(Keyword),
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Keyword {
    Let,
    Fn,
    While,
    If,
    ElseIf,
    Else,
    For,
    Print,
}

#[cfg(test)]
mod tests {
    use super::Tokenizer;
    use insta::assert_yaml_snapshot as test;

    macro_rules! snapshot {
        ($name:ident, $program:expr) => {
            #[test]
            fn $name() {
                let mut tokenizer = Tokenizer::default();
                let res = tokenizer.tokenize($program);
                test!(res);
            }
        };
    }

    snapshot!(print, "print 1;");
    snapshot!(number, " 10000");
    snapshot!(plus, "1 + 1");
    snapshot!(add_assign, "1 += 1");
    snapshot!(sub, "1 - 1");
    snapshot!(sub_assign, "1 -= 1");
    snapshot!(times, "1 * 1");
    snapshot!(mul_assign, "1 *= 1");
    snapshot!(div, "1 / 1");
    snapshot!(div_assign, "1 /= 1");
    snapshot!(rem, "1 % 1");
    snapshot!(mod_assign, "1 %= 1");
    snapshot!(string, "\"hello\" + \"world\"");
    snapshot!(equal_equal, "2 ==  3");
    snapshot!(equal, "1 = 1");
    snapshot!(not_equal, "1 != 1");
    snapshot!(bang, "!1");
    snapshot!(langle, "<");
    snapshot!(less_than_equal, "1 <= 1");
    snapshot!(rangle, "1 > 1");
    snapshot!(greater_than_equal, "1 >= 1");
    snapshot!(ampersand, "&");
    snapshot!(and, "1 && 1");
    snapshot!(pipe, "|");
    snapshot!(or, "1 || 1");
    snapshot!(comma, ",");
    snapshot!(array, "[1, 2]");
    snapshot!(colon, ":");
    snapshot!(semicolon, ";");
    snapshot!(var_decl, "let x = 10;");
    snapshot!(for_loop, "for (let i = 0; i < 10; i++) { print(i); }");
    snapshot!(
        while_loop,
        "let i = 0; while (i < 10) { print(i); i += 1; }"
    );
    snapshot!(fn_decl, "fn incr(i) { i += 1 }");
    snapshot!(fn_call, "incr(10);");
    snapshot!(if_stmt, "if (x < 10) { print(10); }");
    snapshot!(
        else_stmt,
        "if (x < 10) { print(10); } elif (x < 20) { print(20); } else { print(30); }"
    );
}
