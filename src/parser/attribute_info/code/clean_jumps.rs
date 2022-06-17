use super::code_generator::Instructions;

pub fn clean(mut initial: Vec<(usize, Instructions)>) -> Vec<Instructions> {
  let newvec = initial.to_vec();
  for (index1, (pos1, instruction)) in initial.iter_mut().enumerate() {
    match instruction {
      Instructions::ifeq { ref mut offset }
      | Instructions::ifne { ref mut offset }
      | Instructions::iflt { ref mut offset }
      | Instructions::ifge { ref mut offset }
      | Instructions::ifgt { ref mut offset }
      | Instructions::ifle { ref mut offset }
      | Instructions::if_icmpeq { ref mut offset }
      | Instructions::if_icmpne { ref mut offset }
      | Instructions::if_icmplt { ref mut offset }
      | Instructions::if_icmpge { ref mut offset }
      | Instructions::if_icmpgt { ref mut offset }
      | Instructions::if_icmple { ref mut offset }
      | Instructions::if_acmpeq { ref mut offset }
      | Instructions::if_acmpne { ref mut offset }
      | Instructions::goto { ref mut offset }
      | Instructions::jsr { ref mut offset }
      | Instructions::ifnull { ref mut offset }
      | Instructions::ifnonnull { ref mut offset } => {
        let (index2, _) = newvec.iter().enumerate().find(
          |(_, (pos2, _))| {*pos2 == (*pos1 as isize + *offset as isize) as usize}
        ).expect("Jump instruction gave an invalid ref mut offset");
        *offset = (index2 as isize - index1 as isize) as i16;
      }
      Instructions::goto_w { ref mut offset }
      | Instructions::jsr_w { ref mut offset } => {
        let (index2, _) = newvec.iter().enumerate().find(
          |(_, (pos2, _))| {*pos2 == (*pos1 as isize + *offset as isize) as usize}
        ).expect("Jump instruction gave an invalid ref mut offset");
        *offset = (index2 - index1) as i32;
      }
      _ => {}
    }
  }
  initial.iter().map(|val| val.1.clone()).collect::<Vec<Instructions>>()
}
