use crate::backend::sdsl_c;
use crate::meta::common::{self, Code};
use anyhow::Result;

pub struct BitVectorMeta;

impl BitVectorMeta {
    pub fn new() -> Self {
        Self {}
    }
}

impl common::Meta for BitVectorMeta {
    fn file_specifications(
        &self,
        parameter_values: &Vec<String>,
        _id: &str,
    ) -> Result<Vec<common::FileSpecification>> {
        // Type does not have generic parameters. Use common ID across instances.
        let id = sdsl_c::specification::get_id(&self.c_code(&parameter_values)?)?;

        let header = get_header_specification(&id)?;
        let source = get_source_specification()?;

        let c_code = self.c_code(&parameter_values)?;

        let util_specifications = common::util::file_specifications(&c_code, &id)?;
        let io_specifications = common::io::file_specifications(&c_code, Some(&c_code), &id)?;

        let mut specifications = vec![source, header];
        specifications.extend(util_specifications);
        specifications.extend(io_specifications);
        Ok(specifications)
    }
}

fn get_header_specification(id: &str) -> Result<common::FileSpecification> {
    let file_name = std::path::PathBuf::from("bit_vector.hpp");

    Ok(common::FileSpecification {
        replacements: get_header_replacements(&id),
        template_file_name: file_name.clone(),
        target_file_name: file_name.clone(),
        c_file_type: common::CFileType::Hpp,
    })
}

fn get_source_specification() -> Result<common::FileSpecification> {
    let file_name = std::path::PathBuf::from("bit_vector.cpp");
    Ok(common::FileSpecification {
        replacements: maplit::btreemap! {},
        template_file_name: file_name.clone(),
        target_file_name: file_name.clone(),
        c_file_type: common::CFileType::Cpp,
    })
}

fn get_header_replacements(id: &str) -> std::collections::BTreeMap<String, String> {
    maplit::btreemap! {
        "#define BIT_VECTOR_ID _id".to_string() => format!("#define BIT_VECTOR_ID _{}", id)
    }
}

impl common::Path for BitVectorMeta {
    fn path(&self) -> String {
        "sdsl::BitVector".to_string()
    }
}

impl common::Code for BitVectorMeta {
    fn c_code(&self, _parameter_values: &Vec<String>) -> Result<String> {
        Ok("sdsl::bit_vector".to_string())
    }
}

impl common::Parameters for BitVectorMeta {
    fn parameters(&self) -> Vec<common::params::Parameter> {
        vec![]
    }
}
