use crate::{
    chunk::{map_opcode_to_binary, Chunk, OpCode},
    scanner::{Scanner, Token, TokenType},
    value::Value,
    vm::VM,
};

pub struct Parser {
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
    scanner: Scanner,
    compiling_chunk: Chunk,
    compiler: Compiler,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    PrecNone,
    PrecAssignment, // =
    PrecOr,         // Or
    PrecAnd,        // And
    PrecEquality,   // == !=
    PrecComparison, // < > <= >=
    PrecTerm,       // + -
    PrecFactor,     // * /
    PrecUnary,      // ! -
    PrecCall,       // . ()
    PrecPrimary,
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
    String,
    Variable,
    And,
    Or,
}

#[derive(Debug)]
struct ParseRule {
    prefix: ParseFn,
    infix: ParseFn,
    precedence: Precedence,
}

const UINT8_COUNT: usize = (u8::MAX as usize) + 1;
const DEFAULT_TOKEN: Token = Token::new_dummy_token();
const DEFAULT_LOCAL: Local = Local {
    depth: 0,
    name: DEFAULT_TOKEN,
};

struct Compiler {
    local_count: isize,
    scope_depth: isize,
    locals: [Local; UINT8_COUNT],
}

impl Compiler {
    fn new() -> Compiler {
        let array = [DEFAULT_LOCAL; UINT8_COUNT];
        Compiler {
            local_count: 0,
            scope_depth: 0,
            locals: array,
        }
    }
}

struct Local {
    name: Token,
    depth: isize,
}

impl Default for Local {
    fn default() -> Self {
        Local {
            depth: 0,
            name: Token::new_dummy_token(),
        }
    }
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
            compiler: Compiler::new(),
        }
    }

    fn get_rule(operator_type: TokenType) -> ParseRule {
        let (prefix, infix, precedence) = Parser::get_rule_tuple(operator_type);

        ParseRule {
            prefix: prefix,
            infix: infix,
            precedence: precedence,
        }
    }

    fn get_rule_tuple(operator_type: TokenType) -> (ParseFn, ParseFn, Precedence) {
        match operator_type {
            TokenType::TokenLeftParen => (ParseFn::Grouping, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenRightParen => (ParseFn::None, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenLeftBrace => (ParseFn::None, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenRightBrace => (ParseFn::None, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenComma => (ParseFn::None, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenDot => (ParseFn::None, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenMinus => (ParseFn::Unary, ParseFn::Binary, Precedence::PrecTerm),
            TokenType::TokenPlus => (ParseFn::None, ParseFn::Binary, Precedence::PrecTerm),
            TokenType::TokenSemicolon => (ParseFn::None, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenSlash => (ParseFn::None, ParseFn::Binary, Precedence::PrecFactor),
            TokenType::TokenStar => (ParseFn::None, ParseFn::Binary, Precedence::PrecFactor),
            TokenType::TokenBang => (ParseFn::Unary, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenBangEqual => (ParseFn::None, ParseFn::Binary, Precedence::PrecEquality),
            TokenType::TokenEqual => (ParseFn::None, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenEqualEqual => (ParseFn::None, ParseFn::Binary, Precedence::PrecEquality),
            TokenType::TokenGreater => (ParseFn::None, ParseFn::Binary, Precedence::PrecComparison),
            TokenType::TokenGreaterEqual => (ParseFn::None, ParseFn::Binary, Precedence::PrecComparison),
            TokenType::TokenLess => (ParseFn::None, ParseFn::Binary, Precedence::PrecComparison),
            TokenType::TokenLessEqual => (ParseFn::None, ParseFn::Binary, Precedence::PrecComparison),
            TokenType::TokenIdentifier => (ParseFn::Variable, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenString => (ParseFn::String, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenNumber => (ParseFn::Number, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenAnd => (ParseFn::None, ParseFn::And, Precedence::PrecAnd),
            TokenType::TokenClass => (ParseFn::None, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenElse => (ParseFn::None, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenFalse => (ParseFn::Literal, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenFor => (ParseFn::None, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenFun => (ParseFn::None, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenIf => (ParseFn::None, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenNil => (ParseFn::Literal, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenOr => (ParseFn::None, ParseFn::Or, Precedence::PrecOr),
            TokenType::TokenPrint => (ParseFn::None, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenReturn => (ParseFn::None, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenSuper => (ParseFn::None, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenThis => (ParseFn::None, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenTrue => (ParseFn::Literal, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenVar => (ParseFn::None, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenWhile => (ParseFn::None, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenError => (ParseFn::None, ParseFn::None, Precedence::PrecNone),
            TokenType::TokenEof => (ParseFn::None, ParseFn::None, Precedence::PrecNone),
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

            self.error_at_current(&self.current.content.clone());
        }
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(&self.current.clone(), message);
    }

    pub fn compile(&mut self, vm: &mut VM) -> bool {
        self.advance();

        while !self.match_token(TokenType::TokenEof) {
            self.declaration(vm);
        }

        self.end_compiler(vm);
        return !self.had_error;
    }

    fn declaration(&mut self, vm: &mut VM) {
        if self.match_token(TokenType::TokenVar) {
            self.var_declaration(vm);
        } else {
            self.statement(vm);
        }

        if self.panic_mode {
            self.synchronize(vm);
        }
    }

    fn var_declaration(&mut self, vm: &mut VM) {
        let global = self.parse_variable("Expect variable name.", vm);

        if self.match_token(TokenType::TokenEqual) {
            self.expression(vm);
        } else {
            self.emit_byte(map_opcode_to_binary(OpCode::OpNil));
        }

        self.consume(
            TokenType::TokenSemicolon,
            "Expect ';' after variable declaration.",
        );

        self.define_variable(global);
    }

    fn parse_variable(&mut self, message: &str, vm: &mut VM) -> u8 {
        self.consume(TokenType::TokenIdentifier, message);
        self.declare_variable(vm);

        if self.compiler.scope_depth > 0 {
            return 0;
        }

        return self.identifier_constant(&self.previous.clone(), vm);
    }

    fn define_variable(&mut self, global: u8) {
        if self.compiler.scope_depth > 0 {
            self.mark_initialized();
            return;
        }

        self.emit_bytes(map_opcode_to_binary(OpCode::OpDefineGlobal), global);
    }

    fn mark_initialized(&mut self) {
        self.compiler.locals[(self.compiler.local_count - 1) as usize].depth =
            self.compiler.scope_depth;
    }

    fn identifier_constant(&mut self, token: &Token, vm: &mut VM) -> u8 {
        let obj = vm.get_or_create_string_object(&token.content);
        return self.make_constant(obj);
    }

    fn declare_variable(&mut self, vm: &mut VM) {
        if self.compiler.scope_depth == 0 {
            return;
        }

        let name = self.previous.clone();
        let mut i = self.compiler.local_count - 1;

        while i >= 0 {
            let local = &self.compiler.locals[i as usize];

            if local.depth != -1isize && local.depth < self.compiler.scope_depth {
                break;
            }

            if Parser::identifiers_equal(&name, &local.name) {
                self.error("Already a variable with this name in this scope.");
            }

            i = i - 1;
        }

        self.add_local(name, vm);
    }

    fn identifiers_equal(a: &Token, b: &Token) -> bool {
        a.content == b.content
    }

    fn add_local(&mut self, name: Token, _: &mut VM) {
        if self.compiler.local_count == UINT8_COUNT as isize {
            self.error("Too many local variables in function.");
            return;
        }

        let local = &mut self.compiler.locals[self.compiler.local_count as usize];
        self.compiler.local_count = self.compiler.local_count + 1;

        local.name = name;
        local.depth = -1;
    }

    fn synchronize(&mut self, _: &mut VM) {
        self.panic_mode = false;

        while self.current.token_type != TokenType::TokenEof {
            if self.previous.token_type == TokenType::TokenSemicolon {
                return;
            }

            match self.current.token_type {
                TokenType::TokenClass => return,
                TokenType::TokenFun => return,
                TokenType::TokenVar => return,
                TokenType::TokenFor => return,
                TokenType::TokenIf => return,
                TokenType::TokenWhile => return,
                TokenType::TokenPrint => return,
                TokenType::TokenReturn => return,
                _ => {}
            }
        }
    }

    fn statement(&mut self, vm: &mut VM) {
        if self.match_token(TokenType::TokenPrint) {
            self.print_statement(vm);
        } else if self.match_token(TokenType::TokenFor) {
            self.for_statement(vm);
        } else if self.match_token(TokenType::TokenIf) {
            self.if_statement(vm);
        } else if self.match_token(TokenType::TokenWhile) {
            self.while_statement(vm);
        } else if self.match_token(TokenType::TokenLeftBrace) {
            self.begin_scope();
            self.block(vm);
            self.end_scope();
        } else {
            self.expression_statement(vm);
        }
    }

    fn for_statement(&mut self, vm: &mut VM) {
        self.begin_scope();
        self.consume(TokenType::TokenLeftParen, "Expect '(' after 'for'.");
        if self.match_token(TokenType::TokenSemicolon) {
            // No initializer.
        } else if self.match_token(TokenType::TokenVar) {
            self.var_declaration(vm);
        } else {
            self.expression_statement(vm);
        }
        
        let mut loop_start = self.current_chunk().count();
        let mut exit_jump: Option<usize> = None;
        if !self.match_token(TokenType::TokenSemicolon) {
            self.expression(vm);
            self.consume(TokenType::TokenSemicolon, "Expect ';' after loop condition.");

            // Jump out of the loop if the condition is false.
            exit_jump = Some(self.emit_jump(map_opcode_to_binary(OpCode::OpJumpIfFalse)));
            self.emit_byte(map_opcode_to_binary(OpCode::OpPop)); // Condition.
        }

        if !self.match_token(TokenType::TokenRightParen) {
            let body_jump = self.emit_jump(map_opcode_to_binary(OpCode::OpJump));
            let increment_start = self.current_chunk().count();
            self.expression(vm);
            self.emit_byte(map_opcode_to_binary(OpCode::OpPop));
            self.consume(TokenType::TokenRightParen, "Expect ')' after for clauses.");
            self.emit_loop(loop_start);
            loop_start = increment_start;
            self.patch_jump(body_jump);

        }

        self.statement(vm);
        self.emit_loop(loop_start);

        if exit_jump.is_some() {
            self.patch_jump(exit_jump.unwrap());
            self.emit_byte(map_opcode_to_binary(OpCode::OpPop));
        }
        self.end_scope();
    }

    fn while_statement(&mut self, vm: &mut VM) {
        let loop_start = self.current_chunk().count();
        self.consume(TokenType::TokenLeftParen, "Expect '(' after 'while'.");
        self.expression(vm);
        self.consume(TokenType::TokenRightParen, "Expect ')' after condition.");

        let exit_jump = self.emit_jump(map_opcode_to_binary(OpCode::OpJumpIfFalse));
        self.emit_byte(map_opcode_to_binary(OpCode::OpPop));
        self.statement(vm);
        self.emit_loop(loop_start);
        self.patch_jump(exit_jump);
        self.emit_byte(map_opcode_to_binary(OpCode::OpPop));
    }

    fn emit_loop(&mut self, loop_start: usize) {
        self.emit_byte(map_opcode_to_binary(OpCode::OpLoop));
        let offset = self.current_chunk().count() - loop_start + 2;
        if offset > u16::MAX as usize {
            self.error("Loop body too larger");
        }

        self.emit_byte(((offset >> 8) & 0xFF) as u8);
        self.emit_byte((offset & 0xFF) as u8);
    }

    fn if_statement(&mut self, vm: &mut VM) {
        self.consume(TokenType::TokenLeftParen, "Expect '(' after 'if'.");
        self.expression(vm);
        self.consume(TokenType::TokenRightParen, "Expect ')' after condition.");

        let then_jump = self.emit_jump(map_opcode_to_binary(OpCode::OpJumpIfFalse));
        self.emit_byte(map_opcode_to_binary(OpCode::OpPop));
        self.statement(vm);
        let else_jump = self.emit_jump(map_opcode_to_binary(OpCode::OpJump));
        self.patch_jump(then_jump);
        self.emit_byte(map_opcode_to_binary(OpCode::OpPop));

        if self.match_token(TokenType::TokenElse) {
            self.statement(vm);
        }
        self.patch_jump(else_jump);
    }

    fn emit_jump(&mut self, instruction: u8) -> usize {
        self.emit_byte(instruction);
        self.emit_byte(0xFF);
        self.emit_byte(0xFF);
        self.current_chunk().count() - 2
    }

    fn patch_jump(&mut self, offset: usize) {
        let jump = self.current_chunk().count() - offset - 2;
        if jump > u16::MAX as usize {
            self.error("Too much code to jump over.");
        }

        self.current_chunk().code[offset] = ((jump >> 8) & 0xFF) as u8;
        self.current_chunk().code[offset + 1] = (jump & 0xFF) as u8;
    }

    fn block(&mut self, vm: &mut VM) {
        while !self.check(TokenType::TokenRightBrace) && !self.check(TokenType::TokenEof) {
            self.declaration(vm);
        }

        self.consume(TokenType::TokenRightBrace, "Expect '}' after block.");
    }

    fn begin_scope(&mut self) {
        self.compiler.scope_depth = self.compiler.scope_depth + 1;
    }

    fn end_scope(&mut self) {
        self.compiler.scope_depth = self.compiler.scope_depth - 1;

        while self.compiler.local_count > 0
            && self.compiler.locals[(self.compiler.local_count - 1) as usize].depth
                > self.compiler.scope_depth
        {
            self.emit_byte(map_opcode_to_binary(OpCode::OpPop));
            self.compiler.local_count = self.compiler.local_count - 1;
        }
    }

    fn expression_statement(&mut self, vm: &mut VM) {
        self.expression(vm);
        self.consume(TokenType::TokenSemicolon, "Expect ';' after expression.");
        self.emit_byte(map_opcode_to_binary(OpCode::OpPop))
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if !self.check(token_type) {
            return false;
        }
        self.advance();
        true
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        self.current.token_type == token_type
    }

    fn print_statement(&mut self, vm: &mut VM) {
        self.expression(vm);
        self.consume(TokenType::TokenSemicolon, "Expect ';' after value.");
        self.emit_byte(map_opcode_to_binary(OpCode::OpPrint));
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.current.token_type == token_type {
            self.advance();
            return;
        }

        //println!("consume failed. Expected {:?}, got {:?}", token_type, self.current);

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

    fn end_compiler(&mut self, vm: &mut VM) {
        self.emit_return(vm);
    }

    fn emit_return(&mut self, _: &mut VM) {
        self.emit_byte(map_opcode_to_binary(OpCode::OpReturn));
    }

    fn expression(&mut self, vm: &mut VM) {
        self.parse_precedence(Precedence::PrecAssignment, vm);
    }

    fn number(&mut self, _: &mut VM) {
        let value: f64 = self.previous.content.parse().unwrap();
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

    fn grouping(&mut self, vm: &mut VM) {
        self.expression(vm);
        self.consume(TokenType::TokenRightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self, vm: &mut VM) {
        let operator_type: TokenType = self.previous.token_type;

        self.parse_precedence(Precedence::PrecUnary, vm);

        match operator_type {
            TokenType::TokenBang => self.emit_byte(map_opcode_to_binary(OpCode::OpNot)),
            TokenType::TokenMinus => self.emit_byte(map_opcode_to_binary(OpCode::OpNegate)),
            _ => {}
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence, vm: &mut VM) {
        self.advance();
        let rule = Parser::get_rule(self.previous.token_type);

        if rule.prefix == ParseFn::None {
            self.error("Expect expression.");
            return;
        }

        let can_assign = precedence <= Precedence::PrecAssignment;
        self.run_rule(rule.prefix, can_assign, vm);

        while precedence <= Parser::get_rule(self.current.token_type).precedence {
            self.advance();
            let new_rule = Parser::get_rule(self.previous.token_type);
            let infix = new_rule.infix;
            self.run_rule(infix, can_assign, vm);
        }

        if can_assign && self.match_token(TokenType::TokenEqual) {
            self.error("Invalid assignment target.");
        }
    }

    fn string(&mut self, vm: &mut VM) {
        let mut previous_str = self.previous.content.clone();
        previous_str.remove(0);
        previous_str.remove(previous_str.len() - 1);

        let value = vm.get_or_create_string_object(&previous_str);

        self.emit_constant(value);
    }

    fn variable(&mut self, can_assign: bool, vm: &mut VM) {
        self.named_variable(self.previous.clone(), can_assign, vm);
    }

    fn named_variable(&mut self, name: Token, can_assign: bool, vm: &mut VM) {
        let mut arg = self.resolve_local(&name, vm);

        let (get_op, set_op) = if arg != -1 {
            (OpCode::OpGetLocal, OpCode::OpSetLocal)
        } else {
            arg = self.identifier_constant(&name, vm) as isize;
            (OpCode::OpGetGlobal, OpCode::OpSetGlobal)
        };

        if can_assign && self.match_token(TokenType::TokenEqual) {
            self.expression(vm);
            self.emit_bytes(map_opcode_to_binary(set_op), arg as u8);
        } else {
            self.emit_bytes(map_opcode_to_binary(get_op), arg as u8);
        }
    }

    fn resolve_local(&mut self, name: &Token, _: &mut VM) -> isize {
        let mut i = self.compiler.local_count - 1;

        while i >= 0 {
            let local = &self.compiler.locals[i as usize];
            if Parser::identifiers_equal(name, &local.name) {
                if local.depth == -1 {
                    self.error("Can't read local variable in its own initializer.");
                }
                return i;
            }
            i = i - 1;
        }

        -1
    }

    fn literal(&mut self, _: &mut VM) {
        match self.previous.token_type {
            TokenType::TokenFalse => {
                self.emit_byte(map_opcode_to_binary(OpCode::OpFalse));
            }
            TokenType::TokenNil => {
                self.emit_byte(map_opcode_to_binary(OpCode::OpNil));
            }
            TokenType::TokenTrue => {
                self.emit_byte(map_opcode_to_binary(OpCode::OpTrue));
            }
            _ => {}
        }
    }

    fn and(&mut self, vm: &mut VM) {
        let end_jump = self.emit_jump(map_opcode_to_binary(OpCode::OpJumpIfFalse));
        self.emit_byte(map_opcode_to_binary(OpCode::OpPop));
        self.parse_precedence(Precedence::PrecAnd, vm);
        self.patch_jump(end_jump);
    }

    fn or(&mut self, vm: &mut VM) {
        let else_jump = self.emit_jump(map_opcode_to_binary(OpCode::OpJumpIfFalse));
        let end_jump = self.emit_jump(map_opcode_to_binary(OpCode::OpJump));

        self.patch_jump(else_jump);
        self.emit_byte(map_opcode_to_binary(OpCode::OpPop));
        self.parse_precedence(Precedence::PrecOr, vm);
        self.patch_jump(end_jump);
    }

    fn run_rule(&mut self, rule: ParseFn, can_assign: bool, vm: &mut VM) {
        match rule {
            ParseFn::Binary => {
                self.binary(vm);
            }
            ParseFn::Grouping => {
                self.grouping(vm);
            }
            ParseFn::Number => {
                self.number(vm);
            }
            ParseFn::Unary => {
                self.unary(vm);
            }
            ParseFn::Literal => {
                self.literal(vm);
            }
            ParseFn::String => {
                self.string(vm);
            }
            ParseFn::Variable => {
                self.variable(can_assign, vm);
            }
            ParseFn::None => {
                panic!();
            }
            ParseFn::And => {
                self.and(vm);
            }
            ParseFn::Or => {
                self.or(vm);
            }
        }
    }

    fn binary(&mut self, vm: &mut VM) {
        let operator_type: TokenType = self.previous.token_type;

        let rule = Parser::get_rule(operator_type);

        self.parse_precedence(get_next_rule(rule.precedence), vm);

        match operator_type {
            TokenType::TokenBangEqual => self.emit_bytes(
                map_opcode_to_binary(OpCode::OpEqual),
                map_opcode_to_binary(OpCode::OpNot),
            ),
            TokenType::TokenEqualEqual => self.emit_byte(map_opcode_to_binary(OpCode::OpEqual)),
            TokenType::TokenGreater => self.emit_byte(map_opcode_to_binary(OpCode::OpGreater)),
            TokenType::TokenGreaterEqual => self.emit_bytes(
                map_opcode_to_binary(OpCode::OpLess),
                map_opcode_to_binary(OpCode::OpNot),
            ),
            TokenType::TokenLess => self.emit_byte(map_opcode_to_binary(OpCode::OpLess)),
            TokenType::TokenLessEqual => self.emit_bytes(
                map_opcode_to_binary(OpCode::OpGreater),
                map_opcode_to_binary(OpCode::OpNot),
            ),
            TokenType::TokenPlus => self.emit_byte(map_opcode_to_binary(OpCode::OpAdd)),
            TokenType::TokenMinus => self.emit_byte(map_opcode_to_binary(OpCode::OpSubtract)),
            TokenType::TokenStar => self.emit_byte(map_opcode_to_binary(OpCode::OpMultiply)),
            TokenType::TokenSlash => self.emit_byte(map_opcode_to_binary(OpCode::OpDivide)),
            _ => {}
        }
    }
}
