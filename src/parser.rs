use alloc::vec::Vec;

use crate::{opcodes::{ExtendedOpcode1, ExtendedOpcode2, Opcode}, types::{BlockType, LabelIndex, FuncIndex, TypeIndex, TableIndex, ValueType}};

struct Parser<'a> {
    bytecode: &'a [u8],
    position: usize,
}

impl Parser<'_> {
    pub fn consume_byte(&mut self) -> Result<u8, ParserError> {
        let byte = self.peek_byte()?;
        self.position += 1;
        Ok(byte)
    }
    pub fn peek_byte(&mut self) -> Result<u8, ParserError> {
        match self.bytecode.get(self.position) {
            Some(byte) => Ok(*byte),
            None => Err(ParserError::Incomplete),
        }
    }
    pub fn consume_while(&mut self, pred: impl Fn(u8) -> bool) -> Result<Vec<u8>, ParserError> {
        let mut bytes = Vec::new();
        while pred(self.peek_byte()?) {
            bytes.push(self.consume_byte()?);
        }
        Ok(bytes)
    }
    pub fn consume_leb128(&mut self, bits: usize, signed: bool) -> Result<u128,ParserError> {
        let mut value = 0;
        let main_bytes = self.consume_while(|b| b&0x80==0x80)?;
        let last_byte = self.consume_byte()?;
        if main_bytes.len() > bits / 7 {
            return Err(ParserError::Invalid);
        }
        for (index,byte) in main_bytes.iter().enumerate() {
            value += ((*byte & 0x7F) as u128) << (index * 7);
        }
        if signed && last_byte & 0x40 == 0x40 {
            value |= (!0u128) << (7 + main_bytes.len() * 7);
        }
        Ok(value)
    }
}

#[derive(Debug)]
enum ParserError {
    Incomplete,
    Invalid,
}