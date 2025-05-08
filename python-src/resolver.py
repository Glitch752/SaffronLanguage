from .parser import ProgramNode
from .visitor import Visitor

class Resolver(Visitor):
    def __init__(self, program: ProgramNode):
        self.program = program
        self.visitor = Visitor()

        self.scopes: list[dict[str, bool]] = []

    def begin_scope(self):
        self.scopes.append({})
    
    def end_scope(self):
        self.scopes.pop()
    
    def declare(self, name: str):
        if len(self.scopes):
            self.scopes[-1].update({name: False})
        
    def define(self, name: str):
        if len(self.scopes):
            self.scopes[-1].update({name: True})

    def resolve(self):
        self.program.visit(self.visitor)