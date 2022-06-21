use std::collections::BTreeMap;

use super::code_generator::Instructions;

pub fn clean(map: BTreeMap<usize, (usize, Instructions)>) -> Vec<Instructions> {
  for (bytepos, (instpos, instruction)) in map.clone().iter_mut() {
    match instruction {
      Instructions::ifeq { offset }
      | Instructions::ifne { offset }
      | Instructions::iflt { offset }
      | Instructions::ifge { offset }
      | Instructions::ifgt { offset }
      | Instructions::ifle { offset }
      | Instructions::if_icmpeq { offset }
      | Instructions::if_icmpne { offset }
      | Instructions::if_icmplt { offset }
      | Instructions::if_icmpge { offset }
      | Instructions::if_icmpgt { offset }
      | Instructions::if_icmple { offset }
      | Instructions::if_acmpeq { offset }
      | Instructions::if_acmpne { offset }
      | Instructions::goto { offset }
      | Instructions::jsr { offset }
      | Instructions::ifnull { offset }
      | Instructions::ifnonnull { offset } => {
        let (instpos2, _) = map.get(&((*bytepos as isize + *offset as isize) as usize)).unwrap();
        *offset = (*instpos2 as isize - *instpos as isize) as i16;
      }
      Instructions::goto_w { offset } | Instructions::jsr_w { offset } => {
        let (instpos2, _) = map.get(&((*bytepos as isize + *offset as isize) as usize)).unwrap();
        *offset = (*instpos2 as isize - *instpos as isize) as i32;
      }
      _ => {}
    }
  }
  map.into_values().map(|x| x.1).collect()
}
