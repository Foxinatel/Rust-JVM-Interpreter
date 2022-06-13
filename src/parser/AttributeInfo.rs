use super::{
    helpers::{get_u16, get_u32, take_n},
    CpInfo::{CpInfo, CONSTANT},
    ExceptionTable::ExceptionTable,
    StackMapFrame::StackMapFrame
};

#[derive(Debug)]
struct LineNumber {
    start_pc: u16,
    line_number: u16
}

#[derive(Debug)]
enum ATTRIBUTE {
    ConstantValue {constantvalue_index: u16},
    Code {max_stack: u16, max_locals: u16, code_length: u32, code: Vec<u8>, exception_table_length: u16, exception_table: Vec<ExceptionTable>, attributes_count: u16, attributes: Vec<AttributeInfo>},
    StackMapTable {number_of_entries: u16, entries: Vec<StackMapFrame>},
    Exceptions {number_of_exceptions: u16, exception_index_table: Vec<u16>},
    InnerClasses {number_of_classes: u16, classes: Vec<u16>},
    EnclosingMethod {class_index: u16, method_index: u16},
    Synthetic,
    Signature {signature_index: u16},
    SourceFile {sourcefile_index: u16},
    SourceDebugExtension {debug_extension: Vec<u8>},
    LineNumberTable {line_number_table_length: u16, line_number_table: Vec<LineNumber>}
}

#[derive(Debug)]
pub struct AttributeInfo {
    attribute_name_index: u16,
    attribute_length: u32,
    attribute: ATTRIBUTE
}

impl AttributeInfo {
    pub fn read(buf: &mut &[u8], constant_pool: &Vec<CpInfo>) -> Self {
        let attribute_name_index = get_u16(buf);
        let attribute_length = get_u32(buf);
        let attribute = match &constant_pool[attribute_name_index as usize -1].info {
            CONSTANT::Utf8 { tag: _, length: _, bytes } => {
                match bytes.as_str() {
                    "ConstantValue" => ATTRIBUTE::ConstantValue { constantvalue_index: get_u16(buf) },
                    "Code" => {
                        let max_stack = get_u16(buf);
                        let max_locals = get_u16(buf);
                        let code_length = get_u32(buf);
                        let code = take_n(code_length as usize, buf);
                        let exception_table_length = get_u16(buf);
                        let exception_table: Vec<ExceptionTable> = (0..exception_table_length).map(|_|
                            ExceptionTable::read(buf)
                        ).collect();
                        let attributes_count =  get_u16(buf);
                        let attributes: Vec<AttributeInfo> = (0..attributes_count).map(|_|
                            AttributeInfo::read(buf, constant_pool)
                        ).collect();
                        ATTRIBUTE::Code { max_stack, max_locals, code_length, code, exception_table_length, exception_table, attributes_count, attributes}
                    }
                    "StackMapTable" => {
                        let number_of_entries = get_u16(buf);
                        let entries: Vec<StackMapFrame> = (0..number_of_entries).map(|_|
                            StackMapFrame::read(buf)
                        ).collect();
                        ATTRIBUTE::StackMapTable { number_of_entries, entries}
                    }
                    "Exceptions" => todo!(),
                    "InnerClasses" => todo!(),
                    "EnclosingMethod" => todo!(),
                    "Synthetic" => todo!(),
                    "Signature" => todo!(),
                    "SourceFile" => ATTRIBUTE::SourceFile { sourcefile_index: get_u16(buf) },
                    "SourceDebugExtension" => todo!(),
                    "LineNumberTable" => {
                        let line_number_table_length = get_u16(buf);
                        let line_number_table: Vec<LineNumber> = (0..line_number_table_length).map(|_|
                            LineNumber { start_pc: get_u16(buf), line_number: get_u16(buf) }
                        ).collect();
                        ATTRIBUTE::LineNumberTable { line_number_table_length, line_number_table }
                    }
                    "LocalVariableTable" => todo!(),
                    "LocalVariableTypeTable" => todo!(),
                    "Deprecated" => todo!(),
                    "RuntimeVisibleAnnotations" => todo!(),
                    "RuntimeInvisibleAnnotations" => todo!(),
                    "RuntimeVisibleParameterAnnotations" => todo!(),
                    "RuntimeInvisibleParameterAnnotations" => todo!(),
                    "AnnotationDefault" => todo!(),
                    "BootstrapMethods" => todo!(),
                    _ => todo!()
                }
            }
            _ => panic!("Constant at index {} was not a valid Utf8 identifier", attribute_name_index)
        };

        Self {attribute_name_index, attribute_length, attribute}
    }
}