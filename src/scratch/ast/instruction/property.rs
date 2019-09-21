use crate::scratch::ast::instruction::Value;
use crate::scratch::ast::{Variable, SpriteScripts, List};
use crate::scratch::parse::project::sb3::RotationStyle;

#[derive(Copy)] // TODO should Instructions be Copy?
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

enum VisibleThing<'a> {
    Sprite(&'a SpriteScripts<'a>),
    Variable(&'a Variable<'a>),
    List(&'a List<'a>),
}

enum TimeUnit {
    Second,
    Minute,
    Hour,
    DayOfWeek,
    Date,
    Month,
    Year,
    DaysSince2000,
}

enum NamedType {
    Costume,
    Backdrop,
}

enum NamedProperty {
    Number,
    Name,
}

enum RotationStyle {
    LeftRight,
    DontRotate,
    AllAround,
}

enum StageProperty<'a> {
    Backdrop(NamedProperty),
    Volume(),
    Variable(&'a Variable<'a>),
}

enum VisualEffectType {
    Color,
    Fisheye,
    Whirl,
    Pixelate,
    Mosaic,
    Brightness,
    Ghost,
}

enum AudioEffectType {
    Pitch,
    PanLeftRight,
}
