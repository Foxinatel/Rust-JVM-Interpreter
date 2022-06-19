pub struct StreamReader {
  pub stream: Vec<u8>,
  pub ptr: usize
}

impl StreamReader {
  pub fn from(stream: Vec<u8>) -> Self { return Self { stream, ptr: 0 } }

  pub fn done(&self) -> bool { return self.ptr >= self.stream.len(); }

  fn next(&mut self) -> u8 {
    self.ptr += 1;
    self.stream[self.ptr - 1]
  }

  pub fn get_i8(&mut self) -> i8 { self.next() as i8 }

  pub fn get_u8(&mut self) -> u8 { self.next() }

  pub fn get_i16(&mut self) -> i16 { i16::from_be_bytes([self.next(), self.next()]) }

  pub fn get_u16(&mut self) -> u16 { u16::from_be_bytes([self.next(), self.next()]) }

  pub fn get_i32(&mut self) -> i32 {
    i32::from_be_bytes([self.next(), self.next(), self.next(), self.next()])
  }

  pub fn get_u32(&mut self) -> u32 {
    u32::from_be_bytes([self.next(), self.next(), self.next(), self.next()])
  }

  pub fn take_n(&mut self, n: usize) -> Vec<u8> {
    let ret = self.stream[self.ptr..n + self.ptr].to_vec();
    self.ptr += n;
    ret
  }
}
