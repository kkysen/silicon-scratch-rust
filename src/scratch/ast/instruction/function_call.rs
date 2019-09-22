use crate::scratch::ast::compute_kind::{Computable, ComputeKind};
use crate::scratch::ast::Function;
use crate::scratch::ast::instruction::{GetInstruction, Value};

// all the GetInstruction and Values here need to be references to avoid a recursive type
// these could be boxes, but we're going to allocate in a bump arena (bumpalo)
#[derive(Clone, Copy)]
pub enum FunctionCallInstruction<'a> {
    Id(&'a GetInstruction<'a>),
    UnaryOp { op: UnaryOp, value: &'a Value<'a> },
    BinaryOp { op: BinaryOp, left: &'a Value<'a>, right: &'a Value<'a> },
    Function { function: &'a Function<'a>, args: &'a [Value<'a>] },
}

impl Computable for FunctionCallInstruction<'_> {
    fn get_compute_kind(&self) -> ComputeKind {
        // own compute kind is computational, so only sub compute kinds matter
        match self {
            FunctionCallInstruction::Id(get)
            => get.get_compute_kind(),
            FunctionCallInstruction::UnaryOp { op: _, value }
            => value.get_compute_kind(),
            FunctionCallInstruction::BinaryOp { op: _, left, right }
            => (*left, *right).get_compute_kind(),
            FunctionCallInstruction::Function { function, args }
            => function.get_compute_kind(args),
        }
    }
}

#[derive(Clone, Copy)]
pub enum UnaryOp {
    VectorIndex(u8),
    Not(),
    Abs(),
    FloatToInt(FloatToIntOp),
    FloatToFloat(FloatToFloatOp),
}

#[derive(Clone, Copy)]
pub enum FloatToIntOp {
    Round,
    Floor,
    Ceiling,
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub enum BinaryOp {
    Math(MathOp),
    Comparison(ComparisonOp),
    Logic(LogicOp),
}

impl BinaryOp {
    pub fn add() -> BinaryOp {
        BinaryOp::Math(MathOp::Add)
    }
    pub fn subtract() -> BinaryOp {
        BinaryOp::Math(MathOp::Subtract)
    }
    pub fn multiply() -> BinaryOp {
        BinaryOp::Math(MathOp::Multiply)
    }
    pub fn divide() -> BinaryOp {
        BinaryOp::Math(MathOp::Divide)
    }
    pub fn modulo() -> BinaryOp {
        BinaryOp::Math(MathOp::Modulo)
    }
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub enum ComparisonOp {
    LessThan,
    GreaterThan,
    Equals,
}

#[derive(Clone, Copy)]
pub enum LogicOp {
    And,
    Or,
}
