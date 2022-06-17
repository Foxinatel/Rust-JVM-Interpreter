use std::{cell::RefCell, rc::Rc};

use crate::parser::{
  attribute_info::{attribute::ATTRIBUTE, code::code_generator::Instructions},
  cp_info::CpInfo,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Object {}

type Reference = Option<Rc<RefCell<Object>>>;

#[derive(Debug, Clone)]
pub enum Type {
  Reference(Reference),
  ArrayRefI(Option<Rc<RefCell<[i32]>>>),
  ArrayRefL(Option<Rc<RefCell<[i64]>>>),
  ArrayRefF(Option<Rc<RefCell<[f32]>>>),
  ArrayRefD(Option<Rc<RefCell<[f64]>>>),
  ArrayRefA(Option<Rc<RefCell<[Reference]>>>),
  ArrayRefB(Option<Rc<RefCell<[i8]>>>),
  ArrayRefC(Option<Rc<RefCell<[char]>>>),
  ArrayRefS(Option<Rc<RefCell<[i16]>>>),

  Int(i32),
  Long(i64),
  Float(f32),
  Double(f64),
}

struct Frame {}

pub struct JVM {
  pub constant_pool: Vec<CpInfo>,
}

impl JVM {
  pub fn evaluate(&self, code: ATTRIBUTE) -> Option<Type> {
    match code {
      ATTRIBUTE::Code {
        max_stack,
        max_locals,
        code_length: _,
        code,
        exception_table_length: _,
        exception_table: _,
        attributes_count: _,
        attributes: _,
      } => {
        let mut pc = 0;
        let mut locals: Vec<Type> = Vec::with_capacity(max_locals as usize);
        let mut stack: Vec<Type> = Vec::with_capacity(max_stack as usize);
        loop {
          let inst = &code[pc];
          match inst {
            Instructions::nop => {}
            Instructions::aconst_null => stack.push(Type::Reference(None)),
            Instructions::iconst { value } => stack.push(Type::Int(*value)),
            Instructions::lconst { value } => stack.push(Type::Long(*value)),
            Instructions::fconst { value } => stack.push(Type::Float(*value)),
            Instructions::dconst { value } => stack.push(Type::Double(*value)),
            Instructions::bipush { value } => stack.push(Type::Int(*value as i32)),
            Instructions::sipush { value } => stack.push(Type::Int(*value as i32)),
            Instructions::ldc { index } => {
              let constval = &self.constant_pool[*index as usize];
              // match constval {

              // }
            },
            Instructions::ldc_w { index } => todo!(),
            Instructions::ldc2_w { index } => todo!(),
            Instructions::iload { index } => {
              let _val = &locals[*index as usize];
              match _val {
                Type::Int(val) => stack.push(Type::Int(*val)),
                _ => panic!("Local variable was not of type Int"),
              }
            }
            Instructions::lload { index } => {
              let _val = &locals[*index as usize];
              match _val {
                Type::Long(val) => stack.push(Type::Long(*val)),
                _ => panic!("Local variable was not of type Int"),
              }
            }
            Instructions::fload { index } => {
              let _val = &locals[*index as usize];
              match _val {
                Type::Float(val) => stack.push(Type::Float(*val)),
                _ => panic!("Local variable was not of type Int"),
              }
            }
            Instructions::dload { index } => {
              let _val = &locals[*index as usize];
              match _val {
                Type::Double(val) => stack.push(Type::Double(*val)),
                _ => panic!("Local variable was not of type Int"),
              }
            }
            Instructions::aload { index } => {
              let _val = &locals[*index as usize];
              match _val {
                Type::Reference(val) => stack.push(Type::Reference(val.clone())),
                _ => panic!("Local variable was not of type Int"),
              }
            }
            Instructions::iaload => {
              let _index = stack.pop().unwrap();
              match _index {
                Type::Int(index) => {
                  let _arrayref = stack.pop().unwrap();
                  match _arrayref {
                    Type::ArrayRefI(arrayref) => stack.push(Type::Int(
                      (*arrayref.expect("NullPointerException")).borrow()[index as usize],
                    )),
                    _ => panic!("Value on stack was not of type ArrayRefI"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::laload => {
              let _index = stack.pop().unwrap();
              match _index {
                Type::Int(index) => {
                  let _arrayref = stack.pop().unwrap();
                  match _arrayref {
                    Type::ArrayRefL(arrayref) => stack.push(Type::Long(
                      (*arrayref.expect("NullPointerException")).borrow()[index as usize],
                    )),
                    _ => panic!("Value on stack was not of type ArrayRefL"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::faload => {
              let _index = stack.pop().unwrap();
              match _index {
                Type::Int(index) => {
                  let _arrayref = stack.pop().unwrap();
                  match _arrayref {
                    Type::ArrayRefF(arrayref) => stack.push(Type::Float(
                      (*arrayref.expect("NullPointerException")).borrow()[index as usize],
                    )),
                    _ => panic!("Value on stack was not of type ArrayRefF"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::daload => {
              let _index = stack.pop().unwrap();
              match _index {
                Type::Int(index) => {
                  let _arrayref = stack.pop().unwrap();
                  match _arrayref {
                    Type::ArrayRefD(arrayref) => stack.push(Type::Double(
                      (*arrayref.expect("NullPointerException")).borrow()[index as usize],
                    )),
                    _ => panic!("Value on stack was not of type ArrayRefD"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::aaload => {
              let _index = stack.pop().unwrap();
              match _index {
                Type::Int(index) => {
                  let _arrayref = stack.pop().unwrap();
                  match _arrayref {
                    Type::ArrayRefA(arrayref) => stack.push(Type::Reference(
                      (*arrayref.expect("NullPointerException")).borrow()[index as usize].clone(),
                    )),
                    _ => panic!("Value on stack was not of type ArrayRefA"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::baload => {
              let _index = stack.pop().unwrap();
              match _index {
                Type::Int(index) => {
                  let _arrayref = stack.pop().unwrap();
                  match _arrayref {
                    Type::ArrayRefB(arrayref) => stack.push(Type::Int(
                      (*arrayref.expect("NullPointerException")).borrow()[index as usize] as i32,
                    )),
                    _ => panic!("Value on stack was not of type ArrayRefB"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::caload => {
              let _index = stack.pop().unwrap();
              match _index {
                Type::Int(index) => {
                  let _arrayref = stack.pop().unwrap();
                  match _arrayref {
                    Type::ArrayRefC(arrayref) => stack.push(Type::Int(
                      (*arrayref.expect("NullPointerException")).borrow()[index as usize] as i32,
                    )),
                    _ => panic!("Value on stack was not of type ArrayRefC"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::saload => {
              let _index = stack.pop().unwrap();
              match _index {
                Type::Int(index) => {
                  let _arrayref = stack.pop().unwrap();
                  match _arrayref {
                    Type::ArrayRefS(arrayref) => stack.push(Type::Int(
                      (*arrayref.expect("NullPointerException")).borrow()[index as usize] as i32,
                    )),
                    _ => panic!("Value on stack was not of type ArrayRefS"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::istore { index } => locals[*index as usize] = stack.pop().unwrap(),
            Instructions::lstore { index } => locals[*index as usize] = stack.pop().unwrap(),
            Instructions::fstore { index } => locals[*index as usize] = stack.pop().unwrap(),
            Instructions::dstore { index } => locals[*index as usize] = stack.pop().unwrap(),
            Instructions::astore { index } => locals[*index as usize] = stack.pop().unwrap(),
            Instructions::iastore => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Int(value) => {
                  let _index = stack.pop().unwrap();
                  match _index {
                    Type::Int(index) => {
                      let _arrayref = stack.pop().unwrap();
                      match _arrayref {
                        Type::ArrayRefI(arrayref) => {
                          (*arrayref.expect("NullPointerException")).borrow_mut()[index as usize] =
                            value
                        }
                        _ => panic!("Value on stack was not of type ArrayRefI"),
                      }
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::lastore => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Long(value) => {
                  let _index = stack.pop().unwrap();
                  match _index {
                    Type::Int(index) => {
                      let _arrayref = stack.pop().unwrap();
                      match _arrayref {
                        Type::ArrayRefL(arrayref) => {
                          (*arrayref.expect("NullPointerException")).borrow_mut()[index as usize] =
                            value
                        }
                        _ => panic!("Value on stack was not of type ArrayRefL"),
                      }
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Long"),
              }
            }
            Instructions::fastore => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Float(value) => {
                  let _index = stack.pop().unwrap();
                  match _index {
                    Type::Int(index) => {
                      let _arrayref = stack.pop().unwrap();
                      match _arrayref {
                        Type::ArrayRefF(arrayref) => {
                          (*arrayref.expect("NullPointerException")).borrow_mut()[index as usize] =
                            value
                        }
                        _ => panic!(),
                      }
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Float"),
              }
            }
            Instructions::dastore => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Double(value) => {
                  let _index = stack.pop().unwrap();
                  match _index {
                    Type::Int(index) => {
                      let _arrayref = stack.pop().unwrap();
                      match _arrayref {
                        Type::ArrayRefD(arrayref) => {
                          (*arrayref.expect("NullPointerException")).borrow_mut()[index as usize] =
                            value
                        }
                        _ => panic!("Value on stack was not of type ArrayRefD"),
                      }
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Double"),
              }
            }
            Instructions::aastore => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Reference(value) => {
                  let _index = stack.pop().unwrap();
                  match _index {
                    Type::Int(index) => {
                      let _arrayref = stack.pop().unwrap();
                      match _arrayref {
                        Type::ArrayRefA(arrayref) => {
                          (*arrayref.expect("NullPointerException")).borrow_mut()[index as usize] =
                            value
                        }
                        _ => panic!("Value on stack was not of type ArrayRefA"),
                      }
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Long"),
              }
            }
            Instructions::bastore => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Int(value) => {
                  let _index = stack.pop().unwrap();
                  match _index {
                    Type::Int(index) => {
                      let _arrayref = stack.pop().unwrap();
                      match _arrayref {
                        Type::ArrayRefB(arrayref) => {
                          (*arrayref.expect("NullPointerException")).borrow_mut()[index as usize] =
                            value as i8
                        }
                        _ => panic!("Value on stack was not of type ArrayRefB"),
                      }
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Long"),
              }
            }
            Instructions::castore => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Int(value) => {
                  let _index = stack.pop().unwrap();
                  match _index {
                    Type::Int(index) => {
                      let _arrayref = stack.pop().unwrap();
                      match _arrayref {
                        Type::ArrayRefC(arrayref) => {
                          (*arrayref.expect("NullPointerException")).borrow_mut()[index as usize] =
                            value as u8 as char
                        }
                        _ => panic!("Value on stack was not of type ArrayRefC"),
                      }
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Long"),
              }
            }
            Instructions::sastore => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Int(value) => {
                  let _index = stack.pop().unwrap();
                  match _index {
                    Type::Int(index) => {
                      let _arrayref = stack.pop().unwrap();
                      match _arrayref {
                        Type::ArrayRefS(arrayref) => {
                          (*arrayref.expect("NullPointerException")).borrow_mut()[index as usize] =
                            value as i16
                        }
                        _ => panic!("Value on stack was not of type ArrayRefS"),
                      }
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Long"),
              }
            }
            Instructions::pop => {
              stack.pop();
            }
            Instructions::pop2 => {
              stack.pop();
              stack.pop();
            }
            Instructions::dup => {
              let val = stack.last().unwrap();
              stack.push(val.clone());
            }
            Instructions::dup_x1 => {
              let val = stack.last().unwrap();
              stack.insert(stack.len() - 2, val.clone())
            }
            Instructions::dup_x2 => {
              let value1 = stack.pop().unwrap();
              let value2 = stack.pop().unwrap();
              let value3_opt = stack.pop();
              match value3_opt {
                Some(value3) => {
                  stack.push(value1.clone());
                  stack.push(value3.clone());
                  stack.push(value2.clone());
                  stack.push(value1.clone());
                }
                None => {
                  stack.push(value1.clone());
                  stack.push(value2.clone());
                  stack.push(value1.clone());
                }
              }
            }
            Instructions::dup2 => {
              let value1 = stack.pop().unwrap();
              let value2_opt = stack.pop();
              match value2_opt {
                Some(value2) => {
                  stack.push(value2.clone());
                  stack.push(value1.clone());
                  stack.push(value2.clone());
                  stack.push(value1.clone());
                }
                None => {
                  stack.push(value1.clone());
                  stack.push(value1.clone());
                }
              }
            }
            Instructions::dup2_x1 => todo!(),
            Instructions::dup2_x2 => todo!(),
            Instructions::swap => {
              let len = stack.len();
              stack.swap(len - 1, len - 2)
            }
            Instructions::iadd => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Int(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Int(value1) => {
                      stack.push(Type::Int(value1 + value2));
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::ladd => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Long(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Long(value1) => {
                      stack.push(Type::Long(value1 + value2));
                    }
                    _ => panic!("Value on stack was not of type Long"),
                  }
                }
                _ => panic!("Value on stack was not of type Long"),
              }
            }
            Instructions::fadd => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Float(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Float(value1) => {
                      stack.push(Type::Float(value1 + value2));
                    }
                    _ => panic!("Value on stack was not of type Float"),
                  }
                }
                _ => panic!("Value on stack was not of type Float"),
              }
            }
            Instructions::dadd => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Double(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Double(value1) => {
                      stack.push(Type::Double(value1 + value2));
                    }
                    _ => panic!("Value on stack was not of type Double"),
                  }
                }
                _ => panic!("Value on stack was not of type Double"),
              }
            }
            Instructions::isub => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Int(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Int(value1) => {
                      stack.push(Type::Int(value1 - value2));
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::lsub => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Long(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Long(value1) => {
                      stack.push(Type::Long(value1 - value2));
                    }
                    _ => panic!("Value on stack was not of type Long"),
                  }
                }
                _ => panic!("Value on stack was not of type Long"),
              }
            }
            Instructions::fsub => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Float(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Float(value1) => {
                      stack.push(Type::Float(value1 - value2));
                    }
                    _ => panic!("Value on stack was not of type Float"),
                  }
                }
                _ => panic!("Value on stack was not of type Float"),
              }
            }
            Instructions::dsub => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Double(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Double(value1) => {
                      stack.push(Type::Double(value1 - value2));
                    }
                    _ => panic!("Value on stack was not of type Double"),
                  }
                }
                _ => panic!("Value on stack was not of type Double"),
              }
            }
            Instructions::imul => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Int(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Int(value1) => {
                      stack.push(Type::Int(value1 * value2));
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::lmul => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Long(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Long(value1) => {
                      stack.push(Type::Long(value1 * value2));
                    }
                    _ => panic!("Value on stack was not of type Long"),
                  }
                }
                _ => panic!("Value on stack was not of type Long"),
              }
            }
            Instructions::fmul => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Float(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Float(value1) => {
                      stack.push(Type::Float(value1 * value2));
                    }
                    _ => panic!("Value on stack was not of type Float"),
                  }
                }
                _ => panic!("Value on stack was not of type Float"),
              }
            }
            Instructions::dmul => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Double(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Double(value1) => {
                      stack.push(Type::Double(value1 * value2));
                    }
                    _ => panic!("Value on stack was not of type Double"),
                  }
                }
                _ => panic!("Value on stack was not of type Double"),
              }
            }
            Instructions::idiv => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Int(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Int(value1) => {
                      stack.push(Type::Int(value1 / value2));
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::ldiv => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Long(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Long(value1) => {
                      stack.push(Type::Long(value1 / value2));
                    }
                    _ => panic!("Value on stack was not of type Long"),
                  }
                }
                _ => panic!("Value on stack was not of type Long"),
              }
            }
            Instructions::fdiv => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Float(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Float(value1) => {
                      stack.push(Type::Float(value1 / value2));
                    }
                    _ => panic!("Value on stack was not of type Float"),
                  }
                }
                _ => panic!("Value on stack was not of type Float"),
              }
            }
            Instructions::ddiv => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Double(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Double(value1) => {
                      stack.push(Type::Double(value1 / value2));
                    }
                    _ => panic!("Value on stack was not of type Double"),
                  }
                }
                _ => panic!("Value on stack was not of type Double"),
              }
            }
            Instructions::irem => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Int(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Int(value1) => {
                      stack.push(Type::Int(value1 % value2));
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::lrem => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Long(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Long(value1) => {
                      stack.push(Type::Long(value1 % value2));
                    }
                    _ => panic!("Value on stack was not of type Long"),
                  }
                }
                _ => panic!("Value on stack was not of type Long"),
              }
            }
            Instructions::frem => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Float(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Float(value1) => {
                      stack.push(Type::Float(value1 % value2));
                    }
                    _ => panic!("Value on stack was not of type Float"),
                  }
                }
                _ => panic!("Value on stack was not of type Float"),
              }
            }
            Instructions::drem => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Double(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Double(value1) => {
                      stack.push(Type::Double(value1 % value2));
                    }
                    _ => panic!("Value on stack was not of type Double"),
                  }
                }
                _ => panic!("Value on stack was not of type Double"),
              }
            }
            Instructions::ineg => {
              let _val = stack.pop().unwrap();
              match _val {
                Type::Int(val) => stack.push(Type::Int(-val)),
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::lneg => {
              let _val = stack.pop().unwrap();
              match _val {
                Type::Long(val) => stack.push(Type::Long(-val)),
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::fneg => {
              let _val = stack.pop().unwrap();
              match _val {
                Type::Float(val) => stack.push(Type::Float(-val)),
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::dneg => {
              let _val = stack.pop().unwrap();
              match _val {
                Type::Double(val) => stack.push(Type::Double(-val)),
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::ishl => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Int(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Int(value1) => {
                      stack.push(Type::Int(value1 << (value2 | 0b11111)));
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::lshl => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Int(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Long(value1) => {
                      stack.push(Type::Long(value1 << (value2 | 0b111111)));
                    }
                    _ => panic!("Value on stack was not of type Long"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::ishr => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Int(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Int(value1) => {
                      stack.push(Type::Int(value1 >> (value2 | 0b11111)));
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::lshr => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Int(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Long(value1) => {
                      stack.push(Type::Long(value1 >> (value2 | 0b111111)));
                    }
                    _ => panic!("Value on stack was not of type Long"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::iushr => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Int(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Int(value1) => {
                      stack.push(Type::Int((value1 as u32 >> (value2 | 0b11111)) as i32));
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::lushr => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Int(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Long(value1) => {
                      stack.push(Type::Long((value1 as u64 >> (value2 | 0b111111)) as i64));
                    }
                    _ => panic!("Value on stack was not of type Long"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::iand => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Int(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Int(value1) => stack.push(Type::Int(value1 & value2)),
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::land => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Long(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Long(value1) => stack.push(Type::Long(value1 & value2)),
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::ior => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Int(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Int(value1) => stack.push(Type::Int(value1 | value2)),
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::lor => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Long(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Long(value1) => stack.push(Type::Long(value1 | value2)),
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::ixor => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Int(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Int(value1) => stack.push(Type::Int(value1 ^ value2)),
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::lxor => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Long(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Long(value1) => stack.push(Type::Long(value1 ^ value2)),
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::iinc { index, r#const } => {
              let mut _val = &mut locals[*index as usize];
              match _val {
                Type::Int(ref mut val) => *val += *r#const as i32,
                _ => panic!("Local variable was not of type Int"),
              }
            }
            Instructions::i2l => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Int(value) => stack.push(Type::Long(value as i64)),
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::i2f => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Int(value) => stack.push(Type::Float(value as f32)),
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::i2d => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Int(value) => stack.push(Type::Double(value as f64)),
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::l2i => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Long(value) => stack.push(Type::Int(value as i32)),
                _ => panic!("Value on stack was not of type Long"),
              }
            }
            Instructions::l2f => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Long(value) => stack.push(Type::Float(value as f32)),
                _ => panic!("Value on stack was not of type Long"),
              }
            }
            Instructions::l2d => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Long(value) => stack.push(Type::Double(value as f64)),
                _ => panic!("Value on stack was not of type Long"),
              }
            }
            Instructions::f2i => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Float(value) => stack.push(Type::Int(value as i32)),
                _ => panic!("Value on stack was not of type Float"),
              }
            }
            Instructions::f2l => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Float(value) => stack.push(Type::Long(value as i64)),
                _ => panic!("Value on stack was not of type Float"),
              }
            }
            Instructions::f2d => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Float(value) => stack.push(Type::Double(value as f64)),
                _ => panic!("Value on stack was not of type Float"),
              }
            }
            Instructions::d2i => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Double(value) => stack.push(Type::Int(value as i32)),
                _ => panic!("Value on stack was not of type Double"),
              }
            }
            Instructions::d2l => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Double(value) => stack.push(Type::Long(value as i64)),
                _ => panic!("Value on stack was not of type Double"),
              }
            }
            Instructions::d2f => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Double(value) => stack.push(Type::Float(value as f32)),
                _ => panic!("Value on stack was not of type Double"),
              }
            }
            Instructions::i2b => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Int(value) => stack.push(Type::Int(value as i8 as i32)),
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::i2c => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Int(value) => stack.push(Type::Int(value as u8 as i32)),
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::i2s => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Int(value) => stack.push(Type::Int(value as i16 as i32)),
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::lcmp => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Long(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Long(value1) => {
                      if value1 > value2 {
                        stack.push(Type::Int(1))
                      } else if value1 < value2 {
                        stack.push(Type::Int(-1))
                      } else {
                        stack.push(Type::Int(0))
                      }
                    }
                    _ => panic!("Value on stack was not of type Long"),
                  }
                }
                _ => panic!("Value on stack was not of type Long"),
              }
            }
            Instructions::fcmpl => todo!(),
            Instructions::fcmpg => todo!(),
            Instructions::dcmpl => todo!(),
            Instructions::dcmpg => todo!(),
            Instructions::ifeq { offset } => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Int(value) => {
                  if value == 0 {
                    pc = (pc as isize + *offset as isize - 1) as usize; 
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::ifne { offset } => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Int(value) => {
                  if value != 0 {
                    pc = (pc as isize + *offset as isize - 1) as usize; 
                  }
                }
                _ => panic!("Value on stack was not of type Long"),
              }
            }
            Instructions::iflt { offset } => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Int(value) => {
                  if value < 0 {
                    pc = (pc as isize + *offset as isize - 1) as usize; 
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::ifge { offset } => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Int(value) => {
                  if value >= 0 {
                    pc = (pc as isize + *offset as isize - 1) as usize; 
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::ifgt { offset } => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Int(value) => {
                  if value > 0 {
                    pc = (pc as isize + *offset as isize - 1) as usize; 
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::ifle { offset } => {
              let _value = stack.pop().unwrap();
              match _value {
                Type::Int(value) => {
                  if value <= 0 {
                    pc = (pc as isize + *offset as isize - 1) as usize; 
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::if_icmpeq { offset } => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Int(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Int(value1) => {
                      if value1 == value2 {
                        pc = (pc as isize + *offset as isize - 1) as usize; 
                      }
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::if_icmpne { offset } => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Int(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Int(value1) => {
                      if value1 != value2 {
                        pc = (pc as isize + *offset as isize - 1) as usize; 
                      }
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::if_icmplt { offset } => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Int(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Int(value1) => {
                      if value1 < value2 {
                        pc = (pc as isize + *offset as isize - 1) as usize; 
                      }
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::if_icmpge { offset } => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Int(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Int(value1) => {
                      if value1 >= value2 {
                        pc = (pc as isize + *offset as isize - 1) as usize; 
                      }
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::if_icmpgt { offset } => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Int(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Int(value1) => {
                      if value1 > value2 {
                        pc = (pc as isize + *offset as isize - 1) as usize; 
                      }
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::if_icmple { offset } => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Int(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Int(value1) => {
                      if value1 <= value2 {
                        pc = (pc as isize + *offset as isize - 1) as usize; 
                      }
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::if_acmpeq { offset } => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Reference(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Reference(value1) => {
                      if value1 == value2 {
                        pc = (pc as isize + *offset as isize - 1) as usize; 
                      }
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::if_acmpne { offset } => {
              let _value2 = stack.pop().unwrap();
              match _value2 {
                Type::Reference(value2) => {
                  let _value1 = stack.pop().unwrap();
                  match _value1 {
                    Type::Reference(value1) => {
                      if value1 != value2 {
                        pc = (pc as isize + *offset as isize - 1) as usize; 
                      }
                    }
                    _ => panic!("Value on stack was not of type Int"),
                  }
                }
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::goto { offset } => {
              println!("aaa {}", *offset);
              pc = (pc as isize + *offset as isize - 1) as usize; 
            },
            Instructions::jsr { offset } => todo!(),
            Instructions::ret { index } => todo!(),
            Instructions::tableswitch {
              default,
              low,
              high,
              offsets,
            } => todo!(),
            Instructions::lookupswith {
              default,
              npairs,
              pairs,
            } => todo!(),
            Instructions::ireturn => {
              let value = stack.pop().unwrap();
              match value {
                Type::Int(_) => return Some(value),
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::lreturn => {
              let value = stack.pop().unwrap();
              match value {
                Type::Long(_) => return Some(value),
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::freturn => {
              let value = stack.pop().unwrap();
              match value {
                Type::Float(_) => return Some(value),
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::dreturn => {
              let value = stack.pop().unwrap();
              match value {
                Type::Double(_) => return Some(value),
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::areturn => {
              let value = stack.pop().unwrap();
              match value {
                Type::Reference(_) => return Some(value),
                _ => panic!("Value on stack was not of type Int"),
              }
            }
            Instructions::r#return => return None,
            Instructions::getstatic { index } => todo!(),
            Instructions::putstatic { index } => todo!(),
            Instructions::getfield { index } => todo!(),
            Instructions::putfield { index } => todo!(),
            Instructions::invokevirtual { index } => todo!(),
            Instructions::invokespecial { index } => todo!(),
            Instructions::invokestatic { index } => todo!(),
            Instructions::invokeinterface { index, count } => todo!(),
            Instructions::invokedynamic { index } => todo!(),
            Instructions::new { index } => todo!(),
            Instructions::newarray { atype } => todo!(),
            Instructions::anewarray { index } => todo!(),
            Instructions::arraylength => todo!(),
            Instructions::athrow => todo!(),
            Instructions::checkcast { index } => todo!(),
            Instructions::instanceof { index } => todo!(),
            Instructions::monitorenter => todo!(),
            Instructions::monitorexit => todo!(),
            Instructions::wide1 {
              opcode,
              index_extension,
            } => todo!(),
            Instructions::wide2 {
              opcode,
              index_extension,
              constbytes,
            } => todo!(),
            Instructions::multianewarray { index, dimensions } => todo!(),
            Instructions::ifnull { offset } => todo!(),
            Instructions::ifnonnull { offset } => todo!(),
            Instructions::goto_w { offset } => todo!(),
            Instructions::jsr_w { offset } => todo!(),
          }
          pc += 1;
        }
      }
      _ => panic!("Tried to evaluate a non-code attribute"),
    }
  }
}
