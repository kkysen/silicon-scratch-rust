use crate::scratch::ast::{Constant, List, Variable};
use crate::scratch::ast::compute_kind::{Computable, ComputeKind};
use crate::scratch::ast::instruction::control_flow::ControlFlowInstruction;
use crate::scratch::ast::instruction::function_call::FunctionCallInstruction;
use crate::scratch::ast::instruction::list::{ListInstruction, ReturningListInstruction};
use crate::scratch::ast::instruction::property::PropertyInstruction;
use crate::scratch::ast::instruction::render::RenderInstruction;

pub mod function_call;
pub mod property;
pub mod list;
pub mod control_flow;
pub mod render;

pub enum Instruction<'a> {
    Op(FunctionCallInstruction<'a>),
    Get(GetInstruction<'a>),
    Set(SetInstruction<'a>),
    List(ListInstruction<'a>, &'a Value<'a>),
    ControlFlow(ControlFlowInstruction<'a>),
    Render(RenderInstruction<'a>),
}

impl Computable for Instruction<'_> {
    fn get_compute_kind(&self) -> ComputeKind {
        match self {
            Instruction::Op(op) => op.get_compute_kind(),
            Instruction::Get(get) => get.get_compute_kind(),
            Instruction::Set(set) => set.get_compute_kind(),
            Instruction::List(op, list) => (op, *list).get_compute_kind(),
            Instruction::ControlFlow(inst) => inst.get_compute_kind(),
            Instruction::Render(inst) => inst.get_compute_kind(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum ReadWriteInstruction<'a> {
    Variable(&'a Variable<'a>),
    List(&'a List<'a>),
    Property(PropertyInstruction<'a>),
}

impl Computable for ReadWriteInstruction<'_> {
    fn get_compute_kind(&self) -> ComputeKind {
        match self {
            // global variables and list are computational,
            // but variable that are custom block args are really values
            ReadWriteInstruction::Variable(variable) => variable.get_compute_kind(),
            ReadWriteInstruction::List(list) => list.get_compute_kind(),
            ReadWriteInstruction::Property(property) => property.get_compute_kind(),
        }
    }
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

impl Computable for GetInstruction<'_> {
    fn get_compute_kind(&self) -> ComputeKind {
        match self {
            GetInstruction::Constant(constant) => constant.get_compute_kind(),
            GetInstruction::ReadWrite(property) => property.get_compute_kind(),
            GetInstruction::Element(op, list) => (op, *list).get_compute_kind(),
        }
    }
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

impl Computable for SetInstruction<'_> {
    fn get_compute_kind(&self) -> ComputeKind {
        (&self.property, &self.value).get_compute_kind()
    }
}

pub type Value<'a> = FunctionCallInstruction<'a>;
