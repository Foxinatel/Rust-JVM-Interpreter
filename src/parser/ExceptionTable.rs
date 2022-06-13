use super::helpers::get_u16;

#[derive(Debug)]
pub struct ExceptionTable {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl ExceptionTable {
    pub fn read(buf: &mut &[u8]) -> Self {
        return Self { start_pc: get_u16(buf), end_pc: get_u16(buf), handler_pc: get_u16(buf), catch_type: get_u16(buf) }
    }
}