
pub enum MathTerm<U, V, W> {
    Number(U),
    Fraction(U, V),
    Negation(U),
    Power(U, V),
    Sum(U, U),
    Product(U, V),
    Compex(U, U),
    Variable(String),
    Sin(U),
    Cos(U),
    Tan(U),
    Log(U, V),
    Derivation(U, String),
    Integral(U, V, W, String),
    Polynom(Vec<U>),
}

pub struct NaturalType {
    pub value: Vec<u8>,
}

pub struct NaturalZeroType {
    pub value: NaturalType,
    pub zero: bool,
}

pub struct IntegerType {
    pub value: NaturalZeroType,
    pub negative: bool,
}
