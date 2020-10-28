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
                Instruction::JUMP | Instruction::JUMP_IF_FALSE | Instruction::JUMP_IF_TRUE => {
                    write!(f, "offset={}", cursor.read_offset())?
                }
                Instruction::PUSH_CONST
                | Instruction::DUP
                | Instruction::LOAD_MODULE
                | Instruction::LOAD_GLOBAL
                | Instruction::LOAD_LOCAL
                | Instruction::LOAD_FIELD
                | Instruction::LOAD_UPVALUE
                | Instruction::STORE_GLOBAL
                | Instruction::STORE_LOCAL
                | Instruction::STORE_FIELD
                | Instruction::STORE_METHOD
                | Instruction::STORE_UPVALUE
                | Instruction::ASSIGN_LOCAL
                | Instruction::ASSIGN_FIELD
                | Instruction::ASSIGN_UPVALUE
                | Instruction::CLOSE_UPVALUE
                | Instruction::CALL
                | Instruction::CREATE_ARRAY
                | Instruction::CREATE_CLASS => write!(f, "const={}", cursor.read_u8())?,
                Instruction::LOAD_FIELD_TO_LOCAL => write!(f, "const={} slot={}", cursor.read_u8(), cursor.read_u8())?,
                Instruction::CREATE_CLOSURE => {
                    let const_index = cursor.read_u8() as usize;
                    let function = &self.constants()[const_index];

                    match function {
                        Value::FnScript(fn_script) => {
                            write!(f, "{:<8} |", const_index)?;

                            for _ in 0..fn_script.bytecode.upvalue_count() {
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
                writeln!(f, "Function: {:?}", fn_script.name)?;
                writeln!(f, "--------")?;
                fn_script.bytecode.fmt(f)?;
            }
        }

        Ok(())
    }
}
