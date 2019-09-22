use crate::scratch::ast::{Block, Constant, List, Variable};
use crate::scratch::ast::instruction::control_flow::ControlFlowInstruction;
use crate::scratch::ast::instruction::list::{ListInstruction, ReturningListInstruction};
use crate::scratch::ast::instruction::operator::OperatorInstruction;
use crate::scratch::ast::instruction::property::PropertyInstruction;
use crate::scratch::ast::instruction::render::RenderInstruction;

pub mod operator;
pub mod property;
pub mod list;
pub mod control_flow;
pub mod render;

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

impl<'a> ReadWriteInstruction<'a> {
    fn add_write(&mut self) {
//        match self {
//             TODO
//        }
    }
}

pub enum GetInstruction<'a> {
    Constant(Constant<'a>),
    ReadWrite(ReadWriteInstruction<'a>),
    Element(ReturningListInstruction<'a>, &'a Value<'a>),
}

pub struct SetInstruction<'a> {
    property: ReadWriteInstruction<'a>,
    value: Value<'a>,
}

impl<'a> SetInstruction<'a> {
    pub fn new(property: ReadWriteInstruction<'a>, value: Value<'a>) -> SetInstruction<'a> {
        // TODO property.add_write();
        SetInstruction {
            property,
            value,
        }
    }
}

pub type Value<'a> = OperatorInstruction<'a>;

pub struct CustomInstruction<'a> {
    args: Vec<Variable<'a>>,
    body: Block<'a>,
}
