use crate::{chunk::{Chunk, OpCode, map_opcode_to_binary}, scanner::{Scanner, Token, TokenType}, value::Value};

struct Parser {
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
    scanner: Scanner,
    compiling_chunk: Chunk,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    PrecNone,
    PrecAssignment,  // =
    PrecOr,          // Or
    PrecAnd,         // And
    PrecEquality,    // == !=
    PrecComparison,  // < > <= >=
    PrecTerm,        // + -
    PrecFactor,      // * /
    PrecUnary,       // ! -
    PrecCall,        // . ()
    PrecPrimary
}

fn get_next_rule(precedence: Precedence) -> Precedence {
    match precedence {
        Precedence::PrecNone => Precedence::PrecAssignment,
        Precedence::PrecAssignment => Precedence::PrecOr,
        Precedence::PrecOr => Precedence::PrecAnd,
        Precedence::PrecAnd => Precedence::PrecEquality,
        Precedence::PrecEquality => Precedence::PrecComparison,
        Precedence::PrecComparison => Precedence::PrecTerm,
        Precedence::PrecTerm => Precedence::PrecFactor,
        Precedence::PrecFactor => Precedence::PrecUnary,
        Precedence::PrecUnary => Precedence::PrecCall,
        Precedence::PrecCall => Precedence::PrecPrimary,
        Precedence::PrecPrimary => Precedence::PrecPrimary,

    }
}

enum ParseFn {
    Grouping,
    Unary,
    Binary,
    Number,
    None
}

struct ParseRule {
    prefix: ParseFn,
    infix: ParseFn,
    precedence: Precedence
}

impl Parser {

    pub fn get_rule(operator_type: TokenType) -> (ParseFn, ParseFn, Precedence) {
        match operator_type {
            TokenType::TokenLeftParen =>    (ParseFn::Grouping, ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenRightParen =>   (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenLeftBrace =>    (ParseFn::None,     ParseFn::None,   Precedence::PrecNone), 
            TokenType::TokenRightBrace =>   (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenComma =>         (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenDot =>           (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenMinus =>         (ParseFn::Unary,    ParseFn::Binary, Precedence::PrecTerm),
            TokenType::TokenPlus =>          (ParseFn::None,     ParseFn::Binary, Precedence::PrecTerm),
            TokenType::TokenSemicolon =>     (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenSlash =>         (ParseFn::None,     ParseFn::Binary, Precedence::PrecFactor),
            TokenType::TokenStar =>          (ParseFn::None,     ParseFn::Binary, Precedence::PrecFactor),
            TokenType::TokenBang =>          (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenBangEqual =>    (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenEqual =>         (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenEqualEqual =>   (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenGreater =>       (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenGreaterEqual => (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenLess =>          (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenLessEqual =>    (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenIdentifier =>    (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenString =>        (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenNumber =>        (ParseFn::Number,   ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenAnd =>           (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenClass =>         (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenElse =>          (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenFalse =>         (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenFor =>           (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenFun =>           (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenIf =>            (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenNil =>           (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenOr =>            (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenPrint =>         (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenReturn =>        (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenSuper =>         (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenThis =>          (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenTrue =>          (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenVar =>           (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenWhile =>         (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenError =>         (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenEof =>           (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
        }
                    
    }

    fn error(&mut self, message: &str) {
        self.error_at(&self.previous, message);
    }

    fn error_at(&mut self, token: &Token, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        print!("[line {} Error", token.line);

        if token.token_type == TokenType::TokenEof {
            print!(" at end");
        } else if token.token_type == TokenType::TokenError {

        } else {
            print!("at {}", token.content)
        }

        println!(": {}", message);
        self.had_error = true;
    }

    fn advance(&mut self) {
        self.previous = self.current;

        loop {
            self.current = self.scanner.scan_token();

            if self.current.token_type == TokenType::TokenError {
                break;
            }

            self.error_at_current("TODO: implement source substring to token");
        }
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(&self.current, message);
    }

    pub fn compile(&mut self, source: &str) -> bool {
        let mut scanner = Scanner::new(source.into());
        self.advance();
        self.expression();
        self.consume(TokenType::TokenEof, "Expect end of expression.");
        return !self.had_error;
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.current.token_type == token_type {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    fn emit_byte(&mut self, byte: u8) {
        self.current_chunk().write_chunk(byte, self.previous.line);
    }

    fn current_chunk(&mut self) -> &Chunk {
        &self.compiling_chunk
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn emit_return(&mut self) {
        self.emit_byte(map_opcode_to_binary(OpCode::OpReturn));
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::PrecAssignment);
    }
    
    fn number(&mut self) {
        let value : Value  = self.previous.content.parse().unwrap();
        self.emit_constant(value);
    }

    fn emit_constant(&mut self, value: Value) {
        self.emit_bytes(map_opcode_to_binary(OpCode::OpConstant), self.make_constant(value));
    }

    pub fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.current_chunk().add_constant(value);
        if constant > (u8::MAX) as usize {
            self.error("Too many constants in one chunk.");
            return 0u8;
        }

        return constant as u8;
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::TokenRightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self) {
        let operator_type: TokenType = self.previous.token_type;

        self.parse_precedence(Precedence::PrecUnary);

        match operator_type {
            TokenType::TokenMinus => self.emit_byte(map_opcode_to_binary(OpCode::OpNegate)),
            _ => {}
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {

    }

    fn binary(&mut self) {
        let operator_type: TokenType = self.previous.token_type;

        let (prefix, infix, precedence) = Parser::get_rule(operator_type);

        self.parse_precedence(get_next_rule(precedence));

        match operator_type {
            TokenType::TokenPlus => self.emit_byte(map_opcode_to_binary(OpCode::OpAdd)),
            TokenType::TokenMinus => self.emit_byte(map_opcode_to_binary(OpCode::OpSubtract)),
            TokenType::TokenStar => self.emit_byte(map_opcode_to_binary(OpCode::OpMultiply)),
            TokenType::TokenSlash => self.emit_byte(map_opcode_to_binary(OpCode::OpDivide)),
            _ => {}
        }
    }
    
}