use crate::value::Value;
pub use cursor::BytecodeCursor;
use dice_error::span::Span;
use instruction::Instruction;
use std::{collections::HashMap, fmt::Display, rc::Rc};

mod cursor;
pub mod instruction;

#[derive(Debug)]
struct BytecodeInner {
    slot_count: usize,
    upvalue_count: usize,
    constants: Box<[Value]>,
    data: Box<[u8]>,
    source_map: HashMap<u64, Span>,
}

#[derive(Debug, Clone)]
pub struct Bytecode {
    inner: Rc<BytecodeInner>,
}

impl Bytecode {
    pub fn new(
        data: Box<[u8]>,
        slot_count: usize,
        upvalue_count: usize,
        constants: Box<[Value]>,
        source_map: HashMap<u64, Span>,
    ) -> Self {
        Self {
            inner: Rc::new(BytecodeInner {
                constants,
                slot_count,
                upvalue_count,
                source_map,
                data,
            }),
        }
    }

    #[allow(dead_code)]
    pub fn source_map(&self) -> &HashMap<u64, Span> {
        &self.inner.source_map
    }

    pub fn constants(&self) -> &[Value] {
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
                        Value::FnScript(fn_script) => {
                            write!(f, "const={:<8} |", const_index)?;

                            for _ in 0..fn_script.bytecode().upvalue_count() {
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
            if let Value::FnScript(fn_script) = const_value {
                writeln!(f, "Function: {:?}", fn_script.name())?;
                writeln!(f, "--------")?;
                fn_script.bytecode().fmt(f)?;
            }
        }

        Ok(())
    }
}
