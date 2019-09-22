use crate::scratch::ast::instruction::Value;
use crate::scratch::ast::{Block, SpriteScripts, BroadCast, Script};
use crate::scratch::ast::compute_kind::{Computable, ComputeKind};

pub enum ControlFlowInstruction<'a> {
    BroadCast(BroadCastInstruction, &'a BroadCast),
    Wait(Value<'a>),
    AskAndWait(Value<'a>),
    Branch {r#if: Block<'a>, r#else: Block<'a>},
    While(Block<'a>),
    Stop(StopInstruction<'a>),
    CreateClone(&'a SpriteScripts<'a>),
    DeleteSelf(),
}

impl Computable for ControlFlowInstruction<'_> {
    fn get_compute_kind(&self) -> ComputeKind {
        match self {
            ControlFlowInstruction::BroadCast(_, _) => ComputeKind::Reactive,
            ControlFlowInstruction::Wait(_) => ComputeKind::Reactive,
            ControlFlowInstruction::AskAndWait(_) => ComputeKind::Reactive,
            ControlFlowInstruction::Branch { r#if, r#else } => (r#if, r#else).get_compute_kind(),
            ControlFlowInstruction::While(block) => block.get_compute_kind(),
            // TODO are these last three right?
            ControlFlowInstruction::Stop(_) => ComputeKind::Reactive,
            ControlFlowInstruction::CreateClone(_) => ComputeKind::Reactive,
            ControlFlowInstruction::DeleteSelf() => ComputeKind::Reactive,
        }
    }
}

pub enum BroadCastInstruction {
    Resume,
    Wait,
}

pub enum StopInstruction<'a> {
    OwnScript(&'a Script<'a>),
    OtherScriptsInSprite(&'a SpriteScripts<'a>),
}
