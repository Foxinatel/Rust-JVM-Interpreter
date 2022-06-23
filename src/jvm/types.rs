use std::{cell::RefCell, rc::Rc};

use super::dynamic_class::ClassDynamics;

#[derive(Debug, Clone)]
pub enum HeapType {
  Class(Rc<RefCell<ClassDynamics>>),
  Interface(),

  ArrayI(Rc<RefCell<Vec<i32>>>),
  ArrayL(Rc<RefCell<Vec<i64>>>),
  ArrayF(Rc<RefCell<Vec<f32>>>),
  ArrayD(Rc<RefCell<Vec<f64>>>),
  ArrayB(Rc<RefCell<Vec<i8>>>),
  ArrayC(Rc<RefCell<Vec<u16>>>),
  ArrayS(Rc<RefCell<Vec<i16>>>),
  ArrayA(Rc<RefCell<Vec<Option<HeapType>>>>)
}

#[derive(Debug, Clone)]
pub enum Type {
  //Primitives:
  Int(i32),
  Long(i64),
  Float(f32),
  Double(f64),
  ReturnAddress(usize),
  Reference(Option<HeapType>)
}
