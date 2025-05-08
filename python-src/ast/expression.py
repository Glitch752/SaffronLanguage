from parser import AstNode

class ExpressionNode(AstNode):
    def __init__(self):
        self.is_result: bool = False

    def pretty(self, tab):
        return ""
    
    def visit(self, visitor):
        return visitor.visit_expression(self)

class BinaryExpression(ExpressionNode):
    def __init__(self):
        self.left: AstNode = None
        self.operator: str = ""
        self.right: AstNode = None

class BoolLiteral(ExpressionNode):
    def __init__(self):
        self.value: bool = False
    
    def pretty(self, tab):
        return "{ " + f"Bool Literal: {self.value}" + " }"

class IntLiteral(ExpressionNode):
    def __init__(self, value):
        self.value: int = value
    
    def pretty(self, tab):
        return "{ " + f"Int Literal: {self.value}" + " }"
    
class FloatLiteral(ExpressionNode):
    def __init__(self, value):
        self.value: float = value
    
    def pretty(self, tab):
        return "{ " + f"Float Literal: {self.value}" + " }"
    
class CharLiteral(ExpressionNode):
    def __init__(self, value):
        self.value: str = value
    
    def pretty(self, tab):
        return "{ " + f"Char Literal: {self.value}" + " }"
    
class StringLiteral(ExpressionNode):
    def __init__(self, value):
        self.value: str = value
    
    def pretty(self, tab):
        return "{ " + f"String Literal: {self.value}" + " }"

class IdentifierNode(ExpressionNode):
    def __init__(self, name):
        self.name: str = name
    
    def pretty(self, tab):
        return f"Identifier:\n{'| '*tab}{self.name}"

class IfNode(ExpressionNode):
    def __init__(self):
        self.condition: ExpressionNode = None
        self.then_branch: ExpressionNode = None
        self.else_branch: ExpressionNode = None

    def pretty(self, tab):
        return f"If Node:\n{'| '*tab}Condition: {self.condition.pretty(tab + 1)}\n{'| '*tab}Then: {self.then_branch.pretty(tab + 1)}\n{'| '*tab}Else: {self.else_branch.pretty(tab+1) if self.else_branch else 'none'}"

class LoopNode(ExpressionNode):
    def __init__(self):
        self.loop_condition: ExpressionNode = None
        self.branch: ExpressionNode = None

class BlockNode(ExpressionNode):
    def __init__(self):
        self.statements: list[AstNode] = []
    
    def pretty(self, tab):
        string = f"Block:\n{'| '*tab}"

        for expr in self.statements:
            if expr:
                string += str(expr.pretty(tab + 1)) + f"\n{'| '*tab}"
        
        return string