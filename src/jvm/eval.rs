use std::collections::HashMap;

use super::{
  helpers::refeq,
  types::{HeapType, Type},
  JVM
};
use crate::parser::{attribute_info::{code::code_generator::Instructions, Attribute}, method_info::MethodInfo};

// This macro chopped out so much bloat, thank god
macro_rules! get_type {
  ($variant:ident, $val:expr) => {{
    let Type::$variant(value) = $val else {
                      panic!("Found value {:?} which is not of type {}", $val, stringify!($variant))
                    };
    value
  }};
}

macro_rules! assert_type {
  ($variant:ident, $val:expr) => {
    let Type::$variant(_) = $val else {
                      panic!("Found value {:?} which is not of type {}", $val, stringify!($variant))
                    };
  };
}

pub trait Eval {
  fn get_methods(&self) -> &HashMap<String, MethodInfo>;

  fn evaluate(&self, jvm: &JVM, method: String) -> Option<Type> {
    let attr = &self.get_methods().get(&method).unwrap().attributes[0];
    let Attribute::Code { max_stack, max_locals, code, exception_table: _, attributes: _ } = attr else {panic!()};
    let mut stack = Vec::with_capacity(*max_stack as usize);
    let mut locals = Vec::with_capacity(*max_locals as usize);
    let mut pc = 0;
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
        Instructions::ldc { index: _ } => todo!(),
        Instructions::ldc_w { index: _ } => todo!(),
        Instructions::ldc2_w { index: _ } => todo!(),
        Instructions::iload { index } => {
          let val = get_type!(Int, &locals[*index as usize]);
          stack.push(Type::Int(*val))
        }
        Instructions::lload { index } => {
          let val = get_type!(Long, &locals[*index as usize]);
          stack.push(Type::Long(*val))
        }
        Instructions::fload { index } => {
          let val = get_type!(Float, &locals[*index as usize]);
          stack.push(Type::Float(*val))
        }
        Instructions::dload { index } => {
          let val = get_type!(Double, &locals[*index as usize]);
          stack.push(Type::Double(*val))
        }
        Instructions::aload { index } => {
          let val = get_type!(Reference, &locals[*index as usize]);
          stack.push(Type::Reference(val.clone()))
        }
        Instructions::iaload => {
          let index = get_type!(Int, stack.pop().unwrap());
          let reference = get_type!(Reference, stack.pop().unwrap()).expect("NullPointerException");
          let HeapType::ArrayI(arrayref) = &reference else {panic!()};
          let array = arrayref.borrow();
          stack.push(Type::Int(array[index as usize]))
        }
        Instructions::laload => {
          let index = get_type!(Int, stack.pop().unwrap());
          let reference = get_type!(Reference, stack.pop().unwrap()).expect("NullPointerException");
          let HeapType::ArrayL(arrayref) = &reference else {panic!()};
          let array = arrayref.borrow();
          stack.push(Type::Long(array[index as usize]))
        }
        Instructions::faload => {
          let index = get_type!(Int, stack.pop().unwrap());
          let reference = get_type!(Reference, stack.pop().unwrap()).expect("NullPointerException");
          let HeapType::ArrayF(arrayref) = &reference else {panic!()};
          let array = arrayref.borrow();
          stack.push(Type::Float(array[index as usize]))
        }
        Instructions::daload => {
          let index = get_type!(Int, stack.pop().unwrap());
          let reference = get_type!(Reference, stack.pop().unwrap()).expect("NullPointerException");
          let HeapType::ArrayD(arrayref) = &reference else {panic!()};
          let array = arrayref.borrow();
          stack.push(Type::Double(array[index as usize]))
        }
        Instructions::aaload => {
          let index = get_type!(Int, stack.pop().unwrap());
          let reference = get_type!(Reference, stack.pop().unwrap()).expect("NullPointerException");
          let HeapType::ArrayA(arrayref) = &reference else {panic!()};
          let array = arrayref.borrow();
          stack.push(Type::Reference(array[index as usize].clone()))
        }
        Instructions::baload => {
          let index = get_type!(Int, stack.pop().unwrap());
          let reference = get_type!(Reference, stack.pop().unwrap()).expect("NullPointerException");
          let HeapType::ArrayB(arrayref) = &reference else {panic!()};
          let array = arrayref.borrow();
          stack.push(Type::Int(array[index as usize] as i32))
        }
        Instructions::caload => {
          let index = get_type!(Int, stack.pop().unwrap());
          let reference = get_type!(Reference, stack.pop().unwrap()).expect("NullPointerException");
          let HeapType::ArrayC(arrayref) = &reference else {panic!()};
          let array = arrayref.borrow();
          stack.push(Type::Int(array[index as usize] as i32))
        }
        Instructions::saload => {
          let index = get_type!(Int, stack.pop().unwrap());
          let reference = get_type!(Reference, stack.pop().unwrap()).expect("NullPointerException");
          let HeapType::ArrayS(arrayref) = &reference else {panic!()};
          let array = arrayref.borrow();
          stack.push(Type::Int(array[index as usize] as i32))
        }
        Instructions::istore { index } => {
          let value = stack.pop().unwrap();
          assert_type!(Int, value);
          locals[*index as usize] = value
        }
        Instructions::lstore { index } => {
          let value = stack.pop().unwrap();
          assert_type!(Long, value);
          locals[*index as usize] = value
        }
        Instructions::fstore { index } => {
          let value = stack.pop().unwrap();
          assert_type!(Float, value);
          locals[*index as usize] = value
        }
        Instructions::dstore { index } => {
          let value = stack.pop().unwrap();
          assert_type!(Double, value);
          locals[*index as usize] = value
        }
        Instructions::astore { index } => {
          let value = stack.pop().unwrap();
          assert_type!(Reference, value);
          locals[*index as usize] = value
        }
        Instructions::iastore => {
          let value = get_type!(Int, stack.pop().unwrap());
          let index = get_type!(Int, stack.pop().unwrap());
          let reference = get_type!(Reference, stack.pop().unwrap()).expect("NullPointerException");
          let HeapType::ArrayI(arrayref) = &reference else {panic!()};
          let mut array = arrayref.borrow_mut();
          array[index as usize] = value;
        }
        Instructions::lastore => {
          let value = get_type!(Long, stack.pop().unwrap());
          let index = get_type!(Int, stack.pop().unwrap());
          let reference = get_type!(Reference, stack.pop().unwrap()).expect("NullPointerException");
          let HeapType::ArrayL(arrayref) = &reference else {panic!()};
          let mut array = arrayref.borrow_mut();
          array[index as usize] = value;
        }
        Instructions::fastore => {
          let value = get_type!(Float, stack.pop().unwrap());
          let index = get_type!(Int, stack.pop().unwrap());
          let reference = get_type!(Reference, stack.pop().unwrap()).expect("NullPointerException");
          let HeapType::ArrayF(arrayref) = &reference else {panic!()};
          let mut array = arrayref.borrow_mut();
          array[index as usize] = value;
        }
        Instructions::dastore => {
          let value = get_type!(Double, stack.pop().unwrap());
          let index = get_type!(Int, stack.pop().unwrap());
          let reference = get_type!(Reference, stack.pop().unwrap()).expect("NullPointerException");
          let HeapType::ArrayD(arrayref) = &reference else {panic!()};
          let mut array = arrayref.borrow_mut();
          array[index as usize] = value;
        }
        Instructions::aastore => {
          let value = get_type!(Reference, stack.pop().unwrap());
          let index = get_type!(Int, stack.pop().unwrap());
          let reference = get_type!(Reference, stack.pop().unwrap()).expect("NullPointerException");
          let HeapType::ArrayA(arrayref) = &reference else {panic!()};
          let mut array = arrayref.borrow_mut();
          array[index as usize] = value;
        }
        Instructions::bastore => {
          let value = get_type!(Int, stack.pop().unwrap());
          let index = get_type!(Int, stack.pop().unwrap());
          let reference = get_type!(Reference, stack.pop().unwrap()).expect("NullPointerException");
          let HeapType::ArrayB(arrayref) = &reference else {panic!()};
          let mut array = arrayref.borrow_mut();
          array[index as usize] = value as i8;
        }
        Instructions::castore => {
          let value = get_type!(Int, stack.pop().unwrap());
          let index = get_type!(Int, stack.pop().unwrap());
          let reference = get_type!(Reference, stack.pop().unwrap()).expect("NullPointerException");
          let HeapType::ArrayC(arrayref) = &reference else {panic!()};
          let mut array = arrayref.borrow_mut();
          array[index as usize] = value as u16;
        }
        Instructions::sastore => {
          let value = get_type!(Int, stack.pop().unwrap());
          let index = get_type!(Int, stack.pop().unwrap());
          let reference = get_type!(Reference, stack.pop().unwrap()).expect("NullPointerException");
          let HeapType::ArrayS(arrayref) = &reference else {panic!()};
          let mut array = arrayref.borrow_mut();
          array[index as usize] = value as i16;
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
              stack.push(value3);
              stack.push(value2);
              stack.push(value1);
            }
            None => {
              stack.push(value1.clone());
              stack.push(value2);
              stack.push(value1);
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
              stack.push(value2);
              stack.push(value1);
            }
            None => {
              stack.push(value1.clone());
              stack.push(value1);
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
          let value2 = get_type!(Int, stack.pop().unwrap());
          let value1 = get_type!(Int, stack.last_mut().unwrap());
          *value1 += value2;
        }
        Instructions::ladd => {
          let value2 = get_type!(Long, stack.pop().unwrap());
          let value1 = get_type!(Long, stack.last_mut().unwrap());
          *value1 += value2;
        }
        Instructions::fadd => {
          let value2 = get_type!(Float, stack.pop().unwrap());
          let value1 = get_type!(Float, stack.last_mut().unwrap());
          *value1 += value2;
        }
        Instructions::dadd => {
          let value2 = get_type!(Double, stack.pop().unwrap());
          let value1 = get_type!(Double, stack.last_mut().unwrap());
          *value1 += value2;
        }
        Instructions::isub => {
          let value2 = get_type!(Int, stack.pop().unwrap());
          let value1 = get_type!(Int, stack.last_mut().unwrap());
          *value1 -= value2;
        }
        Instructions::lsub => {
          let value2 = get_type!(Long, stack.pop().unwrap());
          let value1 = get_type!(Long, stack.last_mut().unwrap());
          *value1 -= value2;
        }
        Instructions::fsub => {
          let value2 = get_type!(Float, stack.pop().unwrap());
          let value1 = get_type!(Float, stack.last_mut().unwrap());
          *value1 -= value2;
        }
        Instructions::dsub => {
          let value2 = get_type!(Double, stack.pop().unwrap());
          let value1 = get_type!(Double, stack.last_mut().unwrap());
          *value1 -= value2;
        }
        Instructions::imul => {
          let value2 = get_type!(Int, stack.pop().unwrap());
          let value1 = get_type!(Int, stack.last_mut().unwrap());
          *value1 *= value2;
        }
        Instructions::lmul => {
          let value2 = get_type!(Long, stack.pop().unwrap());
          let value1 = get_type!(Long, stack.last_mut().unwrap());
          *value1 *= value2;
        }
        Instructions::fmul => {
          let value2 = get_type!(Float, stack.pop().unwrap());
          let value1 = get_type!(Float, stack.last_mut().unwrap());
          *value1 *= value2;
        }
        Instructions::dmul => {
          let value2 = get_type!(Double, stack.pop().unwrap());
          let value1 = get_type!(Double, stack.last_mut().unwrap());
          *value1 *= value2;
        }
        Instructions::idiv => {
          let value2 = get_type!(Int, stack.pop().unwrap());
          let value1 = get_type!(Int, stack.last_mut().unwrap());
          *value1 /= value2;
        }
        Instructions::ldiv => {
          let value2 = get_type!(Long, stack.pop().unwrap());
          let value1 = get_type!(Long, stack.last_mut().unwrap());
          *value1 /= value2;
        }
        Instructions::fdiv => {
          let value2 = get_type!(Float, stack.pop().unwrap());
          let value1 = get_type!(Float, stack.last_mut().unwrap());
          *value1 /= value2;
        }
        Instructions::ddiv => {
          let value2 = get_type!(Double, stack.pop().unwrap());
          let value1 = get_type!(Double, stack.last_mut().unwrap());
          *value1 /= value2;
        }
        Instructions::irem => {
          let value2 = get_type!(Int, stack.pop().unwrap());
          let value1 = get_type!(Int, stack.last_mut().unwrap());
          *value1 %= value2;
        }
        Instructions::lrem => {
          let value2 = get_type!(Long, stack.pop().unwrap());
          let value1 = get_type!(Long, stack.last_mut().unwrap());
          *value1 %= value2;
        }
        Instructions::frem => {
          let value2 = get_type!(Float, stack.pop().unwrap());
          let value1 = get_type!(Float, stack.last_mut().unwrap());
          *value1 %= value2;
        }
        Instructions::drem => {
          let value2 = get_type!(Double, stack.pop().unwrap());
          let value1 = get_type!(Double, stack.last_mut().unwrap());
          *value1 %= value2;
        }
        Instructions::ineg => {
          let value = get_type!(Int, stack.last_mut().unwrap());
          *value = -*value;
        }
        Instructions::lneg => {
          let value = get_type!(Long, stack.last_mut().unwrap());
          *value = -*value;
        }
        Instructions::fneg => {
          let value = get_type!(Float, stack.last_mut().unwrap());
          *value = -*value;
        }
        Instructions::dneg => {
          let value = get_type!(Double, stack.last_mut().unwrap());
          *value = -*value;
        }
        Instructions::ishl => {
          let shift = get_type!(Int, stack.pop().unwrap());
          let value = get_type!(Int, stack.last_mut().unwrap());
          *value <<= shift | 0b11111
        }
        Instructions::lshl => {
          let shift = get_type!(Int, stack.pop().unwrap());
          let value = get_type!(Long, stack.last_mut().unwrap());
          *value <<= shift | 0b111111
        }
        Instructions::ishr => {
          let shift = get_type!(Int, stack.pop().unwrap());
          let value = get_type!(Int, stack.last_mut().unwrap());
          *value >>= shift | 0b11111
        }
        Instructions::lshr => {
          let shift = get_type!(Int, stack.pop().unwrap());
          let value = get_type!(Long, stack.last_mut().unwrap());
          *value >>= shift | 0b111111
        }
        Instructions::iushr => {
          let shift = get_type!(Int, stack.pop().unwrap());
          let value = get_type!(Int, stack.last_mut().unwrap());
          *value = (*value as u32 >> (shift | 0b11111)) as i32;
        }
        Instructions::lushr => {
          let shift = get_type!(Int, stack.pop().unwrap());
          let value = get_type!(Long, stack.last_mut().unwrap());
          *value = (*value as u64 >> (shift | 0b111111)) as i64;
        }
        Instructions::iand => {
          let value2 = get_type!(Int, stack.pop().unwrap());
          let value1 = get_type!(Int, stack.last_mut().unwrap());
          *value1 &= value2;
        }
        Instructions::land => {
          let value2 = get_type!(Long, stack.pop().unwrap());
          let value1 = get_type!(Long, stack.last_mut().unwrap());
          *value1 &= value2;
        }
        Instructions::ior => {
          let value2 = get_type!(Int, stack.pop().unwrap());
          let value1 = get_type!(Int, stack.last_mut().unwrap());
          *value1 |= value2;
        }
        Instructions::lor => {
          let value2 = get_type!(Long, stack.pop().unwrap());
          let value1 = get_type!(Long, stack.last_mut().unwrap());
          *value1 |= value2;
        }
        Instructions::ixor => {
          let value2 = get_type!(Int, stack.pop().unwrap());
          let value1 = get_type!(Int, stack.last_mut().unwrap());
          *value1 ^= value2;
        }
        Instructions::lxor => {
          let value2 = get_type!(Long, stack.pop().unwrap());
          let value1 = get_type!(Long, stack.last_mut().unwrap());
          *value1 ^= value2;
        }
        Instructions::iinc { index, r#const } => {
          let value = get_type!(Int, &mut locals[*index as usize]);
          *value += *r#const as i32
        }
        Instructions::i2l => {
          let value = get_type!(Int, stack.pop().unwrap());
          stack.push(Type::Long(value as i64))
        }
        Instructions::i2f => {
          let value = get_type!(Int, stack.pop().unwrap());
          stack.push(Type::Float(value as f32))
        }
        Instructions::i2d => {
          let value = get_type!(Int, stack.pop().unwrap());
          stack.push(Type::Double(value as f64))
        }
        Instructions::l2i => {
          let value = get_type!(Long, stack.pop().unwrap());
          stack.push(Type::Int(value as i32))
        }
        Instructions::l2f => {
          let value = get_type!(Long, stack.pop().unwrap());
          stack.push(Type::Float(value as f32))
        }
        Instructions::l2d => {
          let value = get_type!(Long, stack.pop().unwrap());
          stack.push(Type::Double(value as f64))
        }
        Instructions::f2i => {
          let value = get_type!(Float, stack.pop().unwrap());
          stack.push(Type::Int(value as i32))
        }
        Instructions::f2l => {
          let value = get_type!(Float, stack.pop().unwrap());
          stack.push(Type::Long(value as i64))
        }
        Instructions::f2d => {
          let value = get_type!(Float, stack.pop().unwrap());
          stack.push(Type::Double(value as f64))
        }
        Instructions::d2i => {
          let value = get_type!(Double, stack.pop().unwrap());
          stack.push(Type::Int(value as i32))
        }
        Instructions::d2l => {
          let value = get_type!(Double, stack.pop().unwrap());
          stack.push(Type::Long(value as i64))
        }
        Instructions::d2f => {
          let value = get_type!(Double, stack.pop().unwrap());
          stack.push(Type::Float(value as f32))
        }
        Instructions::i2b => {
          let value = get_type!(Int, stack.pop().unwrap());
          stack.push(Type::Int(value as i8 as i32))
        }
        Instructions::i2c => {
          let value = get_type!(Int, stack.pop().unwrap());
          stack.push(Type::Int(value as u8 as i32))
        }
        Instructions::i2s => {
          let value = get_type!(Int, stack.pop().unwrap());
          stack.push(Type::Int(value as i16 as i32))
        }
        Instructions::lcmp => {
          let value2 = get_type!(Long, stack.pop().unwrap());
          let value1 = get_type!(Long, stack.pop().unwrap());
          if value1 > value2 {
            stack.push(Type::Int(1))
          } else if value1 < value2 {
            stack.push(Type::Int(-1))
          } else {
            stack.push(Type::Int(0))
          }
        }
        Instructions::fcmpl => todo!(),
        Instructions::fcmpg => todo!(),
        Instructions::dcmpl => todo!(),
        Instructions::dcmpg => todo!(),
        Instructions::ifeq { offset } => {
          if get_type!(Int, stack.pop().unwrap()) == 0 {
            pc = (pc as isize + *offset as isize - 1) as usize;
          }
        }
        Instructions::ifne { offset } => {
          if get_type!(Int, stack.pop().unwrap()) != 0 {
            pc = (pc as isize + *offset as isize - 1) as usize;
          }
        }
        Instructions::iflt { offset } => {
          if get_type!(Int, stack.pop().unwrap()) < 0 {
            pc = (pc as isize + *offset as isize - 1) as usize;
          }
        }
        Instructions::ifge { offset } => {
          if get_type!(Int, stack.pop().unwrap()) >= 0 {
            pc = (pc as isize + *offset as isize - 1) as usize;
          }
        }
        Instructions::ifgt { offset } => {
          if get_type!(Int, stack.pop().unwrap()) > 0 {
            pc = (pc as isize + *offset as isize - 1) as usize;
          }
        }
        Instructions::ifle { offset } => {
          if get_type!(Int, stack.pop().unwrap()) <= 0 {
            pc = (pc as isize + *offset as isize - 1) as usize;
          }
        }
        Instructions::if_icmpeq { offset } => {
          let value2 = get_type!(Int, stack.pop().unwrap());
          let value1 = get_type!(Int, stack.pop().unwrap());
          if value1 == value2 {
            pc = (pc as isize + *offset as isize - 1) as usize;
          }
        }
        Instructions::if_icmpne { offset } => {
          let value2 = get_type!(Int, stack.pop().unwrap());
          let value1 = get_type!(Int, stack.pop().unwrap());
          if value1 != value2 {
            pc = (pc as isize + *offset as isize - 1) as usize;
          }
        }
        Instructions::if_icmplt { offset } => {
          let value2 = get_type!(Int, stack.pop().unwrap());
          let value1 = get_type!(Int, stack.pop().unwrap());
          if value1 < value2 {
            pc = (pc as isize + *offset as isize - 1) as usize;
          }
        }
        Instructions::if_icmpge { offset } => {
          let value2 = get_type!(Int, stack.pop().unwrap());
          let value1 = get_type!(Int, stack.pop().unwrap());
          if value1 >= value2 {
            pc = (pc as isize + *offset as isize - 1) as usize;
          }
        }
        Instructions::if_icmpgt { offset } => {
          let value2 = get_type!(Int, stack.pop().unwrap());
          let value1 = get_type!(Int, stack.pop().unwrap());
          if value1 > value2 {
            pc = (pc as isize + *offset as isize - 1) as usize;
          }
        }
        Instructions::if_icmple { offset } => {
          let value2 = get_type!(Int, stack.pop().unwrap());
          let value1 = get_type!(Int, stack.pop().unwrap());
          if value1 <= value2 {
            pc = (pc as isize + *offset as isize - 1) as usize;
          }
        }
        Instructions::if_acmpeq { offset } => {
          let value2 = get_type!(Reference, stack.pop().unwrap());
          let value1 = get_type!(Reference, stack.pop().unwrap());
          if refeq(value1, value2) {
            pc = (pc as isize + *offset as isize - 1) as usize;
          }
        }
        Instructions::if_acmpne { offset } => {
          let value2 = get_type!(Reference, stack.pop().unwrap());
          let value1 = get_type!(Reference, stack.pop().unwrap());
          if !refeq(value1, value2) {
            pc = (pc as isize + *offset as isize - 1) as usize;
          }
        }
        // IMPORTANT REMINDER: "offset" is the number of instructions to offset by.
        // This differs from the raw classfile data, which uses an offset in bytes.
        Instructions::goto { offset } => {
          pc = (pc as isize + *offset as isize - 1) as usize;
        }
        Instructions::jsr { offset: _ } => todo!(),
        Instructions::ret { index: _ } => todo!(),
        Instructions::tableswitch { default: _, low: _, high: _, offsets: _ } => todo!(),
        Instructions::lookupswith { default: _, npairs: _, pairs: _ } => todo!(),
        Instructions::ireturn => {
          let val = stack.pop().unwrap();
          assert_type!(Int, val);
          return Some(val);
        }
        Instructions::lreturn => {
          let val = stack.pop().unwrap();
          assert_type!(Long, val);
          return Some(val);
        }
        Instructions::freturn => {
          let val = stack.pop().unwrap();
          assert_type!(Float, val);
          return Some(val);
        }
        Instructions::dreturn => {
          let val = stack.pop().unwrap();
          assert_type!(Double, val);
          return Some(val);
        }
        Instructions::areturn => {
          let val = stack.pop().unwrap();
          assert_type!(Reference, val);
          return Some(val);
        }
        Instructions::r#return => return None,
        Instructions::getstatic { fieldref } => {
          let class = jvm.classes.get(&fieldref.class.to_string()).unwrap();
          let field = class.fields.get(&fieldref.name_and_type.to_string()).unwrap();
          todo!();
          // stack.push(field.value);
        }
        Instructions::putstatic { fieldref } => {
          let value = stack.pop().unwrap();
          let class = jvm.classes.get(&fieldref.class.to_string()).unwrap();
          let field = class.fields.get(&fieldref.name_and_type.to_string()).unwrap();
          todo!();
          // field.value = value;
        }
        Instructions::getfield { fieldref } => {
          let objectref = get_type!(Reference, stack.pop().unwrap()).expect("NullPointerException");
          let HeapType::Class(objectref) = objectref else {panic!()};
          let object = objectref.borrow();
          let field = object.fields.get(&fieldref.name_and_type.to_string()).unwrap();
          todo!();
          // stack.push(field.value);
        }
        Instructions::putfield { fieldref } => {
          let value = stack.pop().unwrap();
          let objectref = get_type!(Reference, stack.pop().unwrap()).expect("NullPointerException");
          let HeapType::Class(objectref) = objectref else {panic!()};
          let mut object = objectref.borrow_mut();
          let mut field = object.fields.get(&fieldref.name_and_type.to_string()).unwrap();
          todo!();
          // field.value = value;
        }
        Instructions::invokevirtual { methodref: _ } => todo!(),
        Instructions::invokespecial { methodref: _ } => todo!(),
        Instructions::invokestatic { methodref: _ } => todo!(),
        Instructions::invokeinterface { interfacemethodref: _, count: _ } => todo!(),
        Instructions::invokedynamic { invokedynamic: _ } => todo!(),
        Instructions::new { class } => {
          let classobj = jvm.classes.get(&class.name).unwrap();
          let new = classobj.instantiate();
          stack.push(Type::Reference(Some(HeapType::Class(new))));
        }
        Instructions::newarray { atype: _ } => {}
        Instructions::anewarray { index: _ } => todo!(),
        Instructions::arraylength => todo!(),
        Instructions::athrow => todo!(),
        Instructions::checkcast { index: _ } => todo!(),
        Instructions::instanceof { index: _ } => todo!(),
        Instructions::monitorenter => todo!(),
        Instructions::monitorexit => todo!(),
        Instructions::wide1 { opcode: _, index_extension: _ } => todo!(),
        Instructions::wide2 { opcode: _, index_extension: _, constbytes: _ } => todo!(),
        Instructions::multianewarray { index: _, dimensions: _ } => todo!(),
        Instructions::ifnull { offset: _ } => todo!(),
        Instructions::ifnonnull { offset: _ } => todo!(),
        Instructions::goto_w { offset: _ } => todo!(),
        Instructions::jsr_w { offset: _ } => todo!()
      }
      pc += 1;
    }
  }
}
