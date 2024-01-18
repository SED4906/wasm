use num_enum::{IntoPrimitive, TryFromPrimitive, TryFromPrimitiveError};

#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ValueType {
    Empty = 0x40,
    ExternRef = 0x6F,
    FuncRef,
    V128 = 0x7B,
    F64,
    F32,
    I64,
    I32,
}

pub enum BlockType {
    Value(ValueType),
    Index(u32),
}

impl TryFrom<i64> for BlockType {
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 0 {
            Ok(BlockType::Value((value as u8 & 0x7F).try_into()?))
        } else {
            Ok(BlockType::Index(value.try_into().unwrap()))
        }
    }

    type Error = TryFromPrimitiveError<ValueType>;
}

pub type TypeIndex = u32;
pub type FuncIndex = u32;
pub type TableIndex = u32;
pub type MemoryIndex = u32;
pub type GlobalIndex = u32;
pub type ElemIndex = u32;
pub type DataIndex = u32;
pub type LocalIndex = u32;
pub type LabelIndex = u32;
pub type LaneIndex = u8;

#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum RefType {
    ExternRef = 0x6F,
    FuncRef,
}