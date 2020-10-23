pub mod operator {
    pub const DICE_ROLL: &str = "#dice_roll";
    pub const DIE_ROLL: &str = "#die_roll";
    pub const MUL: &str = "#mul";
    pub const DIV: &str = "#div";
    pub const REM: &str = "#rem";
    pub const ADD: &str = "#add";
    pub const SUB: &str = "#sub";
    pub const GT: &str = "#gt";
    pub const GTE: &str = "#gte";
    pub const LT: &str = "#lt";
    pub const LTE: &str = "#lte";
    pub const RANGE_INCLUSIVE: &str = "#range_inclusive";
    pub const RANGE_EXCLUSIVE: &str = "#range_exclusive";
    pub const OPERATORS: &[&str] = &[
        DICE_ROLL,
        DIE_ROLL,
        MUL,
        DIV,
        REM,
        ADD,
        SUB,
        GT,
        GTE,
        LT,
        LTE,
        RANGE_EXCLUSIVE,
        RANGE_INCLUSIVE,
    ];
}

pub mod module {
    pub const EXPORT: &str = "#export";
}

pub mod class {
    pub const SELF: &str = "self";
    pub const NEW: &str = "new";
}

pub mod iterator {
    pub const NEXT: &str = "next";
    pub const VALUE: &str = "value";
    pub const DONE: &str = "is_done";
    pub const ITER: &str = "iter";
}

pub mod object {
    pub const TO_STRING: &str = "to_string";
}
