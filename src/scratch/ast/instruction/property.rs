use crate::scratch::ast::instruction::Value;
use crate::scratch::ast::Variable;

pub enum PropertyInstruction<'a> {
    Visibility(), // TODO add args for visibility, more complex
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
