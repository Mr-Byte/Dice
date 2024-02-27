use crate::{source::Source, span::Span};
pub use cursor::BytecodeCursor;
use instruction::Instruction;
use std::{collections::HashMap, fmt::Display, rc::Rc};

mod cursor;
pub mod instruction;
pub mod source;
pub mod span;

#[derive(Debug)]
struct BytecodeInner {
    slot_count: usize,
    upvalue_count: usize,
    constants: Box<[ConstantValue]>,
    data: Box<[u8]>,
    source: Source,
    source_map: HashMap<u64, Span>,
}

#[derive(Debug, Clone)]
pub struct Bytecode {
    inner: Rc<BytecodeInner>,
}

// TODO: Change Symbol and Function to store constant data that isn't dependent on runtime values.
#[derive(Debug, Clone, PartialEq)]
pub enum ConstantValue {
    Int(i64),
    Float(f64),
    String(String),
    Symbol(String),
    Function(FunctionBytecode),
}

#[derive(Debug, Clone)]
pub struct FunctionBytecode {
    pub bytecode: Bytecode,
    pub name: String,
    // TODO: Figure out a better way to allow for functions with the same name?
    // Is this even needed??
    pub id: uuid::Uuid,
}

impl FunctionBytecode {
    pub fn new(bytecode: Bytecode, name: String, id: uuid::Uuid) -> Self {
        Self { bytecode, name, id }
    }
}

impl PartialEq for FunctionBytecode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Bytecode {
    pub fn new(
        data: Box<[u8]>,
        slot_count: usize,
        upvalue_count: usize,
        constants: Box<[ConstantValue]>,
        source: Source,
        source_map: HashMap<u64, Span>,
    ) -> Self {
        Self {
            inner: Rc::new(BytecodeInner {
                constants,
                slot_count,
                upvalue_count,
                source,
                source_map,
                data,
            }),
        }
    }

    pub fn source(&self) -> &Source {
        &self.inner.source
    }

    #[allow(dead_code)]
    pub fn source_map(&self) -> &HashMap<u64, Span> {
        &self.inner.source_map
    }

    pub fn constants(&self) -> &[ConstantValue] {
        &self.inner.constants
    }

    pub fn cursor(&self) -> BytecodeCursor<'_> {
        BytecodeCursor::new(&*self.inner.data)
    }

    pub fn slot_count(&self) -> usize {
        self.inner.slot_count
    }

    pub fn upvalue_count(&self) -> usize {
        self.inner.upvalue_count
    }
}

impl Display for Bytecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Code")?;
        writeln!(f, "--------")?;

        let mut cursor = self.cursor();
        let mut position = 0;

        while let Some(instruction) = cursor.read_instruction() {
            write!(f, "{:6} | {:<24} | ", position, format!("{}", instruction))?;

            match instruction {
                Instruction::Jump | Instruction::JumpIfFalse | Instruction::JumpIfTrue => {
                    write!(f, "offset={}", cursor.read_offset())?
                }
                Instruction::PushConst
                | Instruction::Dup
                | Instruction::LoadModule
                | Instruction::LoadGlobal
                | Instruction::LoadLocal
                | Instruction::LoadField
                | Instruction::LoadMethod
                | Instruction::LoadUpvalue
                | Instruction::StoreGlobal
                | Instruction::StoreLocal
                | Instruction::StoreField
                | Instruction::StoreMethod
                | Instruction::StoreUpvalue
                | Instruction::AssignLocal
                | Instruction::AssignField
                | Instruction::AssignUpvalue
                | Instruction::CloseUpvalue
                | Instruction::Call
                | Instruction::CallSuper
                | Instruction::CreateArray
                | Instruction::InheritClass
                | Instruction::AssertTypeForLocal
                | Instruction::AssertTypeOrNullForLocal => write!(f, "const={}", cursor.read_u8())?,
                Instruction::LoadFieldToLocal => write!(f, "const={} slot={}", cursor.read_u8(), cursor.read_u8())?,
                Instruction::CreateClosure => {
                    let const_index = cursor.read_u8() as usize;
                    let function = &self.constants()[const_index];

                    match function {
                        ConstantValue::Function(function) => {
                            write!(f, "const={:<8} |", const_index)?;

                            for _ in 0..function.bytecode.upvalue_count() {
                                let kind = match cursor.read_u8() {
                                    1 => "parent_local",
                                    _ => "upvalue",
                                };
                                let index = cursor.read_u8() as usize;

                                write!(f, " ({}={})", kind, index)?;
                            }
                        }
                        _ => write!(f, "NOT A FUNCTION!")?,
                    }
                }
                _ => (),
            }

            position = cursor.position();

            writeln!(f)?;
        }

        writeln!(f)?;

        for const_value in self.constants() {
            if let ConstantValue::Function(function) = const_value {
                writeln!(f, "Function: {:?}", function.name)?;
                writeln!(f, "--------")?;
                function.bytecode.fmt(f)?;
            }
        }

        Ok(())
    }
}
