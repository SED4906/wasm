use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Opcode {
    Unreachable = 0x00,
    Nop,
    Block,
    Loop,
    If,
    Else,
    End = 0x0B,
    Branch,
    BranchIf,
    BranchTable,
    Return,
    Call,
    CallIndirect,
    Drop = 0x1A,
    SelectNumeric,
    Select,
    LocalGet = 0x20,
    LocalSet,
    LocalTee,
    GlobalGet,
    GlobalSet,
    TableGet,
    TableSet,
    I32Load = 0x28,
    I64Load,
    F32Load,
    F64Load,
    I32Load8S,
    I32Load8U,
    I32Load16S,
    I32Load16U,
    I64Load8S,
    I64Load8U,
    I64Load16S,
    I64Load16U,
    I64Load32S,
    I64Load32U,
    I32Store,
    I64Store,
    F32Store,
    F64Store,
    I32Store8,
    I32Store16,
    I64Store8,
    I64Store16,
    I64Store32,
    MemorySize,
    MemoryGrow,
    I32Constant,
    I64Constant,
    F32Constant,
    F64Constant,
    I32EqualZero,
    I32Equal,
    I32NotEqual,
    I32LessThanS,
    I32LessThanU,
    I32GreaterThanS,
    I32GreaterThanU,
    I32LessEqualS,
    I32LessEqualU,
    I32GreaterEqualS,
    I32GreaterEqualU,
    I64EqualZero,
    I64Equal,
    I64NotEqual,
    I64LessThanS,
    I64LessThanU,
    I64GreaterThanS,
    I64GreaterThanU,
    I64LessEqualS,
    I64LessEqualU,
    I64GreaterEqualS,
    I64GreaterEqualU,
    F32Equal,
    F32NotEqual,
    F32LessThan,
    F32GreaterThan,
    F32LessEqual,
    F32GreaterEqual,
    F64Equal,
    F64NotEqual,
    F64LessThan,
    F64GreaterThan,
    F64LessEqual,
    F64GreaterEqual,
    I32Clz,
    I32Ctz,
    I32Popcount,
    I32Add,
    I32Subtract,
    I32Multiply,
    I32DivideS,
    I32DivideU,
    I32RemainderS,
    I32RemainderU,
    I32And,
    I32Or,
    I32Xor,
    I32ShiftLeft,
    I32ShiftRightS,
    I32ShiftRightU,
    I32RotateLeft,
    I32RotateRight,
    I64Clz,
    I64Ctz,
    I64Popcount,
    I64Add,
    I64Subtract,
    I64Multiply,
    I64DivideS,
    I64DivideU,
    I64RemainderS,
    I64RemainderU,
    I64And,
    I64Or,
    I64Xor,
    I64ShiftLeft,
    I64ShiftRightS,
    I64ShiftRightU,
    I64RotateLeft,
    I64RotateRight,
    F32Absolute,
    F32Negate,
    F32Ceiling,
    F32Floor,
    F32Truncate,
    F32Nearest,
    F32SquareRoot,
    F32Add,
    F32Subtract,
    F32Multiply,
    F32Divide,
    F32Minimum,
    F32Maximum,
    F32CopySign,
    F64Absolute,
    F64Negate,
    F64Ceiling,
    F64Floor,
    F64Truncate,
    F64Nearest,
    F64SquareRoot,
    F64Add,
    F64Subtract,
    F64Multiply,
    F64Divide,
    F64Minimum,
    F64Maximum,
    F64CopySign,
    I32WrapI64,
    I32TruncateF32S,
    I32TruncateF32U,
    I32TruncateF64S,
    I32TruncateF64U,
    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncateF32S,
    I64TruncateF32U,
    I64TruncateF64S,
    I64TruncateF64U,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F32DemoteF64,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F64PromoteF32,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
    I32Extend8S,
    I32Extend16S,
    I64Extend8S,
    I64Extend16S,
    I64Extend32S,
    RefNull = 0xD0,
    RefIsNull,
    RefFunc,
    ExtendedOpcode1 = 0xFC,
    ExtendedOpcode2,
}

#[derive(PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum ExtendedOpcode1 {
    I32TruncateSaturatingF32S,
    I32TruncateSaturatingF32U,
    I32TruncateSaturatingF64S,
    I32TruncateSaturatingF64U,
    I64TruncateSaturatingF32S,
    I64TruncateSaturatingF32U,
    I64TruncateSaturatingF64S,
    I64TruncateSaturatingF64U,
    MemoryInit,
    DataDrop,
    MemoryCopy,
    MemoryFill,
    TableInit,
    ElemDrop,
    TableCopy,
    TableGrow,
    TableSize,
    TableFill,
}

#[derive(PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum ExtendedOpcode2 {
    V128Load,
    V128Load8x8S,
    V128Load8x8U,
    V128Load16x4S,
    V128Load16x4U,
    V128Load32x2S,
    V128Load32x2U,
    V128Load8Splat,
    V128Load16Splat,
    V128Load32Splat,
    V128Load64Splat,
    V128Store,
    V128Const,
    V128Shuffle,
    I8x16Swizzle,
    I8x16Splat,
    I16x8Splat,
    I32x4Splat,
    I64x2Splat,
    F32x4Splat,
    F64x2Splat,
    I8x16ExtractLaneS,
    I8x16ExtractLaneU,
    I8x16ReplaceLane,
    I16x8ExtractLaneS,
    I16x8ExtractLaneU,
    I16x8ReplaceLane,
    I32x4ExtractLane,
    I32x4ReplaceLane,
    I64x2ExtractLane,
    I64x2ReplaceLane,
    F32x4ExtractLane,
    F32x4ReplaceLane,
    F64x2ExtractLane,
    F64x2ReplaceLane,
    I8x16Equal,
    I8x16NotEqual,
    I8x16LessThanS,
    I8x16LessThanU,
    I8x16GreaterThanS,
    I8x16GreaterThanU,
    I8x16LessEqualS,
    I8x16LessEqualU,
    I8x16GreaterEqualS,
    I8x16GreaterEqualU,
    I16x8Equal,
    I16x8NotEqual,
    I16x8essThanS,
    I16x8LessThanU,
    I16x8GreaterThanS,
    I16x8GreaterThanU,
    I16x8LessEqualS,
    I16x8LessEqualU,
    I16x8GreaterEqualS,
    I16x8GreaterEqualU,
    I32x4Equal,
    I32x4NotEqual,
    I32x4essThanS,
    I32x4LessThanU,
    I32x4GreaterThanS,
    I32x4GreaterThanU,
    I32x4LessEqualS,
    I32x4LessEqualU,
    I32x4GreaterEqualS,
    I32x4GreaterEqualU,
    F32x4Equal,
    F32x4NotEqual,
    F32x4LessThan,
    F32x4GreaterThan,
    F32x4LessEqual,
    F32x4GreaterEqual,
    F64x2Equal,
    F64x2NotEqual,
    F64x2LessThan,
    F64x2GreaterThan,
    F64x2LessEqual,
    F64x2GreaterEqual,
    V128Not,
    V128And,
    V128AndNot,
    V128Or,
    V128Xor,
    V128BitSelect,
    V128AnyTrue,
    V128Load8Lane,
    V128Load16Lane,
    V128Load32Lane,
    V128Load64Lane,
    V128Store8Lane,
    V128Store16Lane,
    V128Store32Lane,
    V128Store64Lane,
    V128Load32Zero,
    V128Load64Zero,
    F32x4DemoteF64x2Zero,
    F64x2PromoteLowF32x4,
    I8x16Absolute,
    I8x16Negate,
    I8x16Popcount,
    I8x16AllTrue,
    I8x16Bitmask,
    I8x16NarrowI16x8S,
    I8x16NarrowI16x8U,
    F32x4Ceiling,
    F32x4Floor,
    F32x4Truncate,
    F32x4Nearest,
    I8x16Shl,
    I8x16ShrS,
    I8x16ShrU,
    I8x16Add,
    I8x16AddSaturatingS,
    I8x16AddSaturatingU,
    I8x16Subtract,
    I8x16SubtractSaturatingS,
    I8x16SubtractSaturatingU,
    F64x2Ceiling,
    F64x2Floor,
    I8x16MinimumS,
    I8x16MinimumU,
    I8x16MaximumS,
    I8x16MaximumU,
    F64x2Truncate,
    I8x16AverageU,
    I16x8ExtendAddPairwiseI8x16S,
    I16x8ExtendAddPairwiseI8x16U,
    I32x4ExtendAddPairwiseI16x8S,
    I32x4ExtendAddPairwiseI16x8U,
    I16x8Absolute,
    I16x8Negate,
    I16x8Q15MulrSaturatingS,
    I16x8AllTrue,
    I16x8Bitmask,
    I16x8NarrowI32x4S,
    I16x8NarrowI32x4U,
    I16x8ExtendLowI8x16S,
    I16x8ExtendHighI8x16S,
    I16x8ExtendLowI8x16U,
    I16x8ExtendHighI8x16U,
    I16x8Shl,
    I16x8ShrS,
    I16x8ShrU,
    I16x8Add,
    I16x8AddSaturatingS,
    I16x8AddSaturatingU,
    I16x8Subtract,
    I16x8SubtractSaturatingS,
    I16x8SubtractSaturatingU,
    F64x2Nearest,
    I16x8Multiply,
    I16x8MinimumS,
    I16x8MinimumU,
    I16x8MaximumS,
    I16x8MaximumU,
    I16x8AverageU = 155,
    I16x8ExtendMultiplyLowI8x16S,
    I16x8ExtendMultiplyHighI8x16S,
    I16x8ExtendMultiplyLowI8x16U,
    I16x8ExtendMultiplyHighI8x16U,
    I32x4Absolute,
    I32x4Negate,
    I32x4AllTrue = 163,
    I32x4Bitmask,
    I32x4ExtendLowI16x8S = 167,
    I32x4ExtendHighI16x8S,
    I32x4ExtendLowI16x8U,
    I32x4ExtendHighI16x8U,
    I32x4Shl,
    I32x4ShrS,
    I32x4ShrU,
    I32x4Add,
    I32x4Subtract = 177,
    I32x4Multiply = 181,
    I32x4MinimumS,
    I32x4MinimumU,
    I32x4MaximumS,
    I32x4MaximumU,
    I32x4DotI16x8S,
    I32x4ExtendMultiplyLowI16x8S = 188,
    I32x4ExtendMultiplyHighI16x8S,
    I32x4ExtendMultiplyLowI16x8U,
    I32x4ExtendMultiplyHighI16x8U,
    I64x2Absolute,
    I64x2Neg,
    I64xx2AllTrue = 195,
    I64x2Bitmask,
    I64x2ExtendLowI32x4S = 199,
    I64x2ExtendHighI32x4S,
    I64x2ExtendLowI32x4U,
    I64x2ExtendHighI32x4U,
    I64x2Shl,
    I64x2ShrS,
    I64x2ShrU,
    I64x2Add,
    I64x2Sub = 209,
    I64x2Multiply = 213,
    I64x2ExtendMultiplyLowI32x4S = 220,
    I64x2ExtendMultiplyHighI32x4S,
    I64x2ExtendMultiplyLowI32x4U,
    I64x2ExtendMultiplyHighI32x4U,
    F64x2Absolute = 236,
    F64x2Negate,
    F64x2SquareRoot = 239,
    F64x2Add,
    F64x2Subtract,
    F64x2Multiply,
    F64x2Divide,
    F64x2Minimum,
    F64x2Maximum,
    F64x2PMinimum,
    F64x2PMaximum,
    I32x4TruncateSaturatingF32x4S,
    I32x4TruncateSaturatingF32x4U,
    F32x4ConvertI32x4S,
    F32x4ConvertI32x4U,
    I32x4TruncateSaturatingF64x2S,
    I32x4TruncateSaturatingF64x2U,
    F64x2ConvertLowI32x4S,
    F64x2ConvertLowI32x4U,
}
