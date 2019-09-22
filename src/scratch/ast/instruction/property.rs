use crate::scratch::ast::instruction::Value;
use crate::scratch::ast::{Variable, SpriteScripts, List};

//#[derive(Copy)] // TODO should Instructions be Copy?
pub enum PropertyInstruction<'a> {
    Visibility(VisibleThing<'a>),
    Position(),
    MousePointer(),
    Direction(),
    Size(),
    Volume(),
    Loudness(),
    Timer(),
    CurrentTime(TimeUnit),
    Answer(),
    UserName(),
    Named(NamedType, NamedProperty),
    RotationStyle(RotationStyle),
    Draggable(),
    Layer(),
    Stage(StageProperty<'a>),
    MouseDown(),
    KeyPressed(char),
    VisualEffect(VisualEffectType),
    AudioEffect(AudioEffectType),
    TouchingColor(Value<'a>),
    ColorIsTouchingColor(Value<'a>, Value<'a>),
}

pub enum VisibleThing<'a> {
    Sprite(&'a SpriteScripts<'a>),
    Variable(&'a Variable<'a>),
    List(&'a List<'a>),
}

pub enum TimeUnit {
    Second,
    Minute,
    Hour,
    DayOfWeek,
    Date,
    Month,
    Year,
    DaysSince2000,
}

pub enum NamedType {
    Costume,
    Backdrop,
}

pub enum NamedProperty {
    Number,
    Name,
}

pub enum RotationStyle {
    LeftRight,
    DontRotate,
    AllAround,
}

pub enum StageProperty<'a> {
    Backdrop(NamedProperty),
    Volume(),
    Variable(&'a Variable<'a>),
}

pub enum VisualEffectType {
    Color,
    Fisheye,
    Whirl,
    Pixelate,
    Mosaic,
    Brightness,
    Ghost,
}

pub enum AudioEffectType {
    Pitch,
    PanLeftRight,
}
