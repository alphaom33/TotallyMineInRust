use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Pointer, Write};
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
struct VarExpr {
    var_name: String,
    has_star: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum VarAssign {
    Single(VarExpr),
    Group(Vec<VarExpr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Num {
    Boolean(bool),
    Integer(i128),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq)]
struct Function {
    params: VarExpr,
    body: String,
}

pub enum BaseValue {
    Null,
    Number(Num),
    String(String),
    Array(Vec<usize>),
    Function(Function),
    NativeFunction(Box<dyn Fn(BaseValue) -> BaseType>),
}

impl BaseValue {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl Debug for BaseValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "Null"),
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::Array(arg0) => f.debug_tuple("Array").field(arg0).finish(),
            Self::Function(arg0) => f.debug_tuple("Function").field(arg0).finish(),
            Self::NativeFunction(_arg0) => f.debug_tuple("NativeFunction").finish(),
        }
    }
}

impl Clone for BaseValue {
    fn clone(&self) -> Self {
        match self {
            Self::Null => Self::Null,
            Self::Number(arg0) => Self::Number(arg0.clone()),
            Self::String(arg0) => Self::String(arg0.clone()),
            Self::Array(arg0) => Self::Array(arg0.clone()),
            Self::Function(arg0) => Self::Function(arg0.clone()),
            Self::NativeFunction(_arg0) => Self::Null,
        }
    }
}

impl PartialEq for BaseValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Array(l0), Self::Array(r0)) => l0 == r0,
            (Self::Function(l0), Self::Function(r0)) => l0 == r0,
            (Self::NativeFunction(_l0), Self::NativeFunction(_r0)) => false,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

unsafe impl Sync for BaseValue {}
unsafe impl Send for BaseValue {}

#[derive(Debug, Clone)]
struct BaseType {
    value: BaseValue,
    attrs: HashMap<usize, usize>,
}
