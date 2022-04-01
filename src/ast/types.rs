use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Type {
    pub qualifier: Vec<TypeQualifier>, // TODO(TO/GA): move it into basic type
    pub function_specifier: Vec<FunctionSpecifier>,
    pub storage_class_specifier: StorageClassSpecifier,
    pub basic_type: BasicType,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum StorageClassSpecifier {
    Typedef,
    Extern,
    Static,
    ThreadLocal,
    Auto,
    Register,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum TypeQualifier {
    Const,
    Volatile,
    Restrict,
    Atomic,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum FunctionSpecifier {
    Inline,
    Noreturn,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum BasicType {
    Void,
    Char,
    Int,
    Bool,
    Float,
    Double,
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

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct StructMember {
    pub member_name: String,
    pub member_type: Type,
}

impl Default for Type {
    fn default() -> Self {
        Type {
            qualifier: Default::default(),
            function_specifier: Default::default(),
            storage_class_specifier: StorageClassSpecifier::Auto,
            basic_type: Default::default(),
        }
    }
}

impl Default for BasicType {
    fn default() -> Self {
        BasicType::Int
    }
}
