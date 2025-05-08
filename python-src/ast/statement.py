from ast.expression import ExpressionNode
from parser import AstNode
from visitor import Visitor

class StatementNode(AstNode):
    def __init__(self):
        pass
    def visit(self, visitor: Visitor):
        return visitor.visit_statement(self)
    
class ExpressionStatement(StatementNode):
    def __init__(self, expression: ExpressionNode):
        self.expression: ExpressionNode = expression
    
    def pretty(self, tab):
        return f"Expr Statement:\n{'| '*tab}Expr: {self.expression.pretty(tab+1) if self.expression else 'none'}"

class ReturnNode(StatementNode):
    def __init__(self):
        self.expression: ExpressionNode = None

    def pretty(self, tab):
        return f"Return:\n{'| '*tab}Expr: {self.expression.pretty(tab+1) if self.expression else 'none'}"

class VariableDeclaration(StatementNode):
    def __init__(self):
        self.name: str = ""
        self.variable_type: str = ""
        self.expression: ExpressionNode = None
        self.is_const: bool = False

    def pretty(self, tab):
        return f"Variable Decl:\n{'| '*tab}Name: {self.name}\n{'| '*tab}Type: {self.variable_type}\n{'| '*tab}Expr: {str(self.expression.pretty(tab+1))}\n{'| '*tab}Const?: {self.is_const}"

class FunctionDeclaration(StatementNode):
    def __init__(self):
        self.name: str = ""
        self.params: list[VariableDeclaration] = []
        self.return_type: str = ""
        self.expression: ExpressionNode = None

    def pretty(self, tab):
        return f"Func Decl:\n{'| '*tab}Name: {self.name}\n{'| '*tab}Params: {('\n' + '| ' * (tab + 1)).join([param.pretty(tab + 1) for param in self.params])}\n{'| '*tab}Return Type: {self.return_type}\n{'| '*tab}Expr: {str(self.expression.pretty(tab+1))}\n{'| '*tab}"
