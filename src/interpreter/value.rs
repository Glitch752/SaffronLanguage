pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Char(char),
    Vector(Vec<Value>)
}

impl Default for Value {
    fn default() -> Self {
        Value::Number(0.0)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Boolean(b) => if *b {
                write!(f, "true")
            } else {
                write!(f, "false")
            },
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Char(c) => write!(f, "{}", c),
            Value::Vector(vec) => {
                write!(f, "[")?;
                for value in vec {
                    write!(f, "{}, ", value)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(l), Value::Number(r)) => l == r,
            (Value::String(l), Value::String(r)) => l == r,
            (Value::Boolean(l), Value::Boolean(r)) => l == r,
            (Value::Char(l), Value::Char(r)) => l == r,
            (Value::Vector(l), Value::Vector(r)) => l == r,
            _ => false,
        }
    }
}