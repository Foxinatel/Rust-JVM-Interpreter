use std::{collections::HashMap, env, path::Path};

use self::{static_class::ClassStatics, eval::Eval};
use crate::parser::classfile::ClassFile;

mod dynamic_class;
mod eval;
mod helpers;
mod resolver;
mod static_class;
mod types;

#[derive(Debug)]
pub struct JVM {
  entrypoint: String,
  // classes: HashMap<String, ClassFile>
  classes: HashMap<String, ClassStatics>
}

//set current directory to the target's directory
//resolve other necessary classfiles
impl JVM {
  pub fn from_path(path: String) -> Self {
    env::set_current_dir(Path::new(&path).parent().unwrap()).unwrap();
    let newpath = String::from(Path::new(&path).file_name().unwrap().to_str().unwrap());
    let (name, cf, depends) = ClassFile::read(newpath);
    let mut resolver = resolver::Resolver::new();
    resolver.resolved.insert(name.clone(), ClassStatics::from(cf));
    resolver.resolve(depends);
    Self { entrypoint: name, classes: resolver.resolved }
  }

  pub fn start(&self) {
    self.classes.get(&self.entrypoint).unwrap().evaluate(&self, String::from("main:([Ljava/lang/String;)V"));
  }
}
