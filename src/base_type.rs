struct num{}
struct function{}

struct base_type {
  /*
  number value
  string value
  function value
  bool value
  object value?
  yes attributes and items
  wait
  should we store attributes and items separately
  or put them together like 
  */
  str_val: &str,
  num_val: num,
  bool_val: bool,
  func_val: function,
  obj_val: hashmap
}