use crate::scratch::ast::instruction::{Value, GetInstruction};

pub enum OperatorInstruction<'a> {
    Id(GetInstruction<'a>),
    UnaryOp(UnaryOp, Value<'a>),
    BinaryOp(BinaryOp, Value<'a>, Value<'a>),
}

enum UnaryOp {
    VectorIndex(u8),
    Not(),
    Abs(),
    FloatToInt(FloatToIntOp),
    FloatToFloat(FloatToFloatOp),
}

enum FloatToIntOp {
    Round,
    Floor,
    Ceiling,
}

enum FloatToFloatOp {
    Sqrt,
    Sin,
    Cos,
    Tan,
    ASin,
    ACos,
    ATan,
    Ln,
    Log,
    Exp,
}

enum BinaryOp {
    Math(MathOp),
    Comparison(ComparisonOp),
    Logic(LogicOp),
}

enum MathOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Pow,
    Random,
    Distance,
}

enum ComparisonOp {
    LessThan,
    GreaterThan,
    Equals,
}

enum LogicOp {
    And,
    Or,
}
