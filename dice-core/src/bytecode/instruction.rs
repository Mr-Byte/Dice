use std::fmt::{Debug, Display};

macro_rules! define_instructions {
    (prev=$prev:ident @) => {};
    (prev=$prev:ident @ $vis:vis $next:ident $($sub_vis:vis $name:ident)*) => {
        $vis const $next: Self = Self(Self::$prev.0 + 1);
        define_instructions! {
            prev=$next @
            $($sub_vis $name)*
        }
    };

    ($vis:vis const $first:ident; $($sub_vis:vis const $name:ident;)*) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        #[repr(transparent)]
        pub struct Instruction(u8);

        impl Instruction {
            $vis const $first: Self = Self(0);
            define_instructions! {
                prev=$first @
                $($sub_vis $name)*
            }

            pub const fn value(self) -> u8 {
                self.0
            }
        }

        impl From<u8> for Instruction {
            fn from(value: u8) -> Self {
                Instruction(value)
            }
        }

        impl Display for Instruction {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:02X} | ", self.0)?;

                match *self {
                    Self::$first => write!(f, stringify!($first)),
                    $(Self::$name => write!(f, stringify!($name)),)*
                    _ => unreachable!()
                }
            }
        }
    };
}

define_instructions! {
    // Instructions to push data onto the stack.
    pub const PUSH_NULL;
    pub const PUSH_UNIT;
    pub const PUSH_FALSE;
    pub const PUSH_TRUE;
    pub const PUSH_I0;
    pub const PUSH_I1;
    pub const PUSH_F0;
    pub const PUSH_F1;
    pub const PUSH_CONST;
    // Basic stack manipulation instructions.
    pub const POP;
    pub const DUP;
    pub const SWAP;
    // Compound stack manipulation instructions.
    pub const CREATE_ARRAY;
    pub const CREATE_OBJECT;
    pub const CREATE_CLOSURE;
    // NOTE: Create class takes a constant reference to its name and a 64-bit operand representing its instance type id.
    pub const CREATE_CLASS;
    // NOTE: There's no concept of "storing" a module.
    pub const LOAD_MODULE;
    pub const LOAD_GLOBAL;
    // NOTE: Globals are write-once, if they don't already exist.
    pub const STORE_GLOBAL;
    pub const LOAD_LOCAL;
    pub const STORE_LOCAL;
    pub const ASSIGN_LOCAL;
    pub const LOAD_FIELD;
    pub const STORE_FIELD;
    pub const ASSIGN_FIELD;
    pub const LOAD_INDEX;
    pub const STORE_INDEX;
    pub const ASSIGN_INDEX;
    pub const LOAD_UPVALUE;
    pub const STORE_UPVALUE;
    pub const ASSIGN_UPVALUE;
    pub const CLOSE_UPVALUE;

    // NOTE: Fused operation to load a field into a local variable.
    // TODO: Figure out if a ASSIGN_FIELD_TO_LOCAL variant could be used to optimize assigning from a field.
    pub const LOAD_FIELD_TO_LOCAL;
    // NOTE: Stores the method in a class object's method map.
    // Static functions and the like are stored as fields on the class object itself.
    pub const STORE_METHOD;
    // Operator instructions
    pub const NEG;
    pub const NOT;
    pub const MUL;
    pub const DIV;
    pub const REM;
    pub const ADD;
    pub const SUB;
    pub const GT;
    pub const GTE;
    pub const LT;
    pub const LTE;
    pub const EQ;
    pub const NEQ;
    pub const IS;
    pub const LOGICAL_AND;
    pub const LOGICAL_OR;
    pub const DIE_ROLL;
    pub const DICE_ROLL;
    pub const RANGE_INCLUSIVE;
    pub const RANGE_EXCLUSIVE;
    // Control flow instructions
    pub const JUMP;
    pub const JUMP_IF_FALSE;
    pub const JUMP_IF_TRUE;
    pub const CALL;
    pub const RETURN;
    // Type assertion instructions
    pub const ASSERT_BOOL;
    pub const ASSERT_TYPE_FOR_LOCAL;
    pub const ASSERT_TYPE_OR_NULL_FOR_LOCAL;
    pub const ASSERT_TYPE_AND_RETURN;
    pub const ASSERT_TYPE_OR_NULL_AND_RETURN;
}
