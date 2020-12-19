pub type ErrorCode = &'static str;

// Syntax errors
pub static UNEXPECTED_TOKEN: ErrorCode = "E1000";
pub static INVALID_ESCAPE_SEQUENCE: ErrorCode = "E1001";
pub static UNTERMINATED_STRING: ErrorCode = "E1002";

// Compiler errors
pub static INTERNAL_COMPILER_ERROR: ErrorCode = "E2000";

pub static TOO_MANY_UPVALUES: ErrorCode = "E2100";
pub static TOO_MANY_CONSTANTS: ErrorCode = "E2101";

pub static NEW_METHOD_CANNOT_HAVE_RETURN_TYPE: ErrorCode = "E2200";
pub static NEW_METHOD_MUST_HAVE_RECEIVER: ErrorCode = "E2201";
pub static NEW_MUST_CALL_SUPER_FROM_SUBCLASS: ErrorCode = "E2202";
pub static NEW_RETURN_CANNOT_HAVE_EXPRESSION: ErrorCode = "E2203";
pub static INVALID_SUPER_CALL: ErrorCode = "E2204";
pub static METHOD_RECEIVER_CANNOT_HAVE_TYPE: ErrorCode = "E2205";
pub static FUNCTION_CANNOT_HAVE_DUPLICATE_ARGS: ErrorCode = "E2206";

pub static CLASS_ALREADY_DECLARED: ErrorCode = "E2300";
pub static FUNCTION_ALREADY_DECLARE: ErrorCode = "E2301";

pub static INVALID_ASSIGNMENT_TARGET: ErrorCode = "E2400";
pub static VARIABLE_NOT_DECLARED: ErrorCode = "E2401";
pub static VARIABLE_NOT_INITIALIZED: ErrorCode = "E2402";
pub static CANNOT_REASSIGN_IMMUTABLE_VARIABLE: ErrorCode = "E2403";

pub static INVALID_RETURN_USAGE: ErrorCode = "E2500";
pub static INVALID_BREAK_USAGE: ErrorCode = "E2501";
pub static INVALID_CONTINUE_USAGE: ErrorCode = "E2502";
pub static INVALID_ERROR_PROPAGATE_USAGE: ErrorCode = "E2503";

// Runtime errors
pub static INVALID_BOOL_CONVERSION: ErrorCode = "E3000";
pub static INVALID_INT_CONVERSION: ErrorCode = "E3001";
pub static INVALID_FLOAT_CONVERSION: ErrorCode = "E3002";
pub static INVALID_ARRAY_CONVERSION: ErrorCode = "E3003";
pub static INVALID_STRING_CONVERSION: ErrorCode = "E3004";
pub static INVALID_SYMBOL_CONVERSION: ErrorCode = "E3005";
pub static INVALID_OBJECT_CONVERSION: ErrorCode = "E3006";
pub static INVALID_CLASS_CONVERSION: ErrorCode = "E3007";
