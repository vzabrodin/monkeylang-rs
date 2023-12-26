pub mod token;

use self::token::Token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let input = input.to_string();
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        self.ch = *self.input.as_bytes().get(self.read_position).unwrap_or(&0);
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> u8 {
        *self.input.as_bytes().get(self.read_position).unwrap_or(&0)
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            b'=' if self.peek_char() == b'=' => {
                self.read_char();
                Token::Equals
            }
            b'=' => Token::Assign,
            b'!' if self.peek_char() == b'=' => {
                self.read_char();
                Token::NotEquals
            }
            b'!' => Token::Bang,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Asterisk,
            b'/' => Token::Slash,
            b'<' if self.peek_char() == b'=' => {
                self.read_char();
                Token::LessThanOrEquals
            }
            b'<' => Token::LessThan,
            b'>' if self.peek_char() == b'=' => {
                self.read_char();
                Token::GreaterThanOrEquals
            }
            b'>' => Token::GreaterThan,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'(' => Token::LeftParen,
            b')' => Token::RightParen,
            b'{' => Token::LeftBrace,
            b'}' => Token::RightBrace,
            0 => Token::Eof,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let identifier = self.read_identifier();
                return match &identifier[..] {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "return" => Token::Return,
                    "true" => Token::True,
                    "false" => Token::False,
                    _ => Token::Identifier(identifier),
                };
            }
            b'0'..=b'9' => return Token::Integer(self.read_number()),
            x => Token::Illegal {
                char: x,
                position: self.position,
            },
        };

        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> String {
        let start_position = self.position;
        while let b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' = self.ch {
            self.read_char();
        }
        self.input[start_position..self.position].into()
    }

    fn read_number(&mut self) -> i32 {
        let start_position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.input[start_position..self.position].parse().unwrap()
    }

    fn skip_whitespace(&mut self) -> usize {
        let start_position = self.position;
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }

        self.position - start_position
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Token::Illegal {
                char: _,
                position: _,
            } => None,
            Token::Eof => None,
            token => Some(token),
        }
    }
}

#[test]
fn test_next_token() {
    let input = r#"
let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
"#;

    let tests = [
        Token::Let,
        Token::Identifier("five".into()),
        Token::Assign,
        Token::Integer(5),
        Token::Semicolon,
        Token::Let,
        Token::Identifier("ten".into()),
        Token::Assign,
        Token::Integer(10),
        Token::Semicolon,
        Token::Let,
        Token::Identifier("add".into()),
        Token::Assign,
        Token::Function,
        Token::LeftParen,
        Token::Identifier("x".into()),
        Token::Comma,
        Token::Identifier("y".into()),
        Token::RightParen,
        Token::LeftBrace,
        Token::Identifier("x".into()),
        Token::Plus,
        Token::Identifier("y".into()),
        Token::Semicolon,
        Token::RightBrace,
        Token::Semicolon,
        Token::Let,
        Token::Identifier("result".into()),
        Token::Assign,
        Token::Identifier("add".into()),
        Token::LeftParen,
        Token::Identifier("five".into()),
        Token::Comma,
        Token::Identifier("ten".into()),
        Token::RightParen,
        Token::Semicolon,
        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Integer(5),
        Token::Semicolon,
        Token::Integer(5),
        Token::LessThan,
        Token::Integer(10),
        Token::GreaterThan,
        Token::Integer(5),
        Token::Semicolon,
        Token::If,
        Token::LeftParen,
        Token::Integer(5),
        Token::LessThan,
        Token::Integer(10),
        Token::RightParen,
        Token::LeftBrace,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::RightBrace,
        Token::Else,
        Token::LeftBrace,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::RightBrace,
        Token::Integer(10),
        Token::Equals,
        Token::Integer(10),
        Token::Semicolon,
        Token::Integer(10),
        Token::NotEquals,
        Token::Integer(9),
        Token::Semicolon,
    ];

    let lexer = Lexer::new(input);

    for (extected_token, token) in lexer.zip(tests) {
        debug_assert_eq!(token, extected_token);
    }
}
