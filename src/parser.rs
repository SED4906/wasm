use alloc::vec::Vec;
use nom::{bytes::complete::{take, take_while_m_n}, combinator::{peek, recognize}, IResult, error::ErrorKind, multi::{many0, count}, branch::alt, sequence::pair};

use crate::{opcodes::{ExtendedOpcode1, ExtendedOpcode2, FullOpcode, Opcode}, instruction::Instruction, types::{BlockType, LabelIndex, FuncIndex, TypeIndex, TableIndex, ValueType}};

pub fn parse_expression(input: &[u8]) -> IResult<&[u8], Vec<Instruction>> {
    let (input, expr) = many0(parse_instruction)(input)?;
    let (input, _) = tag_opcode(FullOpcode::OneByte(Opcode::End))(input)?;
    Ok((input, expr))
}

pub fn parse_instruction(input: &[u8]) -> IResult<&[u8], Instruction> {
    alt((parse_instruction_block, parse_instruction_loop, parse_instruction_if, parse_instruction_if_else, parse_instruction1))(input)
}

pub fn parse_instruction1(input: &[u8]) -> IResult<&[u8], Instruction> {
    let (input, opcode) = parse_opcode(input)?;
    match opcode {
        FullOpcode::OneByte(Opcode::Unreachable) => Ok((input,Instruction::Unreachable)),
        FullOpcode::OneByte(Opcode::Nop) => Ok((input,Instruction::Nop)),
        FullOpcode::OneByte(Opcode::Return) => Ok((input,Instruction::Return)),
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
        FullOpcode::OneByte(Opcode::I32EqualZero) => Ok((input, Instruction::I32EqualZero)),
        FullOpcode::OneByte(Opcode::I32NotEqual) => Ok((input, Instruction::I32NotEqual)),
        FullOpcode::OneByte(Opcode::I32LessThanS) => Ok((input, Instruction::I32LessThanS)),
        FullOpcode::OneByte(Opcode::I32LessThanU) => Ok((input, Instruction::I32LessThanU)),
        FullOpcode::OneByte(Opcode::I32GreaterThanS) => Ok((input, Instruction::I32GreaterThanS)),
        FullOpcode::OneByte(Opcode::I32GreaterThanU) => Ok((input, Instruction::I32GreaterThanU)),
        FullOpcode::OneByte(Opcode::I32LessEqualS) => Ok((input, Instruction::I32LessEqualS)),
        FullOpcode::OneByte(Opcode::I32LessEqualU) => Ok((input, Instruction::I32LessEqualU)),
        FullOpcode::OneByte(Opcode::I32GreaterEqualS) => Ok((input, Instruction::I32GreaterEqualS)),
        FullOpcode::OneByte(Opcode::I32GreaterEqualU) => Ok((input, Instruction::I32GreaterEqualU)),
        FullOpcode::OneByte(Opcode::I64EqualZero) => Ok((input, Instruction::I64EqualZero)),
        FullOpcode::OneByte(Opcode::I64NotEqual) => Ok((input, Instruction::I64NotEqual)),
        FullOpcode::OneByte(Opcode::I64LessThanS) => Ok((input, Instruction::I64LessThanS)),
        FullOpcode::OneByte(Opcode::I64LessThanU) => Ok((input, Instruction::I64LessThanU)),
        FullOpcode::OneByte(Opcode::I64GreaterThanS) => Ok((input, Instruction::I64GreaterThanS)),
        FullOpcode::OneByte(Opcode::I64GreaterThanU) => Ok((input, Instruction::I64GreaterThanU)),
        FullOpcode::OneByte(Opcode::I64LessEqualS) => Ok((input, Instruction::I64LessEqualS)),
        FullOpcode::OneByte(Opcode::I64LessEqualU) => Ok((input, Instruction::I64LessEqualU)),
        FullOpcode::OneByte(Opcode::I64GreaterEqualS) => Ok((input, Instruction::I64GreaterEqualS)),
        FullOpcode::OneByte(Opcode::I64GreaterEqualU) => Ok((input, Instruction::I64GreaterEqualU)),
        FullOpcode::OneByte(Opcode::F32Equal) => Ok((input, Instruction::F32Equal)),
        FullOpcode::OneByte(Opcode::F32NotEqual) => Ok((input, Instruction::F32NotEqual)),
        FullOpcode::OneByte(Opcode::F32LessThan) => Ok((input, Instruction::F32LessThan)),
        FullOpcode::OneByte(Opcode::F32GreaterThan) => Ok((input, Instruction::F32GreaterThan)),
        FullOpcode::OneByte(Opcode::F32LessEqual) => Ok((input, Instruction::F32LessEqual)),
        FullOpcode::OneByte(Opcode::F32GreaterEqual) => Ok((input, Instruction::F32GreaterEqual)),
        FullOpcode::OneByte(Opcode::F64Equal) => Ok((input, Instruction::F64Equal)),
        FullOpcode::OneByte(Opcode::F64NotEqual) => Ok((input, Instruction::F64NotEqual)),
        FullOpcode::OneByte(Opcode::F64LessThan) => Ok((input, Instruction::F64LessThan)),
        FullOpcode::OneByte(Opcode::F64GreaterThan) => Ok((input, Instruction::F64GreaterThan)),
        FullOpcode::OneByte(Opcode::F64LessEqual) => Ok((input, Instruction::F64LessEqual)),
        FullOpcode::OneByte(Opcode::F64GreaterEqual) => Ok((input, Instruction::F64GreaterEqual)),
        FullOpcode::OneByte(Opcode::I32Clz) => Ok((input, Instruction::I32Clz)),
        FullOpcode::OneByte(Opcode::I32Ctz) => Ok((input, Instruction::I32Ctz)),
        FullOpcode::OneByte(Opcode::I32Popcount) => Ok((input, Instruction::I32Popcount)),
        FullOpcode::OneByte(Opcode::I32Add) => Ok((input, Instruction::I32Add)),
        FullOpcode::OneByte(Opcode::I32Subtract) => Ok((input, Instruction::I32Subtract)),
        FullOpcode::OneByte(Opcode::I32Multiply) => Ok((input, Instruction::I32Multiply)),
        FullOpcode::OneByte(Opcode::I32DivideS) => Ok((input, Instruction::I32DivideS)),
        FullOpcode::OneByte(Opcode::I32DivideU) => Ok((input, Instruction::I32DivideU)),
        FullOpcode::OneByte(Opcode::I32RemainderS) => Ok((input, Instruction::I32RemainderS)),
        FullOpcode::OneByte(Opcode::I32RemainderU) => Ok((input, Instruction::I32RemainderU)),
        FullOpcode::OneByte(Opcode::I32And) => Ok((input, Instruction::I32And)),
        FullOpcode::OneByte(Opcode::I32Or) => Ok((input, Instruction::I32Or)),
        FullOpcode::OneByte(Opcode::I32Xor) => Ok((input, Instruction::I32Xor)),
        FullOpcode::OneByte(Opcode::I32ShiftLeft) => Ok((input, Instruction::I32ShiftLeft)),
        FullOpcode::OneByte(Opcode::I32ShiftRightS) => Ok((input, Instruction::I32ShiftRightS)),
        FullOpcode::OneByte(Opcode::I32ShiftRightU) => Ok((input, Instruction::I32ShiftRightU)),
        FullOpcode::OneByte(Opcode::I32RotateLeft) => Ok((input, Instruction::I32RotateLeft)),
        FullOpcode::OneByte(Opcode::I32RotateRight) => Ok((input, Instruction::I32RotateRight)),
        FullOpcode::OneByte(Opcode::I64Clz) => Ok((input, Instruction::I64Clz)),
        FullOpcode::OneByte(Opcode::I64Ctz) => Ok((input, Instruction::I64Ctz)),
        FullOpcode::OneByte(Opcode::I64Popcount) => Ok((input, Instruction::I64Popcount)),
        FullOpcode::OneByte(Opcode::I64Add) => Ok((input, Instruction::I64Add)),
        FullOpcode::OneByte(Opcode::I64Subtract) => Ok((input, Instruction::I64Subtract)),
        FullOpcode::OneByte(Opcode::I64Multiply) => Ok((input, Instruction::I64Multiply)),
        FullOpcode::OneByte(Opcode::I64DivideS) => Ok((input, Instruction::I64DivideS)),
        FullOpcode::OneByte(Opcode::I64DivideU) => Ok((input, Instruction::I64DivideU)),
        FullOpcode::OneByte(Opcode::I64RemainderS) => Ok((input, Instruction::I64RemainderS)),
        FullOpcode::OneByte(Opcode::I64RemainderU) => Ok((input, Instruction::I64RemainderU)),
        FullOpcode::OneByte(Opcode::I64And) => Ok((input, Instruction::I64And)),
        FullOpcode::OneByte(Opcode::I64Or) => Ok((input, Instruction::I64Or)),
        FullOpcode::OneByte(Opcode::I64Xor) => Ok((input, Instruction::I64Xor)),
        FullOpcode::OneByte(Opcode::I64ShiftLeft) => Ok((input, Instruction::I64ShiftLeft)),
        FullOpcode::OneByte(Opcode::I64ShiftRightS) => Ok((input, Instruction::I64ShiftRightS)),
        FullOpcode::OneByte(Opcode::I64ShiftRightU) => Ok((input, Instruction::I64ShiftRightU)),
        FullOpcode::OneByte(Opcode::I64RotateLeft) => Ok((input, Instruction::I64RotateLeft)),
        FullOpcode::OneByte(Opcode::I64RotateRight) => Ok((input, Instruction::I64RotateRight)),
        FullOpcode::OneByte(Opcode::F32Absolute) => Ok((input, Instruction::F32Absolute)),
        FullOpcode::OneByte(Opcode::F32Negate) => Ok((input, Instruction::F32Negate)),
        FullOpcode::OneByte(Opcode::F32Ceiling) => Ok((input, Instruction::F32Ceiling)),
        FullOpcode::OneByte(Opcode::F32Floor) => Ok((input, Instruction::F32Floor)),
        FullOpcode::OneByte(Opcode::F32Truncate) => Ok((input, Instruction::F32Truncate)),
        FullOpcode::OneByte(Opcode::F32Nearest) => Ok((input, Instruction::F32Nearest)),
        FullOpcode::OneByte(Opcode::F32SquareRoot) => Ok((input, Instruction::F32SquareRoot)),
        FullOpcode::OneByte(Opcode::F32Add) => Ok((input, Instruction::F32Add)),
        FullOpcode::OneByte(Opcode::F32Subtract) => Ok((input, Instruction::F32Subtract)),
        FullOpcode::OneByte(Opcode::F32Multiply) => Ok((input, Instruction::F32Multiply)),
        FullOpcode::OneByte(Opcode::F32Divide) => Ok((input, Instruction::F32Divide)),
        FullOpcode::OneByte(Opcode::F32Minimum) => Ok((input, Instruction::F32Minimum)),
        FullOpcode::OneByte(Opcode::F32Maximum) => Ok((input, Instruction::F32Maximum)),
        FullOpcode::OneByte(Opcode::F32CopySign) => Ok((input, Instruction::F32CopySign)),
        FullOpcode::OneByte(Opcode::F64Absolute) => Ok((input, Instruction::F64Absolute)),
        FullOpcode::OneByte(Opcode::F64Negate) => Ok((input, Instruction::F64Negate)),
        FullOpcode::OneByte(Opcode::F64Ceiling) => Ok((input, Instruction::F64Ceiling)),
        FullOpcode::OneByte(Opcode::F64Floor) => Ok((input, Instruction::F64Floor)),
        FullOpcode::OneByte(Opcode::F64Truncate) => Ok((input, Instruction::F64Truncate)),
        FullOpcode::OneByte(Opcode::F64Nearest) => Ok((input, Instruction::F64Nearest)),
        FullOpcode::OneByte(Opcode::F64SquareRoot) => Ok((input, Instruction::F64SquareRoot)),
        FullOpcode::OneByte(Opcode::F64Add) => Ok((input, Instruction::F64Add)),
        FullOpcode::OneByte(Opcode::F64Subtract) => Ok((input, Instruction::F64Subtract)),
        FullOpcode::OneByte(Opcode::F64Multiply) => Ok((input, Instruction::F64Multiply)),
        FullOpcode::OneByte(Opcode::F64Divide) => Ok((input, Instruction::F64Divide)),
        FullOpcode::OneByte(Opcode::F64Minimum) => Ok((input, Instruction::F64Minimum)),
        FullOpcode::OneByte(Opcode::F64Maximum) => Ok((input, Instruction::F64Maximum)),
        FullOpcode::OneByte(Opcode::F64CopySign) => Ok((input, Instruction::F64CopySign)),
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
