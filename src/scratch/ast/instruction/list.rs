use crate::scratch::ast::instruction::Value;
use crate::scratch::ast::compute_kind::{Computable, ComputeKind};

pub enum ListInstruction<'a> {
    Void(VoidListInstruction<'a>),
    Returning(ReturningListInstruction<'a>),
}

impl Computable for ListInstruction<'_> {
    fn get_compute_kind(&self) -> ComputeKind {
        match self {
            ListInstruction::Void(op) => op.get_compute_kind(),
            ListInstruction::Returning(op) => op.get_compute_kind(),
        }
    }
}

pub enum VoidListInstruction<'a> {
    Set(Value<'a>, Value<'a>),
    Insert(Value<'a>, Value<'a>),
    Remove(Value<'a>),
    Push(Value<'a>, PushPop),
    // Push is like a constant time version of Insert and Remove
    Clear(),
}

impl Computable for VoidListInstruction<'_> {
    fn get_compute_kind(&self) -> ComputeKind {
        match self {
            VoidListInstruction::Set(index, value) => (index, value).get_compute_kind(),
            VoidListInstruction::Insert(index, value) => (index, value).get_compute_kind(),
            VoidListInstruction::Remove(value) => value.get_compute_kind(),
            VoidListInstruction::Push(value, _) => value.get_compute_kind(),
            VoidListInstruction::Clear() => ComputeKind::Computational,
        }
    }
}

pub enum ReturningListInstruction<'a> {
    Length(),
    Get(Value<'a>),
    Pop(PushPop),
    Find(Value<'a>),
    Contains(Value<'a>),
    ContainsSeq(&'a Value<'a>),
    Concat(&'a Value<'a>),
}

impl Computable for ReturningListInstruction<'_> {
    fn get_compute_kind(&self) -> ComputeKind {
        match self {
            ReturningListInstruction::Length() => ComputeKind::Computational,
            ReturningListInstruction::Get(index) => index.get_compute_kind(),
            ReturningListInstruction::Pop(_) => ComputeKind::Computational,
            ReturningListInstruction::Find(value) => value.get_compute_kind(),
            ReturningListInstruction::Contains(value) => value.get_compute_kind(),
            ReturningListInstruction::ContainsSeq(value) => value.get_compute_kind(),
            ReturningListInstruction::Concat(value) => value.get_compute_kind(),
        }
    }
}

pub struct PushPop {
    side: PushPopSide,
    index: u8,
    // also support popping the nth to last element, where n is a small constant
    // 0 is the normal behavior (e.x. pop the 0th element)
}

pub enum PushPopSide {
    Left,
    Right,
}
