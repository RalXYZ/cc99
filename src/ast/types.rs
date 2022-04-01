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
    Array(
        /// element type
        Box<Type>,
        /// array length
        u64,
    ),
    Function(
        /// return type
        Box<Type>,
        /// parameters' types
        Vec<Type>,
    ),
    Struct(
        /// struct name
        Option<String>,
        /// struct members
        Option<Vec<StructMember>>,
    ),
    /// a name introduced by typedef/struct...
    Identifier(String),
}

#[derive(Serialize, Debug)]
pub struct StructMember {
    pub member_name: String,
    pub member_type: Type,
}
