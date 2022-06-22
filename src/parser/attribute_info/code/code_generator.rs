use crate::parser::{
  cp_info_resolved::{
    Class,
    Fieldref,
    InterfaceMethodref,
    InvokeDynamic,
    Methodref,
    ResolvedCpInfo
  },
  stream_reader::StreamReader
};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum Instructions {
  nop,
  aconst_null,
  iconst { value: i32 },
  lconst { value: i64 },
  fconst { value: f32 },
  dconst { value: f64 },
  bipush { value: i8 },
  sipush { value: i16 },
  ldc { index: u8 },
  ldc_w { index: u16 },
  ldc2_w { index: u16 },
  iload { index: u8 },
  lload { index: u8 },
  fload { index: u8 },
  dload { index: u8 },
  aload { index: u8 },
  iaload,
  laload,
  faload,
  daload,
  aaload,
  baload,
  caload,
  saload,
  istore { index: u8 },
  lstore { index: u8 },
  fstore { index: u8 },
  dstore { index: u8 },
  astore { index: u8 },
  iastore,
  lastore,
  fastore,
  dastore,
  aastore,
  bastore,
  castore,
  sastore,
  pop,
  pop2,
  dup,
  dup_x1,
  dup_x2,
  dup2,
  dup2_x1,
  dup2_x2,
  swap,
  iadd,
  ladd,
  fadd,
  dadd,
  isub,
  lsub,
  fsub,
  dsub,
  imul,
  lmul,
  fmul,
  dmul,
  idiv,
  ldiv,
  fdiv,
  ddiv,
  irem,
  lrem,
  frem,
  drem,
  ineg,
  lneg,
  fneg,
  dneg,
  ishl,
  lshl,
  ishr,
  lshr,
  iushr,
  lushr,
  iand,
  land,
  ior,
  lor,
  ixor,
  lxor,
  iinc { index: u8, r#const: i8 },
  i2l,
  i2f,
  i2d,
  l2i,
  l2f,
  l2d,
  f2i,
  f2l,
  f2d,
  d2i,
  d2l,
  d2f,
  i2b,
  i2c,
  i2s,
  lcmp,
  fcmpl,
  fcmpg,
  dcmpl,
  dcmpg,
  ifeq { offset: i16 },
  ifne { offset: i16 },
  iflt { offset: i16 },
  ifge { offset: i16 },
  ifgt { offset: i16 },
  ifle { offset: i16 },
  if_icmpeq { offset: i16 },
  if_icmpne { offset: i16 },
  if_icmplt { offset: i16 },
  if_icmpge { offset: i16 },
  if_icmpgt { offset: i16 },
  if_icmple { offset: i16 },
  if_acmpeq { offset: i16 },
  if_acmpne { offset: i16 },
  goto { offset: i16 },
  jsr { offset: i16 },
  ret { index: u8 },
  tableswitch { default: i32, low: i32, high: i32, offsets: Vec<i32> },
  lookupswith { default: i32, npairs: i32, pairs: Vec<(i32, i32)> },
  ireturn,
  lreturn,
  freturn,
  dreturn,
  areturn,
  r#return,
  getstatic { fieldref: Fieldref },
  putstatic { fieldref: Fieldref },
  getfield { fieldref: Fieldref },
  putfield { fieldref: Fieldref },
  invokevirtual { methodref: Methodref },
  invokespecial { methodref: Methodref },
  invokestatic { methodref: Methodref },
  invokeinterface { interfacemethodref: InterfaceMethodref, count: u8 },
  invokedynamic { invokedynamic: InvokeDynamic },
  new { class: Class },
  newarray { atype: u8 },
  anewarray { index: u16 },
  arraylength,
  athrow,
  checkcast { index: u16 },
  instanceof { index: u16 },
  monitorenter,
  monitorexit,
  wide1 { opcode: u8, index_extension: u16 },
  wide2 { opcode: u8, index_extension: u16, constbytes: i16 },
  multianewarray { index: u16, dimensions: u8 },
  ifnull { offset: i16 },
  ifnonnull { offset: i16 },
  goto_w { offset: i32 },
  jsr_w { offset: i32 }
}

pub fn generate_instructions(
  sr: &mut StreamReader,
  constant_pool: &Vec<ResolvedCpInfo>
) -> Vec<(usize, (usize, Instructions))> {
  let mut instructions: Vec<(usize, (usize, Instructions))> = Vec::new();
  for i in 0.. {
    if sr.done() {
      return instructions;
    }
    let inst = sr.get_u8();
    instructions.push((sr.ptr, (i, match inst {
      0 => Instructions::nop,
      1 => Instructions::aconst_null,
      (2..=8) => Instructions::iconst { value: inst as i32 - 3 },
      (9..=10) => Instructions::lconst { value: inst as i64 - 9 },
      (11..=13) => Instructions::fconst { value: inst as f32 - 11.0 },
      (14..=15) => Instructions::dconst { value: inst as f64 - 14.0 },
      16 => Instructions::bipush { value: sr.get_i8() },
      17 => Instructions::sipush { value: sr.get_i16() },
      18 => Instructions::ldc { index: sr.get_u8() },
      19 => Instructions::ldc_w { index: sr.get_u16() },
      20 => Instructions::ldc2_w { index: sr.get_u16() },
      21 => Instructions::iload { index: sr.get_u8() },
      22 => Instructions::lload { index: sr.get_u8() },
      23 => Instructions::fload { index: sr.get_u8() },
      24 => Instructions::dload { index: sr.get_u8() },
      25 => Instructions::aload { index: sr.get_u8() },
      (26..=29) => Instructions::iload { index: inst - 26 },
      (30..=33) => Instructions::lload { index: inst - 30 },
      (34..=37) => Instructions::fload { index: inst - 34 },
      (38..=41) => Instructions::dload { index: inst - 38 },
      (42..=45) => Instructions::aload { index: inst - 42 },
      46 => Instructions::iaload,
      47 => Instructions::laload,
      48 => Instructions::faload,
      49 => Instructions::daload,
      50 => Instructions::aaload,
      51 => Instructions::baload,
      52 => Instructions::caload,
      53 => Instructions::saload,
      54 => Instructions::istore { index: sr.get_u8() },
      55 => Instructions::lstore { index: sr.get_u8() },
      56 => Instructions::fstore { index: sr.get_u8() },
      57 => Instructions::dstore { index: sr.get_u8() },
      58 => Instructions::astore { index: sr.get_u8() },
      (59..=62) => Instructions::istore { index: inst - 59 },
      (63..=66) => Instructions::lstore { index: inst - 63 },
      (67..=70) => Instructions::fstore { index: inst - 67 },
      (71..=74) => Instructions::dstore { index: inst - 71 },
      (75..=78) => Instructions::astore { index: inst - 75 },
      79 => Instructions::iastore,
      80 => Instructions::lastore,
      81 => Instructions::fastore,
      82 => Instructions::dastore,
      83 => Instructions::aastore,
      84 => Instructions::bastore,
      85 => Instructions::castore,
      86 => Instructions::sastore,
      87 => Instructions::pop,
      88 => Instructions::pop2,
      89 => Instructions::dup,
      90 => Instructions::dup_x1,
      91 => Instructions::dup_x2,
      92 => Instructions::dup2,
      93 => Instructions::dup2_x1,
      94 => Instructions::dup2_x2,
      95 => Instructions::swap,
      96 => Instructions::iadd,
      97 => Instructions::ladd,
      98 => Instructions::fadd,
      99 => Instructions::dadd,
      100 => Instructions::isub,
      101 => Instructions::lsub,
      102 => Instructions::fsub,
      103 => Instructions::dsub,
      104 => Instructions::imul,
      105 => Instructions::lmul,
      106 => Instructions::fmul,
      107 => Instructions::dmul,
      108 => Instructions::idiv,
      109 => Instructions::ldiv,
      110 => Instructions::fdiv,
      111 => Instructions::ddiv,
      112 => Instructions::irem,
      113 => Instructions::lrem,
      114 => Instructions::frem,
      115 => Instructions::drem,
      116 => Instructions::ineg,
      117 => Instructions::lneg,
      118 => Instructions::fneg,
      119 => Instructions::dneg,
      120 => Instructions::ishl,
      121 => Instructions::lshl,
      122 => Instructions::ishr,
      123 => Instructions::lshr,
      124 => Instructions::iushr,
      125 => Instructions::lushr,
      126 => Instructions::iand,
      127 => Instructions::land,
      128 => Instructions::ior,
      129 => Instructions::lor,
      130 => Instructions::ixor,
      131 => Instructions::lxor,
      132 => Instructions::iinc { index: sr.get_u8(), r#const: sr.get_i8() },
      133 => Instructions::i2l,
      134 => Instructions::i2f,
      135 => Instructions::i2d,
      136 => Instructions::l2i,
      137 => Instructions::l2f,
      138 => Instructions::l2d,
      139 => Instructions::f2i,
      140 => Instructions::f2l,
      141 => Instructions::f2d,
      142 => Instructions::d2i,
      143 => Instructions::d2l,
      144 => Instructions::d2f,
      145 => Instructions::i2b,
      146 => Instructions::i2c,
      147 => Instructions::i2s,
      148 => Instructions::lcmp,
      149 => Instructions::fcmpl,
      150 => Instructions::fcmpg,
      151 => Instructions::dcmpl,
      152 => Instructions::dcmpg,
      153 => Instructions::ifeq { offset: sr.get_i16() },
      154 => Instructions::ifne { offset: sr.get_i16() },
      155 => Instructions::iflt { offset: sr.get_i16() },
      156 => Instructions::ifge { offset: sr.get_i16() },
      157 => Instructions::ifgt { offset: sr.get_i16() },
      158 => Instructions::ifle { offset: sr.get_i16() },
      159 => Instructions::if_icmpeq { offset: sr.get_i16() },
      160 => Instructions::if_icmpne { offset: sr.get_i16() },
      161 => Instructions::if_icmplt { offset: sr.get_i16() },
      162 => Instructions::if_icmpge { offset: sr.get_i16() },
      163 => Instructions::if_icmpgt { offset: sr.get_i16() },
      164 => Instructions::if_icmple { offset: sr.get_i16() },
      165 => Instructions::if_acmpeq { offset: sr.get_i16() },
      166 => Instructions::if_acmpne { offset: sr.get_i16() },
      167 => Instructions::goto { offset: sr.get_i16() },
      168 => Instructions::jsr { offset: sr.get_i16() },
      169 => Instructions::ret { index: sr.get_u8() },
      170 => {
        let default = sr.get_i32();
        let low = sr.get_i32();
        let high = sr.get_i32();
        let offsets: Vec<i32> = (0..high - low + 1).map(|_| sr.get_i32()).collect();
        Instructions::tableswitch { default, low, high, offsets }
      }
      171 => {
        let default = sr.get_i32();
        let npairs = sr.get_i32();
        let pairs: Vec<(i32, i32)> = (0..npairs).map(|_| (sr.get_i32(), sr.get_i32())).collect();
        Instructions::lookupswith { default, npairs, pairs }
      }
      172 => Instructions::ireturn,
      173 => Instructions::lreturn,
      174 => Instructions::freturn,
      175 => Instructions::dreturn,
      176 => Instructions::areturn,
      177 => Instructions::r#return,
      178 => {
        let index = sr.get_u16();
        let ResolvedCpInfo::Fieldref(fieldref) = &constant_pool[index as usize - 1] else {panic!()};
        Instructions::getstatic { fieldref: fieldref.clone() }
      }
      179 => {
        let index = sr.get_u16();
        let ResolvedCpInfo::Fieldref(fieldref) = &constant_pool[index as usize - 1] else {panic!()};
        Instructions::putstatic { fieldref: fieldref.clone() }
      }
      180 => {
        let index = sr.get_u16();
        let ResolvedCpInfo::Fieldref(fieldref) = &constant_pool[index as usize - 1] else {panic!()};
        Instructions::getfield { fieldref: fieldref.clone() }
      }
      181 => {
        let index = sr.get_u16();
        let ResolvedCpInfo::Fieldref(fieldref) = &constant_pool[index as usize - 1] else {panic!()};
        Instructions::putfield { fieldref: fieldref.clone() }
      }
      182 => {
        let index = sr.get_u16();
        let ResolvedCpInfo::Methodref(methodref) = &constant_pool[index as usize - 1] else {panic!()};
        Instructions::invokevirtual { methodref: methodref.clone() }
      }
      183 => {
        let index = sr.get_u16();
        let ResolvedCpInfo::Methodref(methodref) = &constant_pool[index as usize - 1] else {panic!()};
        Instructions::invokespecial { methodref: methodref.clone() }
      }
      184 => {
        let index = sr.get_u16();
        let ResolvedCpInfo::Methodref(methodref) = &constant_pool[index as usize - 1] else {panic!()};
        Instructions::invokestatic { methodref: methodref.clone() }
      }
      185 => {
        let index = sr.get_u16();
        let count = sr.get_u8();
        assert_eq!(sr.get_u8(), 0, "Final byte of invokestatic was non-zero");

        let ResolvedCpInfo::InterfaceMethodref(interfacemethodref) = &constant_pool[index as usize - 1] else {panic!()};
        Instructions::invokeinterface { interfacemethodref: interfacemethodref.clone(), count}
      }
      186 => {
        let index = sr.get_u16();
        assert_eq!(sr.get_u16(), 0, "Final bytes of invokedynamic was non-zero");

        let ResolvedCpInfo::InvokeDynamic(invokedynamic) = &constant_pool[index as usize - 1] else {panic!()};
        Instructions::invokedynamic { invokedynamic: invokedynamic.clone() }
      }
      187 => {
        let index = sr.get_u16();
        let ResolvedCpInfo::Class(class) = &constant_pool[index as usize - 1] else {panic!()};
        Instructions::new { class: class.clone() }
      }
      188 => Instructions::newarray { atype: sr.get_u8() },
      189 => Instructions::anewarray { index: sr.get_u16() },
      190 => Instructions::arraylength,
      191 => Instructions::athrow,
      192 => Instructions::checkcast { index: sr.get_u16() },
      193 => Instructions::instanceof { index: sr.get_u16() },
      194 => Instructions::monitorenter,
      195 => Instructions::monitorexit,
      196 => {
        let opcode = sr.get_u8();
        match opcode {
          (21..=25) | (54..=58) | 169 => {
            Instructions::wide1 { opcode, index_extension: sr.get_u16() }
          }
          132 => {
            Instructions::wide2 { opcode, index_extension: sr.get_u16(), constbytes: sr.get_i16() }
          }
          _ => panic!("Attempted to perform 'wide' on invalid opcode")
        }
      }
      197 => Instructions::multianewarray { index: sr.get_u16(), dimensions: sr.get_u8() },
      198 => Instructions::ifnull { offset: sr.get_i16() },
      199 => Instructions::ifnonnull { offset: sr.get_i16() },
      200 => Instructions::goto_w { offset: sr.get_i32() },
      201 => Instructions::jsr_w { offset: sr.get_i32() },
      (202..) => panic!("Instruction not yet implemented")
    })));
  }
  instructions //technically, this should never be reached. It's just here to hint the return type to the compiler
}
