from .lexemes import Lexemes, keywords, symbols

class Token:
    def __init__(self, token_type: str, value: str = "") -> None:
        self.token_type: str = token_type
        self.value: str = value
    
    def __repr__(self) -> str:
        if self.value != "":
            return "{ Type: " + self.token_type + ", Value: " + self.value + " }"
        return "{ Type: " + self.token_type + " }"

class Lexer:
    def __init__(self, stream: str):
        self.stream: str = stream + "\n"
        self.current: int = 0

        self.tokens: list[Token] = []
    
    def lex(self):
        while self.current < len(self.stream) - 1:
            character: str = self.stream[self.current]

            check_symbol = False

            if character.isalpha():
                identifier: str = character
                while self.current < len(self.stream) - 1:
                    self.current += 1
                    next_char: str = self.stream[self.current]                    

                    if next_char.isalnum() or next_char == '_':
                        identifier += next_char
                    else:
                        character = next_char
                        check_symbol = True
                        break

                if identifier in keywords:
                    self.tokens.append(Token(keywords[identifier]))
                else:
                    self.tokens.append(Token(Lexemes.IDENTIFIER, identifier))
            elif character.isnumeric():
                number: str = character
                while self.current < len(self.stream) - 1:
                    self.current += 1
                    next_char: str = self.stream[self.current]

                    if next_char.isnumeric() or next_char == '.':
                        number += next_char
                    else:
                        character = next_char
                        check_symbol = True
                        break
                
                if '.' in number:
                    self.tokens.append(Token(Lexemes.FLOATLITERAL, number))
                else:
                    self.tokens.append(Token(Lexemes.INTLITERAL, number))
            elif character == "'":
                literal: str = ""
                while self.current < len(self.stream) - 1:
                    self.current += 1
                    next_char: str = self.stream[self.current]
                    if next_char == "'":
                        break
                    else:
                        literal += next_char
                self.tokens.append(Token(Lexemes.CHARLITERAL, literal))
            elif character == '"':
                literal: str = ""
                while self.current < len(self.stream) - 1:
                    self.current += 1
                    next_char: str = self.stream[self.current]
                    if next_char == '"':
                        break
                    else:
                        literal += next_char
                self.tokens.append(Token(Lexemes.STRINGLITERAL, literal))
            elif character == '/':
                next_char = self.stream[self.current + 1] if self.current + 1 <= len(self.stream) - 1 else ""
                
                if next_char == '/':
                    while self.current < len(self.stream) - 1:
                        self.current += 1
                        next_char = self.stream[self.current]
                        if next_char == '\n':
                            break
                else:
                    check_symbol = True
            else:
                check_symbol = True
            
            if check_symbol:
                next_char = self.stream[self.current + 1] if self.current + 1 <= len(self.stream) - 1 else ""
                if (character + next_char) in symbols:
                    self.tokens.append(Token(symbols[character + next_char]))
                elif character in symbols:
                    self.tokens.append(Token(symbols[character]))

            self.current += 1

        return self.tokens