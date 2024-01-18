#![no_std]
extern crate alloc;
use alloc::vec::Vec;
use stack::Stack;
pub mod opcodes;
pub mod parser;
pub mod stack;
pub mod types;
pub mod instruction;

struct Executor<'a> {
    bytecode: &'a [u8],
    position: usize,
    stack: Stack,
}

impl Executor<'_> {
}
