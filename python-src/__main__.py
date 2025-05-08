import os, sys
from .lexer import Lexer
from .parser import Parser, pretty_print
from .resolver import *
from .interpreter import *

if __name__ == "__main__":
    if len(sys.argv) > 1:
        if os.path.exists(sys.argv[1]):
            lexer = Lexer(open(sys.argv[1]).read())
            parser = Parser(lexer.lex())

            pretty_print(parser.parse())
        else:
            print("error: invalid file input")
    else:
        print("error: expected file input")