use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::dynamic_class::ClassDynamics;
use crate::parser::{
  attribute_info::{attribute, Attribute},
  classfile::{ClassAccessFlags, ClassFile},
  cp_info_resolved::Class,
  field_info::{self, FieldInfo},
  method_info::{self, MethodInfo}
};

#[derive(Debug, Clone)]
pub struct ClassStatics {
  pub access_flags: ClassAccessFlags,
  pub super_class: Class,
  pub interfaces: Vec<u16>,
  pub fields: HashMap<String, FieldInfo>,
  pub methods: HashMap<String, MethodInfo>,
  pub attributes: Vec<Attribute>,
  pub dynamic: ClassDynamics
}

impl ClassStatics {
  pub fn from(cf: ClassFile) -> Self {
    let access_flags = cf.access_flags;
    let super_class = cf.super_class;
    let interfaces = cf.interfaces;

    let (mut static_fields, mut dynamic_fields) = (HashMap::new(), HashMap::new());
    for (name, field) in cf.fields {
      if field.access_flags.contains(field_info::Flags::AccStatic) {
        static_fields.insert(name, field);
      } else {
        dynamic_fields.insert(name, field);
      }
    }

    let (mut static_methods, mut dynamic_methods) = (HashMap::new(), HashMap::new());
    for (name, method) in cf.methods {
      if method.access_flags.contains(method_info::Flags::AccStatic) {
        static_methods.insert(name, method);
      } else {
        dynamic_methods.insert(name, method);
      }
    }

    let attributes = cf.attributes;

    ClassStatics {
      access_flags,
      super_class,
      interfaces,
      fields: static_fields,
      methods: static_methods,
      attributes,
      dynamic: ClassDynamics { fields: dynamic_fields, methods: dynamic_methods }
    }
  }

  pub fn instantiate(&self) -> Rc<RefCell<ClassDynamics>> {
    Rc::from(RefCell::from(self.dynamic.clone()))
  }
}
