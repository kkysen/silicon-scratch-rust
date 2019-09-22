use crate::scratch::ast::instruction::Value;
use crate::scratch::ast::{Block, SpriteScripts, BroadCast, Script};

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

pub enum BroadCastInstruction {
    Resume,
    Wait,
}

pub enum StopInstruction<'a> {
    OwnScript(&'a Script<'a>),
    OtherScriptsInSprite(&'a SpriteScripts<'a>),
}
