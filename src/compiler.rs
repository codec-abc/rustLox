use crate::scanner::{Scanner, Token, TokenType};

pub fn compile(source: &str) {
    let mut scanner = Scanner::new(source.into());
    let mut line = 1;
    loop {
        let token: Token = scanner.scan_token();
        if token.line != line {
            print!("{:#06x?} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }

        print!("{:#04x?} {} {}", token.token_type, token.length, token.start);

        if token.token_type == TokenType::TokenEof {
            break;
        }
    }

}