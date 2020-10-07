use crate::bytecode::instruction::Instruction;
use crate::id::type_id::TypeId;
use bytes::Buf as _;
use std::io::Cursor;

pub struct BytecodeCursor<'a> {
    cursor: Cursor<&'a [u8]>,
}

impl<'a> BytecodeCursor<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(data),
        }
    }

    #[inline]
    pub fn set_position(&mut self, position: u64) {
        self.cursor.set_position(position)
    }

    #[inline]
    pub fn read_instruction(&mut self) -> Option<Instruction> {
        if self.cursor.has_remaining() {
            Some(self.cursor.get_u8().into())
        } else {
            None
        }
    }

    #[inline]
    pub fn read_u8(&mut self) -> u8 {
        self.cursor.get_u8()
    }

    #[inline]
    pub fn read_type_id(&mut self) -> TypeId {
        self.cursor.get_u64().into()
    }

    #[inline]
    pub fn read_offset(&mut self) -> i16 {
        self.cursor.get_i16()
    }

    #[inline]
    pub fn offset_position(&mut self, offset: i16) {
        self.set_position(self.cursor.position().wrapping_add(offset as u64));
    }

    #[inline]
    pub fn position(&self) -> u64 {
        self.cursor.position()
    }

    pub fn remaining(&self) -> u64 {
        self.cursor.remaining() as u64
    }
}
