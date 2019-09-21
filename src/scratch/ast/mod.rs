pub mod ast;

use crate::scratch::ast::ValueKind::{Constant, Instruction, Variable, List};
use std::collections::HashMap;

// order important, lower can be cast to higher
#[derive(Copy, Eq, Ord)]
enum ComputeKind {
    Computational,
    Graphical,
    Reactive,
}

enum ValueKind {
    Constant,
    Instruction,
    Variable,
    List,
}

impl ValueKind {
    fn is_constant(&self) -> bool {
        self == Constant
    }
    
    fn is_instruction(&self) -> bool {
        self == Instruction
    }
    
    fn is_variable(&self) -> bool {
        self == Variable
    }
    
    fn is_list(&self) -> bool {
        self == List
    }
    
    fn is_rvalue(&self) -> bool {
        true
    }
    
    fn is_lvalue(&self) -> bool {
        self.is_variable() || self.is_list()
    }
}

struct Type {}

struct Value<'a> {
    kind: ValueKind,
    reads: Vec<&'a Instruction<'a>>,
}

impl Value {
    fn new(kind: ValueKind) -> Value {
        Value { kind, reads: Vec::new() }
    }
    
    fn add_read(&mut self, instruction: &Instruction) {
        self.reads.push(instruction);
    }
    
    fn get_type(&self) -> Type {
        unimplemented!()
    }
}

trait Computable {
    fn get_compute_kind(&self) -> ComputeKind;
}

impl Computable for ComputeKind {
    fn get_compute_kind(&self) -> ComputeKind {
        *self
    }
}

impl<T, I> Computable for I where T: Computable, I: Iterator<Item = T> {
    fn get_compute_kind(&self) -> ComputeKind {
        self.map(Computable::get_compute_kind)
            .max()
            .unwrap_or(ComputeKind::Computational)
    }
}

impl<T, I> Computable for I where T: Computable, I: Iterator<Item = Box<T>> {
    fn get_compute_kind(&self) -> ComputeKind {
        self.map(Box::as_ref()).get_compute_kind()
    }
}

trait InstructionTrait<'a> {
    fn own_compute_kind() -> ComputeKind;
    
    fn raw_args(&self) -> &[Box<dyn Computable>];
    
    fn raw_bodies(&self) -> &[Box<Block<'a>>];
    
    fn args(&self) {
        self.raw_args().iter().map(Box::as_ref())
    }
    
    fn bodies(&self) {
        self.raw_bodies().iter().map(Box::as_ref())
    }
}

enum Property {
    Position(),
    Direction(),
    RotationStyle(),
}

enum ScratchBlock {
    Move(),
    TurnLeft(),
    TurnRight(),
    GoTo(),
    RandomPosition(),
    MousePointer(),
    GlideTo(),
    PointInDirection(),
    PointTowards(),
    ChangeXBy(),
    SetX(),
    ChangeYBy(),
    SetY(),
    BounceOnEdge(),
    SetRotationStyle(),
    XPosition(),
    YPosition(),
    RotationStyle(),
    SayFor(),
    Say(),
    ThinkFor(),
    Think(),
    SwitchCostumeTo(),
    NextCostume(),
    SwitchBackdropTo(),
    NextBackdrop(),
    ChangeSizeBy(),
    SetSize(),
    ChangeLookEffectBy(),
    SetLookEffect(),
    ClearLookEffects(),
    Show(),
    Hide(),
    GoToLayer(),
    MoveLayer(),
    CostumeNumber(),
    CostumeName(),
    BackdropNumber(),
    BackdropName(),
    Size(),
    PlaySound(),
    StartSound(),
    StopAllSounds(),
    ChangeSoundEffectBy(),
    SetSoundEffect(),
    ClearSoundEffects(),
    ChangeVolumeBy(),
    SetVolume(),
    Volume(),
    BroadCast(),
    BroadCastAndWait(),
    Wait(),
    Repeat(),
    Forever(),
    IfThen(),
    IfThenElse(),
    WaitUntil(),
    RepeatUntil(),
    Stop(),
    CreateClone(),
    DeleteThisClone(),
    Touching(),
    TouchingEdge(),
    TouchingColor(),
    ColorIsTouchingColor(),
    DistanceTo(),
    AskAndWait(),
    Answer(),
    KeyPressed(),
    MouseDown(),
    MouseX(),
    MouseY(),
    SetDragMode(),
    Loudness(),
    Timer(),
    ResetTimer(),
    BackdropNumberOfStage(),
    CurrentTime(),
    DaysSince2000(),
    UserName(),
    Add(),
    Subtract(),
    Multiply(),
    Divide(),
    Random(),
    GreaterThan(),
    LessThan(),
    Equals(),
    And(),
    Or(),
    Not(),
    JoinStrings(),
    LetterOfString(),
    LengthOfString(),
    StringContainsString(),
    Modulo(),
    UnaryMathFunc(),
    SetVariable(),
    ChangeVariableBy(),
    ShowVariable(),
    HideVariable(),
    AddToList(),
    DeleteFromList(),
    DeleteAllOfList(),
    InsertAtIntoList(),
    ReplaceItemOfListWith(),
    ItemOfList(),
    ItemNumberOfInList(),
    LengthOfList(),
    ListContains(),
    ShowList(),
    HideList(),
    
}

struct Instruction<'a> {
    value: Value<'a>,
    r#impl: dyn InstructionTrait<'a>,
}

impl Instruction {
    fn init_reads(&mut self) {
        for &arg in self.args {
            arg.add_read(self);
        }
    }
    
    fn new(compute_kind: ComputeKind, args: Vec<Box<Value>>, bodies: Vec<Box<Block>>) -> Instruction {
        let mut inst = Instruction {
            value: Value::new(ValueKind::Instruction),
            own_compute_kind: compute_kind,
            args,
            bodies,
        };
        inst.init_reads();
        inst
    }
}

impl Computable for Instruction {
    fn get_compute_kind(&self) -> ComputeKind {
        [self.own_compute_kind, self.args, self.bodies].get_compute_kind()
    }
}

struct Block<'a> {
    instructions: Vec<Instruction<'a>>,
}

impl Computable for Block {
    fn get_compute_kind(&self) -> ComputeKind {
        self.instructions.get_compute_kind()
    }
}

struct Script<'a> {
    trigger: Event,
    block: Block<'a>,
}

impl Computable for Script {
    fn get_compute_kind(&self) -> ComputeKind {
        self.block.get_compute_kind()
    }
}

struct Sprite {
    // TODO
}

struct SpriteScripts<'a> {
    sprite: Sprite,
    locals: Scope<'a>,
    scripts: Vec<Script<'a>>,
}

struct Program<'a> {
    globals: Scope<'a>,
    sprite_scripts: Vec<SpriteScripts<'a>>,
}

struct Scope<'a> {
    variables: HashMap<&'a str, Variable<'a>>,
    lists: HashMap<&'a str, List<'a>>,
}

struct Constant<'a> {
    value: Value<'a>,
    constant: _Constant,
}

impl Constant {
    fn new(constant: _Constant) -> Constant {
        Constant {
            value: Value::new(ValueKind::Constant),
            constant,
        }
    }
}

impl Computable for Constant {
    fn get_compute_kind(&self) -> ComputeKind {
        ComputeKind::Computational
    }
}

enum _Constant {
    Primitive(Primitive),
    Event(Event),
}

struct Primitive {
    kind: PrimitiveType,
    value: String,
}

enum PrimitiveType {
    Bool,
    Num,
    String,
}

enum Event {
    Click(),
    KeyPress(),
    Condition(),
    BroadCast(String),
}

struct Property {
    compute_kind: ComputeKind,
}

impl Computable for Property {
    fn get_compute_kind(&self) -> ComputeKind {
        self.compute_kind
    }
}

struct LValue<'a> {
    value: Value<'a>,
    name: String,
    writes: Vec<&'a Instruction<'a>>,
}

impl LValue {
    fn new(kind: ValueKind, name: String) -> LValue {
        LValue {
            value: Value::new(kind),
            name,
            writes: Vec::new(),
        }
    }
}

struct Variable<'a> {
    lvalue: LValue<'a>,
    initial_value: Primitive,
}

impl Variable {
    fn new(name: String, initial_value: Primitive) -> Variable {
        Variable {
            lvalue: LValue::new(ValueKind::Variable, name),
            initial_value,
        }
    }
}

struct List<'a> {
    lvalue: LValue<'a>,
    initial_values: Vec<Primitive>,
}

impl List {
    fn new(name: String, initial_values: Vec<Primitive>) -> List {
        List {
            lvalue: LValue::new(ValueKind::List, name),
            initial_values,
        }
    }
}
