use std::ptr::slice_from_raw_parts_mut;

use bumpalo::Bump;
use bumpalo::collections::{String, Vec};

use crate::scratch::ast::compute_kind::{Computable, ComputeKind};
use crate::scratch::ast::instruction::{GetInstruction, Instruction, ReadWriteInstruction, SetInstruction, Value};
use crate::scratch::ast::instruction::function_call::{BinaryOp, CallInstruction, UnaryOp};

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
    instructions: Vec<'a, Instruction<'a>>,
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
    variables: Vec<'a, Variable<'a>>,
    lists: Vec<'a, List<'a>>,
}

pub struct SpriteScripts<'a> {
    sprite: &'a Sprite,
    locals: Scope<'a>,
    scripts: Vec<'a, Script<'a>>,
}

pub struct Function<'a> {
    name: String<'a>,
    params: Vec<'a, Variable<'a>>,
    body: Block<'a>,
}

impl<'a> Function<'a> {
    pub fn params(&self) -> &[Variable<'a>] {
        &self.params[..]
    }
    
    fn params_mut(&self) -> &mut [Variable<'a>] {
        let params = self.params();
        let ptr = params.as_ptr();
        let mut_ptr = ptr as *mut Variable<'a>;
        let slice = slice_from_raw_parts_mut(mut_ptr, params.len());
        unsafe {
            &mut *slice
        }
    }
    
    fn get_compute_kind(&'a self, args: &'a [Value<'a>]) -> ComputeKind {
        // copy args into params, then evaluate body's compute kind
        let params = self.params_mut();
        params
            .iter_mut()
            .zip(args.iter())
            .for_each(|(param, arg)| param.value = *arg);
        self.body.get_compute_kind()
    }
}

pub struct Program<'a> {
    globals: Scope<'a>,
    sprite_scripts: Vec<'a, SpriteScripts<'a>>,
    functions: Vec<'a, Function<'a>>,
}

pub struct Variable<'a> {
    name: String<'a>,
    value: Value<'a>,
    // TODO should this be Option?
    // set to initial value for global, set to arg for function param
    reads: Vec<'a, &'a ReadWriteInstruction<'a>>,
    writes: Vec<'a, &'a ReadWriteInstruction<'a>>,
}

impl Computable for Variable<'_> {
    fn get_compute_kind(&self) -> ComputeKind {
        self.value.get_compute_kind()
    }
}

pub struct List<'a> {
    variable: Variable<'a>,
    // should be empty as Scratch allows it, except for strings
    element_reads: Vec<'a, &'a GetInstruction<'a>>,
    element_writes: Vec<'a, &'a Value<'a>>,
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
        CallInstruction::Id(self.alloc(get))
    }
    
    fn op1(&'a self, op: UnaryOp, value: Value<'a>) -> Value<'a> {
        CallInstruction::UnaryOp {
            op,
            value: self.alloc(value),
        }
    }
    
    fn op2(&'a self, op: BinaryOp, left: Value<'a>, right: Value<'a>) -> Value<'a> {
        CallInstruction::BinaryOp {
            op,
            left: self.alloc(left),
            right: self.alloc(right),
        }
    }
    
    //    fn call(&'a self, func: &'a Function<'a>)
    
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
