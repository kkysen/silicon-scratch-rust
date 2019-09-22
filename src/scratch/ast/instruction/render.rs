use crate::scratch::ast::instruction::Value;
use crate::scratch::ast::Sound;

pub enum RenderInstruction<'a> {
    GlideTo(Value<'a>),
    SayOrThink(SayOrThink, Value<'a>, Option<Value<'a>>),
    ClearVisualEffects(),
    SoundInstruction(SoundInstruction, &'a Sound),
    StopAllSounds(),
    ClearAudioEffects(),
}

pub enum SayOrThink {
    Say,
    Think,
}

pub enum SoundInstruction {
    Play,
    Start,
}
