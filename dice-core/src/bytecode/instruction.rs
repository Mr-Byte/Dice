use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, Debug, ToPrimitive, FromPrimitive)]
pub enum Instruction {
    PushNull,
    PushUnit,
    PushFalse,
    PushTrue,
    PushI0,
    PushI1,
    PushF0,
    PushF1,
    PushConst,
    // Basic stack manipulation instructions.
    Pop,
    Dup,
    Swap,
    // Compound stack manipulation instructions.
    CreateArray,
    CreateObject,
    CreateClosure,
    // NOTE: Create class takes a constant reference to its name and a 64-bit operand representing its instance type id.
    CreateClass,
    InheritClass,
    // NOTE: There's no concept of "storing" a module.
    LoadModule,
    LoadGlobal,
    // NOTE: Globals are write-once, if they don't already exist.
    StoreGlobal,
    LoadLocal,
    StoreLocal,
    AssignLocal,
    LoadField,
    StoreField,
    AssignField,
    LoadIndex,
    StoreIndex,
    AssignIndex,
    LoadUpvalue,
    StoreUpvalue,
    AssignUpvalue,
    CloseUpvalue,

    // NOTE: Fused operation to load a field into a local variable.
    LoadFieldToLocal,
    // NOTE: Stores the method in a class object's method map.
    // Static functions and the like are stored as fields on the class object itself.
    StoreMethod,
    // Operator instructions
    Negate,
    Not,
    Multiply,
    Divide,
    Remainder,
    Add,
    Subtract,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,
    Is,
    DieRoll,
    DiceRoll,
    RangeInclusive,
    RangeExclusive,
    // Control flow instructions
    Jump,
    JumpIfFalse,
    JumpIfTrue,
    Call,
    Return,
    // Type assertion instructions
    AssertBool,
    AssertTypeForLocal,
    AssertTypeOrNullForLocal,
    AssertTypeAndReturn,
    AssertTypeOrNullAndReturn,
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        Self::from_u8(value).expect("Invalid instruction encountered in conversion.")
    }
}

impl Into<u8> for Instruction {
    fn into(self) -> u8 {
        self.to_u8().expect("Invalid instruction encountered in conversion.")
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value: u8 = (*self).into();

        write!(f, "{:02X} | ", value)?;

        let name = match self {
            Instruction::PushNull => "PUSH_NULL",
            Instruction::PushUnit => "PUSH_UNIT",
            Instruction::PushFalse => "PUSH_FALSE",
            Instruction::PushTrue => "PUSH_TRUE",
            Instruction::PushI0 => "PUSH_I0",
            Instruction::PushI1 => "PUSH_I1",
            Instruction::PushF0 => "PUSH_F0",
            Instruction::PushF1 => "PUSH_F1",
            Instruction::PushConst => "PUSH_CONST",
            Instruction::Pop => "POP",
            Instruction::Dup => "DUP",
            Instruction::Swap => "SWAP",
            Instruction::CreateArray => "CREATE_ARRAY",
            Instruction::CreateObject => "CREATE_OBJECT",
            Instruction::CreateClosure => "CREATE_CLOSURE",
            Instruction::CreateClass => "CREATE_CLASS",
            Instruction::InheritClass => "INHERIT_CLASS",
            Instruction::LoadModule => "LOAD_MODULE",
            Instruction::LoadGlobal => "LOAD_GLOBAL",
            Instruction::StoreGlobal => "STORE_GLOBAL",
            Instruction::LoadLocal => "LOAD_LOCAL",
            Instruction::StoreLocal => "STORE_LOCAL",
            Instruction::AssignLocal => "ASSIGN_LOCAL",
            Instruction::LoadField => "LOAD_FIELD",
            Instruction::StoreField => "STORE_FIELD",
            Instruction::AssignField => "ASSIGN_FIELD",
            Instruction::LoadIndex => "LOAD_INDEX",
            Instruction::StoreIndex => "STORE_INDEX",
            Instruction::AssignIndex => "ASSIGN_INDEX",
            Instruction::LoadUpvalue => "LOAD_UPVALUE",
            Instruction::StoreUpvalue => "STORE_UPVALUE",
            Instruction::AssignUpvalue => "ASSIGN_UPVALUE",
            Instruction::CloseUpvalue => "CLOSE_UPVALUE",
            Instruction::LoadFieldToLocal => "LOAD_FIELD_TO_LOCAL",
            Instruction::StoreMethod => "STORE_METHOD",
            Instruction::Negate => "NEG",
            Instruction::Not => "NOT",
            Instruction::Multiply => "MUL",
            Instruction::Divide => "DIV",
            Instruction::Remainder => "REM",
            Instruction::Add => "ADD",
            Instruction::Subtract => "SUB",
            Instruction::GreaterThan => "GT",
            Instruction::GreaterThanOrEqual => "GTE",
            Instruction::LessThan => "LT",
            Instruction::LessThanOrEqual => "LTE",
            Instruction::Equal => "EQ",
            Instruction::NotEqual => "NEQ",
            Instruction::Is => "IS",
            Instruction::DieRoll => "DIE_ROLL",
            Instruction::DiceRoll => "DICE_ROLL",
            Instruction::RangeInclusive => "RANGE_INCLUSIVE",
            Instruction::RangeExclusive => "RANGE_EXCLUSIVE",
            Instruction::Jump => "JUMP",
            Instruction::JumpIfFalse => "JUMP_IF_FALSE",
            Instruction::JumpIfTrue => "JUMP_IF_TRUE",
            Instruction::Call => "CALL",
            Instruction::Return => "RETURN",
            Instruction::AssertBool => "ASSERT_BOOL",
            Instruction::AssertTypeForLocal => "ASSERT_TYPE_FOR_LOCAL",
            Instruction::AssertTypeOrNullForLocal => "ASSERT_TYPE_OR_NULL_FOR_LOCAL",
            Instruction::AssertTypeAndReturn => "ASSERT_TYPE_AND_RETURN",
            Instruction::AssertTypeOrNullAndReturn => "ASSERT_TYPE_OR_NULL_AND_RETURN",
        };

        write!(f, "{}", name)
    }
}
