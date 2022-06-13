pub fn get_u8(buf: &mut &[u8]) -> u8 {
  *buf.take_first().unwrap()
}

pub fn get_u16(buf: &mut &[u8]) -> u16 {
  let (bytes, new_buf) = buf.split_array_ref::<2>();
  *buf = new_buf;
  u16::from_be_bytes(*bytes)
}

pub fn get_u32(buf: &mut &[u8]) -> u32 {
  let (bytes, new_buf) = buf.split_array_ref::<4>();
  *buf = new_buf;
  u32::from_be_bytes(*bytes)
}

pub fn take_n(n: usize, buf: &mut &[u8]) -> Vec<u8> {
  (*buf.take(..(n as usize)).unwrap()).to_vec()
}