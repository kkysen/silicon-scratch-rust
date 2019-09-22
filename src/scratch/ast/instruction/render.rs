use crate::scratch::ast::instruction::Value;
use crate::scratch::ast::Sound;
use crate::scratch::ast::compute_kind::{Computable, ComputeKind};

pub enum RenderInstruction<'a> {
    GlideTo(Value<'a>),
    SayOrThink(SayOrThink, Value<'a>, Option<Value<'a>>),
    ClearVisualEffects(),
    SoundInstruction(SoundInstruction, &'a Sound),
    StopAllSounds(),
    ClearAudioEffects(),
}

impl Computable for RenderInstruction<'_> {
    fn get_compute_kind(&self) -> ComputeKind {
        ComputeKind::Graphical
    }
}

pub enum SayOrThink {
    Say,
    Think,
}

pub enum SoundInstruction {
    Play,
    Start,
}
