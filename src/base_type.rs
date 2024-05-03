struct Num {}
struct Function {}

enum Type {
  NULL, NUM, BOOLEAN, STRING, FUNCTION, OBJECT
}

struct BaseType {
  /*
  number value
  string value
  function value
  bool value
  object value?
  should we store attributes and items separately
  or put them together like javascript
  */
  str_val: &str,
  num_val: num,
  bool_val: bool,
  func_val: Function,
  
  obj_val: hashmap,
  val_type: Type
}

pub fn create_null() -> BaseType {
  return BaseType {
    val_type: Type.NULL
  };
}

pub fn create_num(val: Num) -> BaseType {
  return BaseType {
    val_type: Type.NUM,
    num_val: val
  };
}

pub fn create_bool(val: bool) -> BaseType {
  return BaseType {
    val_type: Type.NUM,
    num_val: val
  };
}

pub fn create_string(val: &str) -> BaseType {
  return BaseType {
    val_type: Type.STRING,
    string_val: val
  };
}

pub fn create_function(val: Function) -> BaseType {
  return BaseType {
    val_type: Type.FUNCTION,
    function_val: val
  };
}

pub fn create_object(val: hashmap) -> BaseType {
  return BaseType {
    val_type: Type.OBJECT,
    object_val: val
  }
}