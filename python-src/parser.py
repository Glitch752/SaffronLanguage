from ast.expression import BlockNode, BoolLiteral, ExpressionNode, FloatLiteral, InfiniteLoopNode, IteratorLoopNode, VariableAccessNode, IfNode, IntLiteral, WhileLoopNode
from ast.statement import FunctionDeclaration, ReturnNode, StatementNode, VariableDeclaration
from .lexer import Token, Lexemes
from .visitor import Visitor

class ParserException(Exception):
    message: str
    token: Token
    
    def __init__(self, token: Token | None, message: str) -> None:
        super().__init__(message)
        self.token = token
        self.message = message

class AstNode:
    def visit(self, visitor: Visitor):
        # Should be implemented by subclasses
        print("TODO: Implement me")
        pass

class ProgramNode(AstNode):
    def __init__(self):
        self.statements: list[AstNode] = []
        
    def visit(self, visitor: Visitor):
        return visitor.visit_program(self)

class Parser:
    def __init__(self, tokens: list[Token]) -> None:
        self.tokens: list[Token] = tokens
        self.current: int = 0

        self.errors = []
        
    def peek(self) -> Token | None:
        return self.tokens[self.current] if self.current + 1 <= len(self.tokens) else None

    def consume(self) -> Token:
        if self.current <= len(self.tokens) - 1:
            tok: Token = self.tokens[self.current]
            self.current += 1
            return tok
        raise ParserException(self.peek(), "Unexpected end of file")
    
    def expect(self, token_type: str) -> Token:
        if self.current <= len(self.tokens) - 1:
            if self.peek().token_type == token_type:
                return self.consume()
            else:
                raise ParserException(self.peek(), f"Expected {token_type}, but got {self.peek().token_type}")
        
        raise ParserException(self.peek(), f"Expected {token_type}, but got EOF")
    
    def expect_identifier(self) -> str:
        if self.current <= len(self.tokens) - 1:
            if self.peek().token_type == Lexemes.IDENTIFIER:
                return self.consume().value
            else:
                raise ParserException(self.peek(), f"Expected identifier, but got {self.peek().token_type}")

        raise ParserException(self.peek(), "Expected identifier, but got EOF")
    
    def advance_if(self, token_type: str):
        if self.peek().token_type == token_type:
            return self.consume()
        
        raise ParserException(self.peek(), f"Unexpected end of file")
        
    def synchronize_to_statement(self):
        while self.current < len(self.tokens) - 1:
            tok: Token = self.peek()
            
            if tok.token_type in [Lexemes.SEMICOLON, Lexemes.CLOSECURLYBRACKET]:
                self.consume()
                return
            if tok.token_type in [Lexemes.CONST, Lexemes.LET, Lexemes.FUNC, Lexemes.RETURN, Lexemes.IF, Lexemes.IMPORT, Lexemes.LOOP, Lexemes.BREAK, Lexemes.CONTINUE]:
                return
            
            self.current += 1

    def parse(self) -> ProgramNode:
        program: ProgramNode = ProgramNode()

        while (self.current < len(self.tokens) - 1):
            try:
                s = self.parse_statement()
            except ParserException as e:
                self.errors.append(self.error_message)
                self.synchronize_to_statement()
                continue

            program.statements.append(s)
            self.expect(Lexemes.SEMICOLON)

        return program
    
    def parse_declaration(self) -> StatementNode:
        tok: Token = self.peek()

        if tok.token_type == Lexemes.CONST or tok.token_type == Lexemes.LET:
            return self.parse_variable_declaration()
        elif tok.token_type == Lexemes.FUNC:
            return self.parse_function_declaration()
        else:
            return self.parse_statement()
        
    def parse_statement(self) -> StatementNode:
        tok: Token = self.peek()
            
        if tok.token_type == Lexemes.BREAK:
            # TODO: Breaking with values
            return self.consume() # consume break
        elif tok.token_type == Lexemes.CONTINUE:
            return self.consume()
        elif tok.token_type == Lexemes.IMPORT:
            self.consume() # consume import
            e = self.parse_expression()
            return e
        elif tok.token_type == Lexemes.RETURN:
            next_tok = self.consume() # consume return
            node: ReturnNode = ReturnNode()

            if next_tok.token_type == Lexemes.SEMICOLON:
                return node
            
            node.expression = self.parse_expression()
            return node
        else:
            e = self.parse_expression()
            e.is_result = self.peek().token_type != Lexemes.SEMICOLON # is this a result?

            return e

    def parse_function_declaration(self) -> FunctionDeclaration:
        node: FunctionDeclaration = FunctionDeclaration()

        if n := self.expect(Lexemes.IDENTIFIER):
            node.name = n.value
            self.expect(Lexemes.OPENPARENTHESIS)
            
            while self.current < len(self.tokens) - 1:
                tok: Token = self.peek()
                
                if tok.token_type == Lexemes.IDENTIFIER:
                    v: VariableDeclaration = VariableDeclaration()
                    v.is_const = True
                    v.name = tok.value
                    self.expect(Lexemes.COLON)
                    v.variable_type = self.consume().value
                    v.expression = ExpressionNode()
                    node.params.append(v)

                if tok.token_type == Lexemes.CLOSEPARENTHESIS:
                    break
            
                self.current += 1

            self.expect(Lexemes.ARROW)
            node.return_type = self.expect(Lexemes.IDENTIFIER)
            self.consume()
            node.expression = self.parse_expression()
        
            return node

        raise ParserException(self.peek(), "Missing function name in function declaration")

    def parse_variable_declaration(self):
        node: VariableDeclaration = VariableDeclaration()
        node.is_const = self.peek().token_type == Lexemes.CONST

        if n := self.expect(Lexemes.IDENTIFIER):
            node.name = n.value

            self.expect(Lexemes.COLON)
            if ty := self.expect(Lexemes.IDENTIFIER):
                node.variable_type = ty.value
                self.expect(Lexemes.ASSIGNMENT)
                self.consume()
                node.expression = self.parse_expression()

                return node

        raise ParserException(self.peek(), "Missing identifier in variable declaration")

    def parse_expression(self):
        if self.peek().token_type == Lexemes.OPENCURLYBRACKET:
            return self.parse_block()
        
        if self.advance_if(Lexemes.IF):
            return self.parse_if()
        if self.advance_if(Lexemes.LOOP):
            return self.parse_loop()
        
        return self.parse_primary_or_lower()
    
    def parse_if(self):
        self.expect(Lexemes.OPENPARENTHESIS)
        self.consume()
        if_node: IfNode = IfNode()
        if_node.condition = self.parse_expression()
        self.expect(Lexemes.CLOSEPARENTHESIS)
        self.consume()
        if_node.then_branch = self.parse_expression()
        return if_node
    
    def parse_loop(self):
        self.expect(Lexemes.OPENPARENTHESIS)
        
        if self.peek() in [Lexemes.CONST, Lexemes.LET]:
            # Iterator loop
            is_const = self.consume() == Lexemes.CONST
            iterator = self.expect_identifier()
            self.expect(Lexemes.COLON)
            iterable = self.parse_expression()
            self.expect(Lexemes.CLOSEPARENTHESIS)
            body = self.parse_statement()
            return IteratorLoopNode(iterator, iterable, body, is_const)
        elif self.advance_if(Lexemes.CLOSEPARENTHESIS):
            # Infinite loop
            body = self.parse_statement()
            return InfiniteLoopNode(body)
        else:
            # While loop
            condition = self.parse_expression()
            self.expect(Lexemes.CLOSEPARENTHESIS)
            body = self.parse_statement()
            return WhileLoopNode(condition, body)

    def parse_assignment_or_lower(self):
        pass

    def parse_logical_or_or_lower(self):
        pass

    def parse_logical_and_or_lower(self):
        pass

    def parse_equality_or_lower(self):
        pass

    def parse_comparison_or_lower(self):
        pass

    def parse_primary_or_lower(self):
        match self.peek().token_type:
            case Lexemes.INTLITERAL:
                node = IntLiteral(int(self.peek().value))
                return node
            case Lexemes.FLOATLITERAL:
                node = FloatLiteral(float(self.peek().value))
                return node
            case Lexemes.TRUE:
                node = BoolLiteral(True)
                return node
            case Lexemes.FALSE:
                node = BoolLiteral(False)
                return node
            case Lexemes.IDENTIFIER:
                node = VariableAccessNode(self.peek().value)
                return node

    def parse_block(self):
        self.consume() # eat the {

        node: BlockNode = BlockNode()

        tok: Token = self.peek()

        while tok and tok.token_type != Lexemes.CLOSECURLYBRACKET:
            node.statements.append(self.parse_statement())
            tok: Token = self.consume()

        return node

def pretty_print(program):
    for statement in program.statements:
        if statement:
            print(statement.pretty(1))