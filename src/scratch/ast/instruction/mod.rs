use crate::scratch::ast::instruction::property::PropertyInstruction;
use crate::scratch::ast::instruction::operator::OperatorInstruction;
use crate::scratch::ast::instruction::list::ListInstruction;
use crate::scratch::ast::instruction::control_flow::ControlFlowInstruction;
use crate::scratch::ast::instruction::render::RenderInstruction;
use crate::scratch::ast::{Variable, List, Block};

pub mod operator;
pub mod property;
pub mod list;
pub mod control_flow;
pub mod render;

pub enum Instruction<'a> {
    Op(OperatorInstruction<'a>),
    Get(GetInstruction<'a>),
    Set(ReadWriteInstruction<'a>, Value<'a>),
    List(ListInstruction<'a>, &'a List<'a>),
    ControlFlow(ControlFlowInstruction<'a>),
    Render(RenderInstruction<'a>),
    Custom(&'a CustomInstruction<'a>),
}

pub enum ReadWriteInstruction<'a> {
    Variable(&'a Variable<'a>),
    Property(PropertyInstruction<'a>),
}

pub enum GetInstruction<'a> {
    ReadWrite(ReadWriteInstruction<'a>),
    Element(ReturningListInstruction<'a>, &'a List<'a>),
}

pub type Value<'a> = OperatorInstruction<'a>;

struct CustomInstruction<'a> {
    args: Vec<Variable<'a>>,
    body: Block<'a>,
}
