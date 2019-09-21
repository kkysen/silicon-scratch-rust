use std::collections::HashMap;
use optional::Optioned;

struct Event {}
struct BroadCast {}
struct Sound {}
struct Sprite {}

struct Instruction<'a> {
    instruction: AnyInstruction<'a>,
    reads: Vec<&'a Instruction<'a>>,
}

enum AnyInstruction<'a> {
    Op(OperatorInstruction<'a>),
    Get(GetInstruction<'a>),
    Set(PropertyInstruction, Value<'a>),
    List(ListInstruction<'a>, &'a List<'a>),
    ControlFlow(ControlFlowInstruction<'a>),
    Render(RenderInstruction<'a>),
    Custom(&'a CustomInstruction<'a>),
}

enum GetInstruction<'a> {
    Variable(&'a Variable<'a>),
    Element(ReturningListInstruction<'a>, &'a List<'a>),
    Property(PropertyInstruction),
}

enum OperatorInstruction<'a> {
    Id(GetInstruction<'a>),
    UnaryOp(UnaryOp, Value<'a>),
    BinaryOp(BinaryOp, Value<'a>, Value<'a>),
}

type Value<'a> = OperatorInstruction<'a>;

enum UnaryOp {
    VectorIndex(u8),
    Not(),
    Abs(),
    FloatToInt(FloatToIntOp),
    FloatToFloat(FloatToFloatOp),
}

enum FloatToIntOp {
    Round,
    Floor,
    Ceiling,
}

enum FloatToFloatOp {
    Sqrt,
    Sin,
    Cos,
    Tan,
    ASin,
    ACos,
    ATan,
    Ln,
    Log,
    Exp,
}

enum BinaryOp {
    Math(MathOp),
    Comparison(ComparisonOp),
    Logic(LogicOp),
}

enum MathOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Pow,
    Random,
    Distance,
}

enum ComparisonOp {
    LessThan,
    GreaterThan,
    Equals,
}

enum LogicOp {
    And,
    Or,
}

enum PropertyInstruction {
    // TODO add args
    Visibility(),
    Position(),
    MousePointer(),
    Direction(),
    VisualEffect(),
    Costume(),
    Backdrop(),
    Size(),
    AudioEffect(),
    Volume(),
    Touching(),
    TouchingColor(),
    ColorIsTouchingColor(),
    Answer(),
    KeyPressed(),
    MouseDown(),
    DragMode(),
    Loudness(),
    Timer(),
    Stage(),
    CurrentTime(),
    UserName(),
}

enum ListInstruction<'a> {
    Void(VoidListInstruction<'a>),
    Returning(ReturningListInstruction<'a>),
}

enum VoidListInstruction<'a> {
    Set(Value<'a>, Value<'a>),
    Insert(Value<'a>, Value<'a>),
    Remove(Value<'a>),
    Push(Value<'a>, PushPop), // Push is like a constant time version of Insert and Remove
    Clear(),
}

enum ReturningListInstruction<'a> {
    Length(),
    Get(Value<'a>),
    Pop(PushPop),
    Find(Value<'a>),
    Contains(Value<'a>),
    ContainsSeq(&'a List<'a>),
    // for list reads, in a.contains(b), can treat a and b the same
    // heuristics will treat them the same
}

struct PushPop {
    side: PushPopSide,
    index: u8,
    // also support popping the nth to last element, where n is a small constant
    // 0 is the normal behavior (e.x. pop the 0th element)
}

enum ControlFlowInstruction<'a> {
    BroadCast(BroadCastInstruction, &'a BroadCast),
    Wait(Value<'a>),
    Branch {r#if: Block<'a>, r#else: Block<'a>},
    While(Block<'a>),
    Stop(StopInstruction<'a>),
    CreateClone(&'a SpriteScripts<'a>),
    DeleteSelf(),
}

enum BroadCastInstruction {
    Resume,
    Wait,
}

enum StopInstruction<'a> {
    OwnScript(&'a Script<'a>),
    OtherScriptsInSprite(&'a SpriteScripts<'a>),
}

enum RenderInstruction<'a> {
    GlideTo(Value<'a>),
    SayOrThink(SayOrThinkInstruction, Value<'a>, Optional<Value<'a>>),
    ClearVisualEffects(),
    SoundInstruction(SoundInstruction, &'a Sound),
    StopAllSounds(),
    ClearAudioEffects(),
}

enum SayOrThinkInstruction {
    Say,
    Think,
}

enum SoundInstruction {
    Play,
    Start,
}

struct CustomInstruction<'a> {
    args: Vec<Variable<'a>>,
    body: Block<'a>,
}

struct Block<'a> {
    instructions: Vec<Instruction<'a>>,
}

struct Script<'a> {
    trigger: Event,
    block: Block<'a>,
}

struct Scope<'a> {
    variables: HashMap<&'a str, Variable<'a>>,
    lists: HashMap<&'a str, List<'a>>,
}

struct SpriteScripts<'a> {
    sprite: &'a Sprite,
    locals: Scope<'a>,
    scripts: Vec<Script<'a>>,
}

struct Program<'a> {
    globals: Scope<'a>,
    sprite_scripts: Vec<SpriteScripts<'a>>,
}

struct Variable<'a> {
    name: String,
    reads: Vec<&'a GetInstruction<'a>>,
    writes: Vec<&'a Value<'a>>,
}

struct List<'a> {
    name: String,
    reads: Vec<&'a ListInstruction<'a>>,
    element_reads: Vec<&'a GetInstruction<'a>>,
    element_writes: Vec<&'a Value<'a>>,
}
