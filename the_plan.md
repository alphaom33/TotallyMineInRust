J = Jesse will work on
M = Matthew will work on

Some things that need to be done:
  Variable memory and scope system (J+M)
  Evaluator/parser (J)
  Object attributes and type system (J+M)
  Syntax errors (and errors in general) (J+M)
  Preprocessing (M)

Plans for preprocessing:
  - [x] Remove comments
  - [x] Change indented blocks to a set of braces
  - [x] Change `foo(){}` to `foo=()->{}`
  - [ ] Change `foo()=bar` to `foo=()->bar`
  - [ ] Change binary/octal/hex numbers to decimal (possibly)
  - [ ] Change `9+10` to `21`
  - [ ] Evaluate number/string literals?
  - [ ] Change `foo+=bar` to `foo = foo + (bar)`