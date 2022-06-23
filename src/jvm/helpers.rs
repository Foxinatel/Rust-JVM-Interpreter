use std::rc::Rc;

use super::types::HeapType;

pub fn refeq(value1: Option<HeapType>, value2: Option<HeapType>) -> bool {
  match (value1, value2) {
    (None, None) => true,
    (None, Some(_)) => false,
    (Some(_), None) => false,
    (Some(value1), Some(value2)) => {
      match (value1, value2) {
        (HeapType::Class(ref1), HeapType::Class(ref2)) => Rc::ptr_eq(&ref1, &ref2),
        //(HeapType::Interface(ref1), HeapType::Interface(ref2)) => Rc::ptr_eq(&ref1, &ref2),
        (HeapType::ArrayI(ref1), HeapType::ArrayI(ref2)) => Rc::ptr_eq(&ref1, &ref2),
        (HeapType::ArrayL(ref1), HeapType::ArrayL(ref2)) => Rc::ptr_eq(&ref1, &ref2),
        (HeapType::ArrayF(ref1), HeapType::ArrayF(ref2)) => Rc::ptr_eq(&ref1, &ref2),
        (HeapType::ArrayD(ref1), HeapType::ArrayD(ref2)) => Rc::ptr_eq(&ref1, &ref2),
        (HeapType::ArrayB(ref1), HeapType::ArrayB(ref2)) => Rc::ptr_eq(&ref1, &ref2),
        (HeapType::ArrayC(ref1), HeapType::ArrayC(ref2)) => Rc::ptr_eq(&ref1, &ref2),
        (HeapType::ArrayS(ref1), HeapType::ArrayS(ref2)) => Rc::ptr_eq(&ref1, &ref2),
        (HeapType::ArrayA(ref1), HeapType::ArrayA(ref2)) => Rc::ptr_eq(&ref1, &ref2),
        _ => false
      }
    }
  }
}
