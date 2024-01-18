use alloc::vec::Vec;
use nom::{bytes::complete::{take, take_while_m_n}, combinator::{peek, recognize}, IResult, error::ErrorKind, multi::{many0, count}, branch::alt, sequence::pair};

use crate::{opcodes::{ExtendedOpcode1, ExtendedOpcode2, FullOpcode, Opcode}, instruction::Instruction, types::{BlockType, LabelIndex, FuncIndex, TypeIndex, TableIndex, ValueType}};

pub fn parse_instruction(input: &[u8]) -> IResult<&[u8], Instruction> {
    alt((parse_instruction_block, parse_instruction_loop, parse_instruction_if, parse_instruction_if_else, parse_instruction1))(input)
}

pub fn parse_instruction1(input: &[u8]) -> IResult<&[u8], Instruction> {
    let (input, opcode) = parse_opcode(input)?;
    match opcode {
        FullOpcode::OneByte(Opcode::Unreachable) => Ok((input,Instruction::Unreachable)),
        FullOpcode::OneByte(Opcode::Nop) => Ok((input,Instruction::Nop)),
        FullOpcode::OneByte(Opcode::Return) => Ok((input,Instruction::Return)),
        FullOpcode::OneByte(Opcode::End) => Ok((input,Instruction::End)),
        FullOpcode::OneByte(Opcode::Branch) => {
            let (input, label) = parse_leb128u32(input)?;
            Ok((input, Instruction::Branch(label as LabelIndex)))
        }
        FullOpcode::OneByte(Opcode::BranchIf) => {
            let (input, label) = parse_leb128u32(input)?;
            Ok((input, Instruction::BranchIf(label as LabelIndex)))
        }
        FullOpcode::OneByte(Opcode::BranchTable) => {
            let (input, table_length) = parse_leb128u32(input)?;
            let (input, table) = count(parse_leb128u32, table_length as usize)(input)?;
            let (input, label) = parse_leb128u32(input)?;
            Ok((input, Instruction::BranchTable(table,label as LabelIndex)))
        }
        FullOpcode::OneByte(Opcode::Return) => Ok((input,Instruction::Return)),
        FullOpcode::OneByte(Opcode::Call) => {
            let (input, func) = parse_leb128u32(input)?;
            Ok((input, Instruction::Call(func as FuncIndex)))
        }
        FullOpcode::OneByte(Opcode::CallIndirect) => {
            let (input, typ) = parse_leb128u32(input)?;
            let (input, tab) = parse_leb128u32(input)?;
            Ok((input, Instruction::CallIndirect(typ as TypeIndex, tab as TableIndex)))
        }
        FullOpcode::OneByte(Opcode::RefNull) => {
            let (input, byte) = take(1usize)(input)?;
            Ok((input, Instruction::RefNull(byte[0].try_into().expect("invalid reference type"))))
        }
        FullOpcode::OneByte(Opcode::RefIsNull) => Ok((input, Instruction::RefIsNull)),
        FullOpcode::OneByte(Opcode::RefFunc) => {
            let (input, func) = parse_leb128u32(input)?;
            Ok((input, Instruction::RefFunc(func as FuncIndex)))
        }
        FullOpcode::OneByte(Opcode::Drop) => Ok((input, Instruction::Drop)),
        FullOpcode::OneByte(Opcode::SelectNumeric) => Ok((input, Instruction::Select)),
        FullOpcode::OneByte(Opcode::Select) => {
            let (input, table_length) = parse_leb128u32(input)?;
            let (input, table) = take(table_length as usize)(input)?;
            Ok((input, Instruction::SelectTyped(table.into_iter().map(|byte| (*byte).try_into().unwrap()).collect::<Vec<ValueType>>())))
        }
        FullOpcode::OneByte(Opcode::LocalGet) => {
            let (input, localindex) = parse_leb128u32(input)?;
            Ok((input, Instruction::LocalGet(localindex)))
        }
        FullOpcode::OneByte(Opcode::LocalSet) => {
            let (input, localindex) = parse_leb128u32(input)?;
            Ok((input, Instruction::LocalSet(localindex)))
        }
        FullOpcode::OneByte(Opcode::LocalTee) => {
            let (input, localindex) = parse_leb128u32(input)?;
            Ok((input, Instruction::LocalTee(localindex)))
        }
        FullOpcode::OneByte(Opcode::GlobalGet) => {
            let (input, globalindex) = parse_leb128u32(input)?;
            Ok((input, Instruction::GlobalGet(globalindex)))
        }
        FullOpcode::OneByte(Opcode::GlobalSet) => {
            let (input, globalindex) = parse_leb128u32(input)?;
            Ok((input, Instruction::GlobalSet(globalindex)))
        }
        FullOpcode::OneByte(Opcode::TableGet) => {
            let (input,tableindex) = parse_leb128u32(input)?;
            Ok((input, Instruction::TableGet(tableindex)))
        }
        FullOpcode::OneByte(Opcode::TableSet) => {
            let (input,tableindex) = parse_leb128u32(input)?;
            Ok((input, Instruction::TableSet(tableindex)))
        }
        FullOpcode::Extended1(ExtendedOpcode1::TableInit) => {
            let (input,elemindex) = parse_leb128u32(input)?;
            let (input,tableindex) = parse_leb128u32(input)?;
            Ok((input, Instruction::TableInit(elemindex, tableindex)))
        }
        FullOpcode::Extended1(ExtendedOpcode1::ElemDrop) => {
            let (input,elemindex) = parse_leb128u32(input)?;
            Ok((input, Instruction::ElemDrop(elemindex)))
        }
        FullOpcode::Extended1(ExtendedOpcode1::TableCopy) => {
            let (input,tableindex1) = parse_leb128u32(input)?;
            let (input,tableindex2) = parse_leb128u32(input)?;
            Ok((input, Instruction::TableCopy(tableindex1, tableindex2)))
        }
        FullOpcode::Extended1(ExtendedOpcode1::TableGrow) => {
            let (input,tableindex) = parse_leb128u32(input)?;
            Ok((input, Instruction::TableGrow(tableindex)))
        }
        FullOpcode::Extended1(ExtendedOpcode1::TableSize) => {
            let (input,tableindex) = parse_leb128u32(input)?;
            Ok((input, Instruction::TableSize(tableindex)))
        }
        FullOpcode::Extended1(ExtendedOpcode1::TableFill) => {
            let (input,tableindex) = parse_leb128u32(input)?;
            Ok((input, Instruction::TableFill(tableindex)))
        }
        FullOpcode::OneByte(Opcode::I32Load) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::I32Load(align, offset)))
        }
        FullOpcode::OneByte(Opcode::I64Load) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::I64Load(align, offset)))
        }
        FullOpcode::OneByte(Opcode::F32Load) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::F32Load(align, offset)))
        }
        FullOpcode::OneByte(Opcode::F64Load) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::F64Load(align, offset)))
        }
        FullOpcode::OneByte(Opcode::I32Load8S) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::I32Load8S(align, offset)))
        }
        FullOpcode::OneByte(Opcode::I32Load8U) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::I32Load8U(align, offset)))
        }
        FullOpcode::OneByte(Opcode::I32Load16S) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::I32Load16S(align, offset)))
        }
        FullOpcode::OneByte(Opcode::I32Load16U) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::I32Load16U(align, offset)))
        }
        FullOpcode::OneByte(Opcode::I64Load8S) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::I64Load8S(align, offset)))
        }
        FullOpcode::OneByte(Opcode::I64Load8U) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::I64Load8U(align, offset)))
        }
        FullOpcode::OneByte(Opcode::I64Load16S) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::I64Load16S(align, offset)))
        }
        FullOpcode::OneByte(Opcode::I64Load16U) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::I64Load16U(align, offset)))
        }
        FullOpcode::OneByte(Opcode::I64Load32S) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::I64Load32S(align, offset)))
        }
        FullOpcode::OneByte(Opcode::I64Load32U) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::I64Load32U(align, offset)))
        }
        FullOpcode::OneByte(Opcode::I32Store) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::I32Store(align, offset)))
        }
        FullOpcode::OneByte(Opcode::I64Store) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::I64Store(align, offset)))
        }
        FullOpcode::OneByte(Opcode::F32Store) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::F32Store(align, offset)))
        }
        FullOpcode::OneByte(Opcode::F64Store) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::F64Store(align, offset)))
        }
        FullOpcode::OneByte(Opcode::I32Store8) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::I32Store8(align, offset)))
        }
        FullOpcode::OneByte(Opcode::I32Store16) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::I32Store16(align, offset)))
        }
        FullOpcode::OneByte(Opcode::I64Store8) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::I64Store8(align, offset)))
        }
        FullOpcode::OneByte(Opcode::I64Store16) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::I64Store16(align, offset)))
        }
        FullOpcode::OneByte(Opcode::I64Store32) => {
            let (input,align) = parse_leb128u32(input)?;
            let (input,offset) = parse_leb128u32(input)?;
            Ok((input, Instruction::I64Store32(align, offset)))
        }
        FullOpcode::OneByte(Opcode::MemorySize) => {
            let (input, _) = take(1usize)(input)?;
            Ok((input, Instruction::MemorySize))
        }
        FullOpcode::OneByte(Opcode::MemoryGrow) => {
            let (input, _) = take(1usize)(input)?;
            Ok((input, Instruction::MemoryGrow))
        }
        FullOpcode::Extended1(ExtendedOpcode1::MemoryInit) => {
            let (input, dataindex) = parse_leb128u32(input)?;
            let (input, _) = take(1usize)(input)?;
            Ok((input, Instruction::MemoryInit(dataindex)))
        }
        FullOpcode::Extended1(ExtendedOpcode1::DataDrop) => {
            let (input, dataindex) = parse_leb128u32(input)?;
            Ok((input, Instruction::DataDrop(dataindex)))
        }
        FullOpcode::Extended1(ExtendedOpcode1::MemoryCopy) => {
            let (input, _) = take(2usize)(input)?;
            Ok((input, Instruction::MemoryCopy))
        }
        FullOpcode::Extended1(ExtendedOpcode1::MemoryFill) => {
            let (input, _) = take(1usize)(input)?;
            Ok((input, Instruction::MemoryFill))
        }
        FullOpcode::OneByte(Opcode::I32Constant) => {
            let (input, value) = parse_leb128u32(input)?;
            Ok((input, Instruction::I32Const(value)))
        }
        FullOpcode::OneByte(Opcode::I64Constant) => {
            let (input, value) = parse_leb128u64(input)?;
            Ok((input, Instruction::I64Const(value)))
        }
        FullOpcode::OneByte(Opcode::F32Constant) => {
            let (input, value) = nom::number::complete::le_f32(input)?;
            Ok((input, Instruction::F32Const(value)))
        }
        FullOpcode::OneByte(Opcode::F64Constant) => {
            let (input, value) = nom::number::complete::le_f64(input)?;
            Ok((input, Instruction::F64Const(value)))
        }
        _ => Ok((input, Instruction::UnimplementedInstruction)),
    }
}

pub fn parse_instruction_block(input: &[u8]) -> IResult<&[u8], Instruction> {
    let (input, _) = tag_opcode(FullOpcode::OneByte(Opcode::Block))(input)?;
    let (input, blocktype) = parse_leb128s33(input)?;
    let (input, instructions) = many0(parse_instruction)(input)?;
    let (input, _) = tag_opcode(FullOpcode::OneByte(Opcode::End))(input)?;
    Ok((input, Instruction::Block(BlockType::try_from(blocktype).unwrap(), instructions)))
}

pub fn parse_instruction_loop(input: &[u8]) -> IResult<&[u8], Instruction> {
    let (input, _) = tag_opcode(FullOpcode::OneByte(Opcode::Loop))(input)?;
    let (input, blocktype) = parse_leb128s33(input)?;
    let (input, instructions) = many0(parse_instruction)(input)?;
    let (input, _) = tag_opcode(FullOpcode::OneByte(Opcode::End))(input)?;
    Ok((input, Instruction::Loop(BlockType::try_from(blocktype).unwrap(), instructions)))
}

pub fn parse_instruction_if(input: &[u8]) -> IResult<&[u8], Instruction> {
    let (input, _) = tag_opcode(FullOpcode::OneByte(Opcode::If))(input)?;
    let (input, blocktype) = parse_leb128s33(input)?;
    let (input, instructions) = many0(parse_instruction)(input)?;
    let (input, _) = tag_opcode(FullOpcode::OneByte(Opcode::End))(input)?;
    Ok((input, Instruction::If(BlockType::try_from(blocktype).unwrap(), instructions)))
}

pub fn parse_instruction_if_else(input: &[u8]) -> IResult<&[u8], Instruction> {
    let (input, _) = tag_opcode(FullOpcode::OneByte(Opcode::If))(input)?;
    let (input, blocktype) = parse_leb128s33(input)?;
    let (input, instructions1) = many0(parse_instruction)(input)?;
    let (input, _) = tag_opcode(FullOpcode::OneByte(Opcode::Else))(input)?;
    let (input, instructions2) = many0(parse_instruction)(input)?;
    let (input, _) = tag_opcode(FullOpcode::OneByte(Opcode::End))(input)?;
    Ok((input, Instruction::IfElse(BlockType::try_from(blocktype).unwrap(), instructions1, instructions2)))
}

pub fn tag_opcode(opcode: FullOpcode) -> impl FnMut(&[u8]) -> IResult<&[u8], FullOpcode> {
    move |input| {
        let (input, full_opcode) = parse_opcode(input)?;
        if full_opcode == opcode {
            Ok((input, full_opcode))
        } else {
            Err(nom::Err::Error(nom::error::Error{input, code: ErrorKind::Tag}))
        }
    }
}


pub fn parse_opcode(input: &[u8]) -> IResult<&[u8], FullOpcode> {
    let (input, byte) = take(1usize)(input)?;
    if let Ok(opcode) = Opcode::try_from(byte[0]) {
        match opcode {
            Opcode::ExtendedOpcode1 => {
                let (input, extop1) = parse_extended_opcode_1(input)?;
                return Ok((input, FullOpcode::Extended1(extop1)));
            }
            Opcode::ExtendedOpcode2 => {
                let (input, extop2) = parse_extended_opcode_2(input)?;
                return Ok((input, FullOpcode::Extended2(extop2)));
            }
            _ => return Ok((input, FullOpcode::OneByte(opcode))),
        }
    }
    Err(nom::Err::Error(nom::error::Error{input, code: ErrorKind::Tag}))
}

pub fn parse_extended_opcode_1(input: &[u8]) -> IResult<&[u8], ExtendedOpcode1> {
    let (input, param) = parse_leb128u32(input)?;
    if let Ok(extop) = ExtendedOpcode1::try_from(param) {
        return Ok((input, extop));
    }
    Err(nom::Err::Error(nom::error::Error{input, code: ErrorKind::Tag}))
}

pub fn parse_extended_opcode_2(input: &[u8]) -> IResult<&[u8], ExtendedOpcode2> {
    let (input, param) = parse_leb128u32(input)?;
    if let Ok(extop) = ExtendedOpcode2::try_from(param) {
        return Ok((input, extop));
    }
    Err(nom::Err::Error(nom::error::Error{input, code: ErrorKind::Tag}))
}

pub fn parse_leb128u32(input: &[u8]) -> IResult<&[u8], u32> {
    let (input, bytes_to_check) = peek(recognize(pair(take_while_m_n(1,64_usize.div_ceil(7), |byte| byte >= 128),take(1usize))))(input)?;
    let mut bytes: usize = 1;
    let mut value: u32 = 0;
    for byte in bytes_to_check {
        value |= (byte << ((bytes - 1) * 7)) as u32;
        if byte & 0x80 == 0 {
            break;
        }
        bytes += 1;
    }
    let (input, _) = take(bytes)(input)?;
    Ok((input, value))
}

pub fn parse_leb128u64(input: &[u8]) -> IResult<&[u8], u64> {
    let (input, bytes_to_check) = peek(recognize(pair(take_while_m_n(1,64_usize.div_ceil(7), |byte| byte >= 128),take(1usize))))(input)?;
    let mut bytes: usize = 1;
    let mut value: u64 = 0;
    for byte in bytes_to_check {
        value |= (byte << ((bytes - 1) * 7)) as u64;
        if byte & 0x80 == 0 {
            break;
        }
        bytes += 1;
    }
    let (input, _) = take(bytes)(input)?;
    Ok((input, value))
}


pub fn parse_leb128s33(input: &[u8]) -> IResult<&[u8], i64> {
    let (input, bytes_to_check) = peek(recognize(pair(take_while_m_n(1,64_usize.div_ceil(7), |byte| byte >= 128),take(1usize))))(input)?;
    let mut bytes: usize = 1;
    let mut value: i64 = 0;
    let mut last_byte = 0;
    for byte in bytes_to_check {
        value |= (byte << ((bytes - 1) * 7)) as i64;
        last_byte = *byte;
        if byte & 0x80 == 0 {
            break;
        }
        bytes += 1;
    }
    if last_byte & 0x40 == 0x40 {
        value |= !0 << (bytes * 7);
    }
    let (input, _) = take(bytes)(input)?;
    Ok((input, value))
}
