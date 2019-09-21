use crate::scratch::ast::instruction::Value;
use crate::scratch::ast::Sound;

pub enum RenderInstruction<'a> {
    GlideTo(Value<'a>),
    SayOrThink(SayOrThink, Value<'a>, Optional<Value<'a>>),
    ClearVisualEffects(),
    SoundInstruction(SoundInstruction, &'a Sound),
    StopAllSounds(),
    ClearAudioEffects(),
}

enum SayOrThink {
    Say,
    Think,
}

enum SoundInstruction {
    Play,
    Start,
}
