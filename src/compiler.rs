use std::rc::Rc;

use crate::{chunk::{Chunk, OpCode, map_opcode_to_binary}, object::{Object, ObjectString}, scanner::{Scanner, Token, TokenType}, value::Value};

pub struct Parser {
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParseFn {
    Grouping,
    Unary,
    Binary,
    Number,
    None,
    Literal,
    String
}

struct ParseRule {
    prefix: ParseFn,
    infix: ParseFn,
    precedence: Precedence
}

impl Parser {

    pub fn get_compiling_chunk(self) -> Chunk {
        self.compiling_chunk
    }

    pub fn new(source: &str) -> Parser {
        Parser {
            current: Token::new_dummy_token(),
            previous: Token::new_dummy_token(),
            had_error: false,
            panic_mode: false,
            scanner: Scanner::new(source),
            compiling_chunk: Chunk::new(),
        }
    }

    fn get_rule(operator_type: TokenType) -> ParseRule {
        let (prefix, infix, precedence) = Parser::get_rule_tuple(operator_type);

        ParseRule {
            prefix: prefix,
            infix: infix,
            precedence: precedence
        }
    }

    fn get_rule_tuple(operator_type: TokenType) -> (ParseFn, ParseFn, Precedence) {
        match operator_type {
            TokenType::TokenLeftParen =>     (ParseFn::Grouping, ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenRightParen =>    (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenLeftBrace =>     (ParseFn::None,     ParseFn::None,   Precedence::PrecNone), 
            TokenType::TokenRightBrace =>    (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenComma =>         (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenDot =>           (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenMinus =>         (ParseFn::Unary,    ParseFn::Binary, Precedence::PrecTerm),
            TokenType::TokenPlus =>          (ParseFn::None,     ParseFn::Binary, Precedence::PrecTerm),
            TokenType::TokenSemicolon =>     (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenSlash =>         (ParseFn::None,     ParseFn::Binary, Precedence::PrecFactor),
            TokenType::TokenStar =>          (ParseFn::None,     ParseFn::Binary, Precedence::PrecFactor),
            TokenType::TokenBang =>          (ParseFn::Unary,    ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenBangEqual =>     (ParseFn::None,     ParseFn::Binary, Precedence::PrecEquality),
            TokenType::TokenEqual =>         (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenEqualEqual =>    (ParseFn::None,     ParseFn::Binary, Precedence::PrecEquality),
            TokenType::TokenGreater =>       (ParseFn::None,     ParseFn::Binary, Precedence::PrecComparison),
            TokenType::TokenGreaterEqual =>  (ParseFn::None,     ParseFn::Binary, Precedence::PrecComparison),
            TokenType::TokenLess =>          (ParseFn::None,     ParseFn::Binary, Precedence::PrecComparison),
            TokenType::TokenLessEqual =>     (ParseFn::None,     ParseFn::Binary, Precedence::PrecComparison),
            TokenType::TokenIdentifier =>    (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenString =>        (ParseFn::String,   ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenNumber =>        (ParseFn::Number,   ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenAnd =>           (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenClass =>         (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenElse =>          (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenFalse =>         (ParseFn::Literal,  ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenFor =>           (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenFun =>           (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenIf =>            (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenNil =>           (ParseFn::Literal,  ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenOr =>            (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenPrint =>         (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenReturn =>        (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenSuper =>         (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenThis =>          (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenTrue =>          (ParseFn::Literal,  ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenVar =>           (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenWhile =>         (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenError =>         (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
            TokenType::TokenEof =>           (ParseFn::None,     ParseFn::None,   Precedence::PrecNone),
        }
                    
    }

    fn error(&mut self, message: &str) {
        self.error_at(&self.previous.clone(), message);
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
            print!(" at {}", token.content)
        }

        println!(": {}", message);
        self.had_error = true;
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();

        loop {
            self.current = self.scanner.scan_token();
            //println!("token read is {:?}", &self.current);

            if self.current.token_type != TokenType::TokenError {
                break;
            }

            self.error_at_current( &self.current.content.clone());
        }
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(&self.current.clone(), message);
    }

    pub fn compile(&mut self) -> bool {
        self.advance();
        self.expression();
        self.consume(TokenType::TokenEof, "Expect end of expression.");
        self.end_compiler();
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
        let line = self.previous.line;
        self.current_chunk().write_chunk(byte, line);
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.compiling_chunk
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
        let value : f64  = self.previous.content.parse().unwrap();
        self.emit_constant(Value::Number(value));
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(map_opcode_to_binary(OpCode::OpConstant), constant);
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
            TokenType::TokenBang => self.emit_byte(map_opcode_to_binary(OpCode::OpNot)),
            TokenType::TokenMinus => self.emit_byte(map_opcode_to_binary(OpCode::OpNegate)),
            _ => {}
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let rule = Parser::get_rule(self.previous.token_type);

        if rule.prefix == ParseFn::None {
            self.error("Expect expression.");
            return;
        }
      
        self.run_rule(rule.prefix);

        while precedence <= Parser::get_rule(self.current.token_type).precedence {
            self.advance();
            let infix= Parser::get_rule(self.previous.token_type).infix;
            self.run_rule(infix);
        }
    }

    fn string(&mut self) {
        let string = Box::new(self.previous.content.clone());
        let string_obj = ObjectString { string: Rc::new(string) };
        let value= Value::Object(Object::ObjString(string_obj));
        self.emit_constant(value);
    }

    fn literal(&mut self) {
        match self.previous.token_type {
            TokenType::TokenFalse => { self.emit_byte(map_opcode_to_binary(OpCode::OpFalse)); }
            TokenType::TokenNil => { self.emit_byte(map_opcode_to_binary(OpCode::OpNil)); }
            TokenType::TokenTrue => { self.emit_byte(map_opcode_to_binary(OpCode::OpTrue)); }
            _ => { }
        }
    }

    fn run_rule(&mut self, rule: ParseFn) {
        match rule {
            ParseFn::Binary => { self.binary(); },
            ParseFn::Grouping => { self.grouping(); },
            ParseFn::Number => { self.number(); },
            ParseFn::Unary => { self.unary(); },
            ParseFn::Literal => { self.literal(); }
            ParseFn::String => { self.string(); }
            ParseFn::None => { panic!(); }
        }
    }

    fn binary(&mut self) {
        let operator_type: TokenType = self.previous.token_type;

        let rule= Parser::get_rule(operator_type);

        self.parse_precedence(get_next_rule(rule.precedence));

        match operator_type {
            TokenType::TokenBangEqual => self.emit_bytes(map_opcode_to_binary(OpCode::OpEqual), map_opcode_to_binary(OpCode::OpNot)),
            TokenType::TokenEqualEqual => self.emit_byte(map_opcode_to_binary(OpCode::OpEqual)),
            TokenType::TokenGreater => self.emit_byte(map_opcode_to_binary(OpCode::OpGreater)),
            TokenType::TokenGreaterEqual => self.emit_bytes(map_opcode_to_binary(OpCode::OpLess), map_opcode_to_binary(OpCode::OpNot)),
            TokenType::TokenLess => self.emit_byte(map_opcode_to_binary(OpCode::OpLess)),
            TokenType::TokenLessEqual => self.emit_bytes(map_opcode_to_binary(OpCode::OpGreater), map_opcode_to_binary(OpCode::OpNot)),
            TokenType::TokenPlus => self.emit_byte(map_opcode_to_binary(OpCode::OpAdd)),
            TokenType::TokenMinus => self.emit_byte(map_opcode_to_binary(OpCode::OpSubtract)),
            TokenType::TokenStar => self.emit_byte(map_opcode_to_binary(OpCode::OpMultiply)),
            TokenType::TokenSlash => self.emit_byte(map_opcode_to_binary(OpCode::OpDivide)),
            _ => {}
        }
    }
    
}