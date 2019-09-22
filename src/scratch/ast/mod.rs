use std::collections::HashMap;

use bumpalo::Bump;

use crate::scratch::ast::compute_kind::{Computable, ComputeKind};
use crate::scratch::ast::instruction::{GetInstruction, Instruction, ReadWriteInstruction, SetInstruction, Value};
use crate::scratch::ast::instruction::function_call::{BinaryOp, FunctionCallInstruction, UnaryOp};
use std::cell::RefCell;

pub mod instruction;
pub mod compute_kind;

pub enum Number {
    UInt(u64),
    Int(i64),
    Float(f64),
}

pub enum Constant<'a> {
    Bool(bool),
    Number(Number),
    String(&'a str),
}

impl Computable for Constant<'_> {
    fn get_compute_kind(&self) -> ComputeKind {
        ComputeKind::Computational
    }
}

pub struct Event {}

pub struct BroadCast {}

pub struct Sound {}

pub struct Sprite {}

pub struct Block<'a> {
    instructions: Vec<Instruction<'a>>,
}

impl Computable for Block<'_> {
    fn get_compute_kind(&self) -> ComputeKind {
        (&self.instructions[..]).get_compute_kind()
    }
}

pub struct Script<'a> {
    trigger: Event,
    block: Block<'a>,
}

pub struct Scope<'a> {
    variables: HashMap<&'a str, Variable<'a>>,
    lists: HashMap<&'a str, List<'a>>,
}

pub struct SpriteScripts<'a> {
    sprite: &'a Sprite,
    locals: Scope<'a>,
    scripts: Vec<Script<'a>>,
}

pub struct Function<'a> {
    name: String,
    args: RefCell<Vec<Variable<'a>>>,
    body: Block<'a>,
}

impl<'a> Function<'a> {
    fn get_compute_kind(&self, args: &[Value<'a>]) -> ComputeKind {
        // copy args into params, then evaluate body's compute kind
        let params = &mut self.args.borrow_mut()[..];
        params
            .iter_mut()
            .zip(args.iter())
            .for_each(|(param, arg)| param.value = *arg);
        self.body.get_compute_kind()
    }
}

pub struct Program<'a> {
    globals: Scope<'a>,
    sprite_scripts: Vec<SpriteScripts<'a>>,
    functions: Vec<Function<'a>>,
}

pub struct Variable<'a> {
    name: String,
    value: Value<'a>, // TODO should this be Option?
    // set to initial value for global, set to arg for function param
    reads: Vec<&'a ReadWriteInstruction<'a>>,
    writes: Vec<&'a ReadWriteInstruction<'a>>,
}

impl Computable for Variable<'_> {
    fn get_compute_kind(&self) -> ComputeKind {
        self.value.get_compute_kind()
    }
}

pub struct List<'a> {
    variable: Variable<'a>,
    // should be empty as Scratch allows it, except for strings
    element_reads: Vec<&'a GetInstruction<'a>>,
    element_writes: Vec<&'a Value<'a>>,
}

impl Computable for List<'_> {
    fn get_compute_kind(&self) -> ComputeKind {
        self.variable.get_compute_kind()
    }
}

pub struct AST<'a> {
    bump: Bump,
    program: Program<'a>,
}

impl<'a> AST<'a> {
    fn alloc<T>(&self, value: T) -> &T {
        self.bump.alloc(value)
    }
    
    fn id(&'a self, get: GetInstruction<'a>) -> Value<'a> {
        FunctionCallInstruction::Id(self.alloc(get))
    }
    
    fn op1(&'a self, op: UnaryOp, value: Value<'a>) -> Value<'a> {
        FunctionCallInstruction::UnaryOp {
            op,
            value: self.alloc(value),
        }
    }
    
    fn op2(&'a self, op: BinaryOp, left: Value<'a>, right: Value<'a>) -> Value<'a> {
        FunctionCallInstruction::BinaryOp {
            op,
            left: self.alloc(left),
            right: self.alloc(right),
        }
    }
    
    fn get(&'a self, readable: ReadWriteInstruction<'a>) -> Value<'a> {
        self.id(GetInstruction::ReadWrite(readable))
    }
    
    fn set(&'a self, writable: ReadWriteInstruction<'a>, value: Value<'a>) -> SetInstruction<'a> {
        SetInstruction::new(writable, value)
    }
    
    fn change_using(&'a self, op: BinaryOp, property: ReadWriteInstruction<'a>, change: Value<'a>) -> SetInstruction<'a> {
        self.set(
            property,
            self.op2(op,
                     self.get(property),
                     change,
            ),
        )
    }
    
    fn change_by(&'a self, property: ReadWriteInstruction<'a>, delta: Value<'a>) -> SetInstruction<'a> {
        self.change_using(BinaryOp::add(), property, delta)
    }
}

//use crate::scratch::ast::ValueKind::{Constant, Instruction, Variable, List};
//use std::collections::HashMap;
//
//// order important, lower can be cast to higher
//#[derive(Copy, Eq, Ord)]
//enum ComputeKind {
//    Computational,
//    Graphical,
//    Reactive,
//}
//
//enum ValueKind {
//    Constant,
//    Instruction,
//    Variable,
//    List,
//}
//
//impl ValueKind {
//    fn is_constant(&self) -> bool {
//        self == Constant
//    }
//
//    fn is_instruction(&self) -> bool {
//        self == Instruction
//    }
//
//    fn is_variable(&self) -> bool {
//        self == Variable
//    }
//
//    fn is_list(&self) -> bool {
//        self == List
//    }
//
//    fn is_rvalue(&self) -> bool {
//        true
//    }
//
//    fn is_lvalue(&self) -> bool {
//        self.is_variable() || self.is_list()
//    }
//}
//
//struct Type {}
//
//struct Value<'a> {
//    kind: ValueKind,
//    reads: Vec<&'a Instruction<'a>>,
//}
//
//impl Value {
//    fn new(kind: ValueKind) -> Value {
//        Value { kind, reads: Vec::new() }
//    }
//
//    fn add_read(&mut self, instruction: &Instruction) {
//        self.reads.push(instruction);
//    }
//
//    fn get_type(&self) -> Type {
//        unimplemented!()
//    }
//}
//
//trait Computable {
//    fn get_compute_kind(&self) -> ComputeKind;
//}
//
//impl Computable for ComputeKind {
//    fn get_compute_kind(&self) -> ComputeKind {
//        *self
//    }
//}
//
//impl<T, I> Computable for I where T: Computable, I: Iterator<Item = T> {
//    fn get_compute_kind(&self) -> ComputeKind {
//        self.map(Computable::get_compute_kind)
//            .max()
//            .unwrap_or(ComputeKind::Computational)
//    }
//}
//
//impl<T, I> Computable for I where T: Computable, I: Iterator<Item = Box<T>> {
//    fn get_compute_kind(&self) -> ComputeKind {
//        self.map(Box::as_ref()).get_compute_kind()
//    }
//}
//
//trait InstructionTrait<'a> {
//    fn own_compute_kind() -> ComputeKind;
//
//    fn raw_args(&self) -> &[Box<dyn Computable>];
//
//    fn raw_bodies(&self) -> &[Box<Block<'a>>];
//
//    fn args(&self) {
//        self.raw_args().iter().map(Box::as_ref())
//    }
//
//    fn bodies(&self) {
//        self.raw_bodies().iter().map(Box::as_ref())
//    }
//}
//
//enum Property {
//    Position(),
//    Direction(),
//    RotationStyle(),
//}
//
//enum ScratchBlock {
//    Move(),
//    TurnLeft(),
//    TurnRight(),
//    GoTo(),
//    RandomPosition(),
//    MousePointer(),
//    GlideTo(),
//    PointInDirection(),
//    PointTowards(),
//    ChangeXBy(),
//    SetX(),
//    ChangeYBy(),
//    SetY(),
//    BounceOnEdge(),
//    SetRotationStyle(),
//    XPosition(),
//    YPosition(),
//    RotationStyle(),
//    SayFor(),
//    Say(),
//    ThinkFor(),
//    Think(),
//    SwitchCostumeTo(),
//    NextCostume(),
//    SwitchBackdropTo(),
//    NextBackdrop(),
//    ChangeSizeBy(),
//    SetSize(),
//    ChangeLookEffectBy(),
//    SetLookEffect(),
//    ClearLookEffects(),
//    Show(),
//    Hide(),
//    GoToLayer(),
//    MoveLayer(),
//    CostumeNumber(),
//    CostumeName(),
//    BackdropNumber(),
//    BackdropName(),
//    Size(),
//    PlaySound(),
//    StartSound(),
//    StopAllSounds(),
//    ChangeSoundEffectBy(),
//    SetSoundEffect(),
//    ClearSoundEffects(),
//    ChangeVolumeBy(),
//    SetVolume(),
//    Volume(),
//    BroadCast(),
//    BroadCastAndWait(),
//    Wait(),
//    Repeat(),
//    Forever(),
//    IfThen(),
//    IfThenElse(),
//    WaitUntil(),
//    RepeatUntil(),
//    Stop(),
//    CreateClone(),
//    DeleteThisClone(),
//    Touching(),
//    TouchingEdge(),
//    TouchingColor(),
//    ColorIsTouchingColor(),
//    DistanceTo(),
//    AskAndWait(),
//    Answer(),
//    KeyPressed(),
//    MouseDown(),
//    MouseX(),
//    MouseY(),
//    SetDragMode(),
//    Loudness(),
//    Timer(),
//    ResetTimer(),
//    BackdropNumberOfStage(),
//    CurrentTime(),
//    DaysSince2000(),
//    UserName(),
//    Add(),
//    Subtract(),
//    Multiply(),
//    Divide(),
//    Random(),
//    GreaterThan(),
//    LessThan(),
//    Equals(),
//    And(),
//    Or(),
//    Not(),
//    JoinStrings(),
//    LetterOfString(),
//    LengthOfString(),
//    StringContainsString(),
//    Modulo(),
//    UnaryMathFunc(),
//    SetVariable(),
//    ChangeVariableBy(),
//    ShowVariable(),
//    HideVariable(),
//    AddToList(),
//    DeleteFromList(),
//    DeleteAllOfList(),
//    InsertAtIntoList(),
//    ReplaceItemOfListWith(),
//    ItemOfList(),
//    ItemNumberOfInList(),
//    LengthOfList(),
//    ListContains(),
//    ShowList(),
//    HideList(),
//
//}
//
//struct Instruction<'a> {
//    value: Value<'a>,
//    r#impl: dyn InstructionTrait<'a>,
//}
//
//impl Instruction {
//    fn init_reads(&mut self) {
//        for &arg in self.args {
//            arg.add_read(self);
//        }
//    }
//
//    fn new(compute_kind: ComputeKind, args: Vec<Box<Value>>, bodies: Vec<Box<Block>>) -> Instruction {
//        let mut inst = Instruction {
//            value: Value::new(ValueKind::Instruction),
//            own_compute_kind: compute_kind,
//            args,
//            bodies,
//        };
//        inst.init_reads();
//        inst
//    }
//}
//
//impl Computable for Instruction {
//    fn get_compute_kind(&self) -> ComputeKind {
//        [self.own_compute_kind, self.args, self.bodies].get_compute_kind()
//    }
//}
//
//struct Block<'a> {
//    instructions: Vec<Instruction<'a>>,
//}
//
//impl Computable for Block {
//    fn get_compute_kind(&self) -> ComputeKind {
//        self.instructions.get_compute_kind()
//    }
//}
//
//struct Script<'a> {
//    trigger: Event,
//    block: Block<'a>,
//}
//
//impl Computable for Script {
//    fn get_compute_kind(&self) -> ComputeKind {
//        self.block.get_compute_kind()
//    }
//}
//
//struct Sprite {
//    // TODO
//}
//
//struct SpriteScripts<'a> {
//    sprite: Sprite,
//    locals: Scope<'a>,
//    scripts: Vec<Script<'a>>,
//}
//
//struct Program<'a> {
//    globals: Scope<'a>,
//    sprite_scripts: Vec<SpriteScripts<'a>>,
//}
//
//struct Scope<'a> {
//    variables: HashMap<&'a str, Variable<'a>>,
//    lists: HashMap<&'a str, List<'a>>,
//}
//
//struct Constant<'a> {
//    value: Value<'a>,
//    constant: _Constant,
//}
//
//impl Constant {
//    fn new(constant: _Constant) -> Constant {
//        Constant {
//            value: Value::new(ValueKind::Constant),
//            constant,
//        }
//    }
//}
//
//impl Computable for Constant {
//    fn get_compute_kind(&self) -> ComputeKind {
//        ComputeKind::Computational
//    }
//}
//
//enum _Constant {
//    Primitive(Primitive),
//    Event(Event),
//}
//
//struct Primitive {
//    kind: PrimitiveType,
//    value: String,
//}
//
//enum PrimitiveType {
//    Bool,
//    Num,
//    String,
//}
//
//enum Event {
//    Click(),
//    KeyPress(),
//    Condition(),
//    BroadCast(String),
//}
//
//struct Property {
//    compute_kind: ComputeKind,
//}
//
//impl Computable for Property {
//    fn get_compute_kind(&self) -> ComputeKind {
//        self.compute_kind
//    }
//}
//
//struct LValue<'a> {
//    value: Value<'a>,
//    name: String,
//    writes: Vec<&'a Instruction<'a>>,
//}
//
//impl LValue {
//    fn new(kind: ValueKind, name: String) -> LValue {
//        LValue {
//            value: Value::new(kind),
//            name,
//            writes: Vec::new(),
//        }
//    }
//}
//
//struct Variable<'a> {
//    lvalue: LValue<'a>,
//    initial_value: Primitive,
//}
//
//impl Variable {
//    fn new(name: String, initial_value: Primitive) -> Variable {
//        Variable {
//            lvalue: LValue::new(ValueKind::Variable, name),
//            initial_value,
//        }
//    }
//}
//
//struct List<'a> {
//    lvalue: LValue<'a>,
//    initial_values: Vec<Primitive>,
//}
//
//impl List {
//    fn new(name: String, initial_values: Vec<Primitive>) -> List {
//        List {
//            lvalue: LValue::new(ValueKind::List, name),
//            initial_values,
//        }
//    }
//}
