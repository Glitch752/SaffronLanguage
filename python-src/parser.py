from ast.expression import BlockNode, BoolLiteral, ExpressionNode, FloatLiteral, IdentifierNode, IfNode, IntLiteral
from ast.staterment import FunctionDeclaration, ReturnNode, VariableDeclaration
from .lexer import Token, Lexemes

class AstNode:
    def visit(self, visitor):
        print("TODO: Implement me")
        pass

class ProgramNode(AstNode):
    def __init__(self):
        self.statements: list[AstNode] = []
        
    def visit(self, visitor):
        return visitor.visit_program(self)

class Parser:
    def __init__(self, tokens: list[Token]) -> None:
        self.tokens: list[Token] = tokens
        self.current: int = 0

        self.errors = []

        self.error_message = "error"
        
    def synchronize_to_statement(self):
        while self.current < len(self.tokens) - 1:
            tok: Token = self.tokens[self.current]
            
            if tok.token_type in [Lexemes.SEMICOLON, Lexemes.CLOSECURLYBRACKET]:
                self.consume()
                return
            if tok.token_type in [Lexemes.CONST, Lexemes.LET, Lexemes.FUNC, Lexemes.RETURN, Lexemes.IF, Lexemes.IMPORT, Lexemes.LOOP, Lexemes.BREAK, Lexemes.CONTINUE]:
                return
            
            self.current += 1
            

    def parse(self):
        program: ProgramNode = ProgramNode()

        while (self.current < len(self.tokens) - 1):
            s = self.parse_statement()

            if s == None:
                self.errors.append(self.error_message)
                self.synchronize_to_statement()
                continue

            program.statements.append(s)
            
            self.consume()

        return program

    def parse_statement(self):
        tok: Token = self.tokens[self.current]
            
        if tok.token_type == Lexemes.CONST or tok.token_type == Lexemes.LET:
            return self.parse_variable_declaration()
        elif tok.token_type == Lexemes.FUNC:
            return self.parse_function_declaration()
        elif tok.token_type == Lexemes.IF:
            if not self.expect(Lexemes.OPENPARENTHESIS):
                self.error_message = "error: missing '(' in variable declaration"
                return None
            self.consume()
            if_node: IfNode = IfNode()
            if_node.condition = self.parse_expression()
            if not self.expect(Lexemes.CLOSEPARENTHESIS):
                self.error_message = "error: missing ')' in variable declaration"
                return None
            self.consume()
            if_node.then_branch = self.parse_expression()
            return if_node
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
            
            if e:
                e.is_result = self.tokens[self.current].token_type != Lexemes.SEMICOLON # is this a result?

            return e

    def parse_function_declaration(self):
        node: FunctionDeclaration = FunctionDeclaration()

        if n := self.expect(Lexemes.IDENTIFIER):
            node.name = n.value

            if not self.expect(Lexemes.OPENPARENTHESIS):
                self.error_message = "error: missing '(' in function declaration"
                return None
            
            while self.current < len(self.tokens) - 1:
                tok: Token = self.tokens[self.current]
                
                if tok.token_type == Lexemes.IDENTIFIER:
                    v: VariableDeclaration = VariableDeclaration()
                    
                    v.is_const = True
                    v.name = tok.value
                    
                    if not self.expect(Lexemes.COLON):
                        self.error_message = "error: missing ':' in function params"
                        return None
                    
                    v.variable_type = self.consume().value

                    v.expression = ExpressionNode()

                    node.params.append(v)

                if tok.token_type == Lexemes.CLOSEPARENTHESIS:
                    break
            
                self.current += 1

            if not self.expect(Lexemes.ARROW):
                self.error_message = "error: missing '->' in function delcaration"
                return None
            
            node.return_type = self.expect(Lexemes.IDENTIFIER)

            if not node.return_type:
                self.error_message = "error: missing return type"
            
            self.consume()

            node.expression = self.parse_expression()
        
            return node

        self.error_message = "error: missing identifier"
        return None

    def parse_variable_declaration(self):
        node: VariableDeclaration = VariableDeclaration()

        node.is_const = self.tokens[self.current].token_type == Lexemes.CONST

        if n := self.expect(Lexemes.IDENTIFIER):
            node.name = n.value

            if not self.expect(Lexemes.COLON):
                self.error_message = "error: missing ':' in variable declaration"
                return None
            
            if ty := self.expect(Lexemes.IDENTIFIER):
                node.variable_type = ty.value

                if not self.expect(Lexemes.ASSIGNMENT):
                    self.error_message = "error: missing '=' in variable declaration"

                self.consume()

                node.expression = self.parse_expression()

                return node

        self.error_message = "error: missing identifier"
        return None

    def parse_expression(self):
        if self.tokens[self.current].token_type == Lexemes.OPENCURLYBRACKET:
            return self.parse_block()
        
        return self.parse_primary_or_lower()

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
        match self.tokens[self.current].token_type:
            case Lexemes.INTLITERAL:
                node = IntLiteral(int(self.tokens[self.current].value))
                return node
            case Lexemes.FLOATLITERAL:
                node = FloatLiteral(float(self.tokens[self.current].value))
                return node
            case Lexemes.TRUE:
                node = BoolLiteral(True)
                return node
            case Lexemes.FALSE:
                node = BoolLiteral(False)
                return node
            case Lexemes.IDENTIFIER:
                node = IdentifierNode(self.tokens[self.current].value)
                return node

    def parse_block(self):
        self.consume() # eat the {

        node: BlockNode = BlockNode()

        tok: Token = self.tokens[self.current]

        while tok and tok.token_type != Lexemes.CLOSECURLYBRACKET:
            node.statements.append(self.parse_statement())
            tok: Token = self.consume()

        return node

    def consume(self):
        self.current += 1

        if self.current <= len(self.tokens) - 1: return self.tokens[self.current]

        return None
    
    def expect(self, token_type: str):
        self.current += 1

        if self.current <= len(self.tokens) - 1:
            if self.tokens[self.current].token_type == token_type:
                return self.tokens[self.current]
        
        return None

def pretty_print(program):
    for statement in program.statements:
        if statement:
            print(statement.pretty(1))