use crate::scratch::ast::instruction::Value;
use crate::scratch::ast::{Variable, SpriteScripts, List};
use crate::scratch::ast::compute_kind::{Computable, ComputeKind};

#[derive(Clone, Copy)]
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

impl Computable for PropertyInstruction<'_> {
    fn get_compute_kind(&self) -> ComputeKind {
        match self {
            PropertyInstruction::UserName() => ComputeKind::Computational,
            PropertyInstruction::Named(_, _) => ComputeKind::Computational,
            PropertyInstruction::Stage(property) => property.get_compute_kind(),
            _ => ComputeKind::Graphical,
        }
    }
}

#[derive(Clone, Copy)]
pub enum VisibleThing<'a> {
    Sprite(&'a SpriteScripts<'a>),
    Variable(&'a Variable<'a>),
    List(&'a List<'a>),
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub enum NamedType {
    Costume,
    Backdrop,
}

#[derive(Clone, Copy)]
pub enum NamedProperty {
    Number,
    Name,
}

#[derive(Clone, Copy)]
pub enum RotationStyle {
    LeftRight,
    DontRotate,
    AllAround,
}

#[derive(Clone, Copy)]
pub enum StageProperty<'a> {
    Backdrop(NamedProperty),
    Volume(),
    Variable(&'a Variable<'a>),
}

impl Computable for StageProperty<'_> {
    fn get_compute_kind(&self) -> ComputeKind {
        match self {
            StageProperty::Backdrop(_) => ComputeKind::Computational,
            StageProperty::Volume() => ComputeKind::Graphical,
            StageProperty::Variable(_) => ComputeKind::Computational,
        }
    }
}

#[derive(Clone, Copy)]
pub enum VisualEffectType {
    Color,
    Fisheye,
    Whirl,
    Pixelate,
    Mosaic,
    Brightness,
    Ghost,
}

#[derive(Clone, Copy)]
pub enum AudioEffectType {
    Pitch,
    PanLeftRight,
}
