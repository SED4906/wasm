use nom::{bytes::complete::take, combinator::peek, IResult, error::ErrorKind, multi::{many0, many_m_n, count}, branch::alt};

use crate::{opcodes::{ExtendedOpcode1, ExtendedOpcode2, FullOpcode, Opcode}, instruction::Instruction, types::{BlockType, LabelIndex}};

pub fn parse_instruction(input: &[u8]) -> IResult<&[u8], Instruction> {
    alt((parse_instruction_block, parse_instruction_loop, parse_instruction_if, parse_instruction_if_else, parse_instruction1))(input)
}

pub fn parse_instruction1(input: &[u8]) -> IResult<&[u8], Instruction> {
    let (input, opcode) = parse_opcode(input)?;
    match opcode {
        FullOpcode::OneByte(Opcode::Unreachable) => Ok((input,Instruction::Unreachable)),
        FullOpcode::OneByte(Opcode::Nop) => Ok((input,Instruction::Nop)),
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
    let (input, bytes_to_check) = peek(take(32_usize.div_ceil(7)))(input)?;
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

pub fn parse_leb128s33(input: &[u8]) -> IResult<&[u8], i64> {
    let (input, bytes_to_check) = peek(take(33_usize.div_ceil(7)))(input)?;
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
