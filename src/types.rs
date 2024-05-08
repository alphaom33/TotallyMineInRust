use std::vec::Vec;
use std::collections::HashMap;

struct VarExpr {
  var_name: String,
  has_star: bool
}

enum VarAssign {
  Single(VarExpr),
  Group(Vec<VarExpr>)
}


enum Num {
  Boolean(bool),
  Integer(i128),
  Float(f64)
}

struct Function {
  params: VarExpr,
  body: String
}


enum BaseValue {
  Null,
  Integer(Num),
  String(String),
  Array(Vec<usize>),
  Function(Function),
  NativeFunction(Box<dyn Fn(BaseValue) -> BaseType>)
}

struct BaseType {
  value: BaseValue,
  attrs: HashMap<usize, usize>
}
