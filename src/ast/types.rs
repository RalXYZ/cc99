use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Type {
    pub qualifier: Option<TypeQualifier>,
    pub specifier: StorageClassSpecifier,
    pub basic_type: BasicType,
}

#[derive(Serialize, Debug)]
pub enum StorageClassSpecifier {
    Typedef,
    Extern,
    Static,
    ThreadLocal,
    Auto,
    Register,
}

#[derive(Serialize, Debug)]
pub enum TypeQualifier {
    Const,
    Volatile,
    Restrict,
    Atomic,
}

#[derive(Serialize, Debug)]
pub enum FunctionSpecifiers {
    Inline,
    Noreturn,
}

#[derive(Serialize, Debug)]
pub enum BasicType {
    Void,
    Char,
    Int,
    Bool,
    Float,
    Pointer(Box<Type>),
    Function(
        /// return type
        Box<Type>,
        /// parameters' types
        Vec<Type>,
    ),
}
