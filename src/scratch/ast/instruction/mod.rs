use crate::scratch::ast::instruction::property::PropertyInstruction;
use crate::scratch::ast::instruction::operator::{OperatorInstruction, BinaryOp, MathOp};
use crate::scratch::ast::instruction::list::ListInstruction;
use crate::scratch::ast::instruction::control_flow::ControlFlowInstruction;
use crate::scratch::ast::instruction::render::RenderInstruction;
use crate::scratch::ast::{Variable, List, Block};

pub mod operator;
pub mod property;
pub mod list;
pub mod control_flow;
pub mod render;

// TODO can't have self-referential data structures, need references or Boxes or bumpalo created references

// example, but need to chain reads too
fn change_by(property: PropertyInstruction, delta: Value) -> SetInstruction {
    SetInstruction {
        property: ReadWriteInstruction::Property(property),
        value: OperatorInstruction::BinaryOp {
            op: BinaryOp::Math(MathOp::Add),
            left: OperatorInstruction::Id(GetInstruction::ReadWrite(ReadWriteInstruction::Property(property))),
            right: delta,
        },
    }
}

pub enum Instruction<'a> {
    Op(OperatorInstruction<'a>),
    Get(GetInstruction<'a>),
    Set(SetInstruction<'a>),
    List(ListInstruction<'a>, &'a Value<'a>),
    ControlFlow(ControlFlowInstruction<'a>),
    Render(RenderInstruction<'a>),
    Custom(&'a CustomInstruction<'a>),
}

pub enum ReadWriteInstruction<'a> {
    Variable(&'a Variable<'a>),
    List(&'a List<'a>),
    Property(PropertyInstruction<'a>),
}

pub enum GetInstruction<'a> {
    ReadWrite(ReadWriteInstruction<'a>),
    Element(ReturningListInstruction<'a>, &'a Value<'a>),
}

pub struct SetInstruction<'a> {
    property: ReadWriteInstruction<'a>,
    value: Value<'a>,
}

pub type Value<'a> = OperatorInstruction<'a>;

struct CustomInstruction<'a> {
    args: Vec<Variable<'a>>,
    body: Block<'a>,
}
