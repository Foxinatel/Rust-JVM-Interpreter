use self::attribute::Attribute;
use super::cp_info_resolved::ResolvedCpInfo;
use crate::stream_reader::StreamReader;

pub mod annotation_default;
pub mod attribute;
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

impl Attribute {
  pub fn read(sr: &mut StreamReader, constant_pool: &Vec<ResolvedCpInfo>) -> Attribute {
    let attribute_name_index = sr.get_u16();
    let attribute_length = sr.get_u32();
    match &constant_pool[attribute_name_index as usize - 1] {
      ResolvedCpInfo::Utf8(string) => match string.as_str() {
        "ConstantValue" => constant_value::read(sr),
        "Code" => code::read(sr, constant_pool),
        "StackMapTable" => stack_map_table::read(sr),
        "Exceptions" => exceptions::read(sr),
        "InnerClasses" => inner_classes::read(sr),
        "EnclosingMethod" => enclosing_method::read(sr),
        "Synthetic" => Attribute::Synthetic,
        "Signature" => signature::read(sr, constant_pool),
        "SourceFile" => source_file::read(sr, constant_pool),
        "SourceDebugExtension" => source_debug_extensions::read(sr, attribute_length),
        "LineNumberTable" => line_number_table::read(sr),
        "LocalVariableTable" => local_variable_table::read(sr),
        "LocalVariableTypeTable" => local_variable_type_table::read(sr),
        "Deprecated" => Attribute::Deprecated,
        "RuntimeVisibleAnnotations" => runtime_annotations::read::<true>(sr),
        "RuntimeInvisibleAnnotations" => runtime_annotations::read::<false>(sr),
        "RuntimeVisibleParameterAnnotations" => runtime_parameter_annotations::read::<true>(sr),
        "RuntimeInvisibleParameterAnnotations" => runtime_parameter_annotations::read::<false>(sr),
        "AnnotationDefault" => annotation_default::read(sr, constant_pool),
        "BootstrapMethods" => bootstrap_methods::read(sr),
        _ => todo!()
      },
      _ => panic!("Constant at index {} was not a valid Utf8 identifier", attribute_name_index)
    }
  }
}
