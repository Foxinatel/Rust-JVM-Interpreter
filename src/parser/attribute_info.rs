use self::attribute::ATTRIBUTE;

use super::{cp_info::{CpInfo, CONSTANT}, helpers::{get_u16, get_u32}};

pub mod attribute;
pub mod annotation_default;
pub mod bootstrap_methods;
pub mod code;
pub mod constant_value;
pub mod enclosing_method;
pub mod exceptions;
pub mod inner_classes;
pub mod line_number_table;
pub mod local_variable_table;
pub mod local_variable_type_table;
pub mod runtime_annotations;
pub mod runtime_parameter_annotations;
pub mod signature;
pub mod source_debug_extensions;
pub mod source_file;
pub mod stack_map_table;

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
                    "ConstantValue" => constant_value::read(buf),
                    "Code" => code::read(buf, constant_pool),
                    "StackMapTable" => stack_map_table::read(buf),
                    "Exceptions" => exceptions::read(buf),
                    "InnerClasses" => inner_classes::read(buf),
                    "EnclosingMethod" => enclosing_method::read(buf),
                    "Synthetic" => ATTRIBUTE::Synthetic,
                    "Signature" => signature::read(buf),
                    "SourceFile" => source_file::read(buf),
                    "SourceDebugExtension" => source_debug_extensions::read(buf, attribute_length),
                    "LineNumberTable" => line_number_table::read(buf),
                    "LocalVariableTable" => local_variable_table::read(buf),
                    "LocalVariableTypeTable" => local_variable_type_table::read(buf),
                    "Deprecated" => ATTRIBUTE::Deprecated,
                    "RuntimeVisibleAnnotations" => runtime_annotations::read::<true>(buf),
                    "RuntimeInvisibleAnnotations" => runtime_annotations::read::<false>(buf),
                    "RuntimeVisibleParameterAnnotations" => runtime_parameter_annotations::read::<true>(buf),
                    "RuntimeInvisibleParameterAnnotations" => runtime_parameter_annotations::read::<false>(buf),
                    "AnnotationDefault" => annotation_default::read(buf),
                    "BootstrapMethods" => bootstrap_methods::read(buf),
                    _ => todo!()
                }
            }
            _ => panic!("Constant at index {} was not a valid Utf8 identifier", attribute_name_index)
        };
        Self {attribute_name_index, attribute_length, attribute}
    }
}