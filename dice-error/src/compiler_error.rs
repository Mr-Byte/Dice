#[derive(thiserror::Error, Debug)]
pub enum CompilerError {
    #[error("The new method on classes is not allowed to have a return type.")]
    NewHasReturnType(),
    #[error("The new method on classes must have a self receiver as the first parameter.")]
    NewMustHaveSelfReceiver(),
    #[error("The self parameter of methods cannot have a type annotation.")]
    SelfParameterHasType(),
    #[error("Function cannot have duplicate argument names.")]
    DuplicateArgumentNames(),
    #[error("Operator methods must have a self parameter.")]
    OperatorMethodHasNoSelf(),
    #[error("Only methods and constructors can access super.")]
    InvalidSuperAccess(),
    #[error("Constructors of derived classes must call their super constructor first.")]
    DerivedMustCallSuper(),
    #[error("The item {0} has already been declared in this scope.")]
    ItemAlreadyDeclared(String),
    #[error("Encountered undeclared variable {0}.")]
    UndeclaredVariable(String),
    #[error("Cannot assign to immutable variable {0}.")]
    ImmutableVariable(String),
    #[error("Variable {0} has not been initialized.")]
    UninitializedVariable(String),
    #[error("Invalid assignment target.")]
    InvalidAssignmentTarget,
    #[error("Invalid operator name {0}.")]
    InvalidOperatorName(String),
    #[error("The break keyword can only be used inside loops.")]
    InvalidBreak,
    #[error("The continue keyword can only be used inside loops.")]
    InvalidContinue,
    #[error("Loops cannot end with an expression. Try adding ; to the last statement.")]
    InvalidLoopEnding,
    #[error("The return keyword can only be used inside functions.")]
    InvalidReturn,
    #[error("The return keyword cannot have an expression when used in constructors.")]
    InvalidConstructorReturn,
    #[error("The error propagate operator !! can only be used inside functions.")]
    InvalidErrorPropagate,
    #[error("Compilation unit has too many constants.")]
    TooManyConstants,
    #[error("Compilation unit has too many upvalues.")]
    TooManyUpvalues,
    #[error("Internal Compiler Error: {0}")]
    InternalCompilerError(String),
}
