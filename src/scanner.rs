pub struct Scanner {
    line: usize,
    source: String,
    start: usize,
    current: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    start: usize,
    length: usize,
    pub line: usize,
    pub content: String,
}

impl Token {
    pub fn new_dummy_token() -> Token {
        Token {
            token_type: TokenType::TokenNumber,
            start: 0,
            length: 0,
            line: 0,
            content: "DummyToken".into()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    TokenLeftParen, 
    TokenRightParen,
    TokenLeftBrace, 
    TokenRightBrace,
    TokenComma, 
    TokenDot, 
    TokenMinus, 
    TokenPlus,
    TokenSemicolon, 
    TokenSlash, 
    TokenStar,
    TokenBang, 
    TokenBangEqual,
    TokenEqual, 
    TokenEqualEqual,
    TokenGreater, 
    TokenGreaterEqual,
    TokenLess, 
    TokenLessEqual,
    TokenIdentifier, 
    TokenString, 
    TokenNumber,
    TokenAnd, 
    TokenClass, 
    TokenElse, 
    TokenFalse,
    TokenFor, 
    TokenFun, 
    TokenIf, 
    TokenNil, 
    TokenOr,
    TokenPrint, 
    TokenReturn, 
    TokenSuper, 
    TokenThis,
    TokenTrue, 
    TokenVar, 
    TokenWhile,
    TokenError, 
    TokenEof
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || 
    (c >= 'A' && c <= 'Z') ||
    c == '_'
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            line: 1,
            current: 0,
            start: 0,
            source: source.into()
        }
    }

    pub fn scan_token(&mut self) -> Token {
        if self.is_at_end() {
            return self.make_token(TokenType::TokenEof);
        }

        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::TokenEof);
        }

        let c = self.advance();

        if is_alpha(c) {
            return self.identifier();
        }

        if is_digit(c) {
            return self.number();
        }

        match c {
            '(' => return self.make_token(TokenType::TokenLeftParen),
            ')' => return self.make_token(TokenType::TokenRightParen),
            '{' => return self.make_token(TokenType::TokenLeftBrace),
            '}' => return self.make_token(TokenType::TokenRightBrace),
            ';' => return self.make_token(TokenType::TokenSemicolon),
            ',' => return self.make_token(TokenType::TokenComma),
            '.' => return self.make_token(TokenType::TokenDot),
            '-' => return self.make_token(TokenType::TokenMinus),
            '+' => return self.make_token(TokenType::TokenPlus),
            '/' => return self.make_token(TokenType::TokenSlash),
            '*' => return self.make_token(TokenType::TokenStar),
            '!' => {
                let token_type = if self.match_char('=') { TokenType::TokenBangEqual } else { TokenType::TokenBang };
                return self.make_token(token_type);
            }
            '=' => {
                let token_type = if self.match_char('=') { TokenType::TokenEqualEqual } else { TokenType::TokenEqual };
                return self.make_token(token_type);
            }
            '<' => {
                let token_type = if self.match_char('=') { TokenType::TokenLessEqual } else { TokenType::TokenLess };
                return self.make_token(token_type);
            }
            '>' => {
                let token_type = if self.match_char('=') { TokenType::TokenGreaterEqual } else { TokenType::TokenGreater };
                return self.make_token(token_type);
            }
            '"' => {
                return self.string();
            }

            _ => return self.error_token("Unexpected character.")
        }
    }

    fn identifier(&mut self) -> Token {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }

        let identifier_type = self.identifier_type();
        self.make_token(identifier_type)
    }

    fn identifier_type(&mut self) -> TokenType {
        let start_char = self.source.as_bytes()[self.start] as char;
        let default = TokenType::TokenIdentifier;

        match start_char {
            'a' => { return self.check_keyword(1, 2, "nd", TokenType::TokenAnd); }
            'c' => { return self.check_keyword(1, 4, "lass", TokenType::TokenClass); }
            'e' => { return self.check_keyword(1, 3, "lse", TokenType::TokenElse); }
            'i' => { return self.check_keyword(1, 1, "f", TokenType::TokenIf); }
            'n' => { return self.check_keyword(1, 2, "il", TokenType::TokenNil); }
            'o' => { return self.check_keyword(1, 1, "r", TokenType::TokenOr); }
            'p' => { return self.check_keyword(1, 4, "rint", TokenType::TokenPrint); }
            'r' => { return self.check_keyword(1, 5, "eturn", TokenType::TokenReturn); }
            's' => { return self.check_keyword(1, 4, "uper", TokenType::TokenSuper); }
            'v' => { return self.check_keyword(1, 2, "ar", TokenType::TokenVar); }
            'w' => { return self.check_keyword(1, 4, "hile", TokenType::TokenWhile); }
            'f' => {
                if self.current - self.start > 1 {
                    let next_starting_char = self.source.as_bytes()[self.start + 1] as char;
                    match next_starting_char {
                        'a' => { return self.check_keyword(2, 3, "lse", TokenType::TokenFalse); }
                        'o' => { return self.check_keyword(2, 1, "r", TokenType::TokenFor); }
                        'u' => { return self.check_keyword(2, 1, "n", TokenType::TokenFun); }
                        _ => return default,
                    }
                }
                default
            }
            't' => {
                if self.current - self.start > 1 {
                    let next_starting_char = self.source.as_bytes()[self.start + 1] as char;
                    match next_starting_char {
                        'h' => { return self.check_keyword(2, 2, "is", TokenType::TokenThis); }
                        'r' => { return self.check_keyword(2, 2, "ue", TokenType::TokenTrue); }
                        _ => return default,
                    }
                }
                default
            }
            _ => default
        }
    }

    fn check_keyword(&mut self, start: usize, length: usize, rest: &str, token_type: TokenType) -> TokenType {
        if self.current - self.start == start + length {
            for i in 0..length  {
                let current_substring_char = self.source.as_bytes()[self.start + start + i];
                let to_compare_to = rest.as_bytes()[i];

                if current_substring_char != to_compare_to {
                    return TokenType::TokenIdentifier;
                }
            }
            return token_type;
        }

        TokenType::TokenIdentifier
    }

    fn number(&mut self) -> Token {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }

        self.make_token(TokenType::TokenNumber)
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line = self.line + 1;
                self.advance();
            }
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string");
        }

        self.advance();

        self.make_token(TokenType::TokenString)
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => { 
                    self.advance(); 
                }
                '\n' => { 
                    self.line = self.line + 1;
                    self.advance();
                    break;
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }

                    break;
                }
                _ => { return; }
            }
        }
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        let next_char = self.source.as_bytes()[self.current + 1];
        next_char as char
    }

    fn peek(&self) -> char {
        let current_char = self.source.as_bytes()[self.current];
        current_char as char
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.get_current_char() != expected {
            return false;
        }

        self.current = self.current + 1;
        return true;
    } 

    fn get_current_char(&self) ->  char {
        let current_char = self.source.as_bytes()[self.current];
        current_char as char
    }

    fn advance(&mut self) -> char {
        let current_char = self.get_current_char();
        self.current = self.current + 1;
        current_char
    }

    fn is_at_end(&self) -> bool {
        self.current >=  self.source.as_bytes().len() - 1
    }

    fn make_token(&self, token_type: TokenType) -> Token {

        let string = self.source[self.start..self.current].into();

        Token {
            token_type: token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
            content: string
        }
    }

    fn error_token(&self, message: &str) -> Token {

        Token {
            token_type: TokenType::TokenError,
            start: 0,
            length: message.as_bytes().len(),
            line: self.line,
            content: message.into()
        }
    }
}
