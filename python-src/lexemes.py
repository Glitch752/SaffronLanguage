class Lexemes:
    # keywords
    
    IF = 'ifKeyword'
    ELSE = 'elseKeyword'
    LOOP = 'loopKeyword'
    FUNC = 'funcKeyword'
    IMPORT = 'importKeyword'
    RETURN = 'returnKeyword'
    LET = 'letKeyword'
    CONST = 'constKeyword'

    # operators / symbols

    PLUS = 'plusOperator'
    MINUS = 'minusOperator'
    MULTIPLY = 'multiplyOperator'
    DIVIDE = 'divideOperator'
    MODULO = 'moduloOperator'

    ASSIGNMENT = 'assignmentOperator'
    COMPARISON = 'comparisonOperator'

    SEMICOLON = 'semicolonSymbol'
    COLON = 'colonSymbol'
    DOT = 'dotSymbol'
    COMMA = 'commaSymbol'

    AND = 'andSymbol'
    OR = 'orSymbol'
    NOT = 'notSymbol'
    ARROW = 'arrowSymbol'

    OPENPARENTHESIS = 'openParenthesis'
    CLOSEPARENTHESIS = 'closeParenthesis'
    OPENSQUAREBRACKET = 'openSquareBracket'
    CLOSESQUAREBRACKET = 'closeSquareBracket'
    OPENCURLYBRACKET = 'openCurlyBracket'
    CLOSECURLYBRACKET = 'closeCurlyBracket'

    # literals

    CHARLITERAL = 'charLiteral'
    STRINGLITERAL = 'stringLiteral'
    INTLITERAL = 'intLiteral'
    FLOATLITERAL = 'floatLiteral'

    # other
    IDENTIFIER = 'identifier'
    TRUE = 'trueValue'
    FALSE = 'falseValue'

keywords = {
    'if': Lexemes.IF,
    'else': Lexemes.ELSE,
    'loop': Lexemes.LOOP,
    'func': Lexemes.FUNC,
    'import': Lexemes.IMPORT,
    'return': Lexemes.RETURN,
    'let': Lexemes.LET,
    'const': Lexemes.CONST,
    'true': Lexemes.TRUE,
    'false': Lexemes.FALSE
}

symbols = {
    '(': Lexemes.OPENPARENTHESIS,
    ')': Lexemes.CLOSEPARENTHESIS,
    '[': Lexemes.OPENSQUAREBRACKET,
    ']': Lexemes.CLOSESQUAREBRACKET,
    '{': Lexemes.OPENCURLYBRACKET,
    '}': Lexemes.CLOSECURLYBRACKET,
    '.': Lexemes.DOT,
    ',': Lexemes.COMMA,
    ';': Lexemes.SEMICOLON,
    ':': Lexemes.COLON,
    '+': Lexemes.PLUS,
    '-': Lexemes.MINUS,
    '*': Lexemes.MULTIPLY,
    '/': Lexemes.DIVIDE,
    '%': Lexemes.MODULO,
    '&&': Lexemes.AND,
    '||': Lexemes.OR,
    '!': Lexemes.NOT,
    '->': Lexemes.ARROW,
    '=': Lexemes.ASSIGNMENT,
    '==': Lexemes.COMPARISON
}