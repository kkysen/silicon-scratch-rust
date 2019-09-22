use crate::scratch::ast::instruction::{Value, GetInstruction};

// all the GetInstruction and Values here need to be references to avoid a recursive type
// these could be boxes, but we're going to allocate in a bump arena (bumpalo)
pub enum OperatorInstruction<'a> {
    Id(&'a GetInstruction<'a>),
    UnaryOp {op: UnaryOp, value: &'a Value<'a>},
    BinaryOp {op: BinaryOp, left: &'a Value<'a>, right: &'a Value<'a>},
}

pub enum UnaryOp {
    VectorIndex(u8),
    Not(),
    Abs(),
    FloatToInt(FloatToIntOp),
    FloatToFloat(FloatToFloatOp),
}

pub enum FloatToIntOp {
    Round,
    Floor,
    Ceiling,
}

pub enum FloatToFloatOp {
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

pub enum BinaryOp {
    Math(MathOp),
    Comparison(ComparisonOp),
    Logic(LogicOp),
}

pub enum MathOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Pow,
    Random,
    Distance,
}

pub enum ComparisonOp {
    LessThan,
    GreaterThan,
    Equals,
}

pub enum LogicOp {
    And,
    Or,
}
