use anyhow::{format_err, Result};

pub mod io;
pub mod params;
pub mod util;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub enum CFileType {
    Cpp,
    Hpp,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct FileSpecification {
    pub replacements: std::collections::BTreeMap<String, String>,
    pub template_file_name: std::path::PathBuf,
    pub target_file_name: std::path::PathBuf,
    pub c_file_type: CFileType,
}

pub trait Meta: Regex + Path + Code + Parameters {
    fn file_specifications(
        &self,
        parameter_values: &Vec<String>,
        id: &str,
    ) -> Result<Vec<FileSpecification>>;
}

pub trait Path {
    fn path(&self) -> String;
}

pub trait Code: Parameters {
    fn c_code(&self, parameter_values: &Vec<String>) -> Result<String>;
}

pub trait Parameters {
    fn parameters(&self) -> Vec<params::Parameter>;
}

pub trait Regex: Path + Parameters {
    fn parameters_regex(&self) -> Result<Option<Vec<regex::Regex>>>;
    fn default_regex(&self) -> Result<Option<regex::Regex>>;
}

impl<T: Path + Parameters> Regex for T {
    /// Return regex for structure with generic parameters.
    ///
    /// Returns None if structure does not include generic parameters.
    fn parameters_regex(&self) -> Result<Option<Vec<regex::Regex>>> {
        let parameters = self.parameters();
        if parameters.is_empty() {
            return Ok(None);
        }

        let get_regex = |parameters: &[params::Parameter]| -> Result<regex::Regex> {
            let parameters_regex = parameters
                .iter()
                .map(|p| p.regex.clone())
                .collect::<Vec<String>>()
                .join(r"\s*,\s*");
            let structure_regex = format!(
                r".*{path}<{params}>;.*",
                path = self.path(),
                params = parameters_regex
            );
            Ok(regex::Regex::new(&structure_regex)?)
        };

        let mut regexes = vec![get_regex(&parameters)?];
        for (index, parameter) in parameters.iter().rev().enumerate() {
            if parameter.has_default {
                let parameters_subset = &parameters[..parameters.len() - index];
                regexes.push(get_regex(parameters_subset)?);
            }
        }

        Ok(Some(regexes))
    }

    /// Return regex for structure without generic parameters
    /// or where all parameters have default values.
    ///
    /// Returns None if structure has generic parameters without default.
    fn default_regex(&self) -> Result<Option<regex::Regex>> {
        // The resultant regex will not capture anything unless
        // all parameters have default values.
        if !self.parameters().iter().all(|p| p.has_default) {
            return Ok(None);
        }

        let structure_regex = format!(r".*{path};.*", path = self.path(),);
        Ok(Some(regex::Regex::new(&structure_regex)?))
    }
}

pub fn get_target_file_name(
    template_file_name: &std::path::PathBuf,
    id: &str,
) -> Result<std::path::PathBuf> {
    let stem = template_file_name
        .file_stem()
        .and_then(|s| s.to_str().to_owned())
        .ok_or(format_err!(
            "Failed to find stem for file: {}",
            template_file_name.display()
        ))?;
    let extension = template_file_name
        .extension()
        .and_then(|s| s.to_str().to_owned())
        .ok_or(format_err!(
            "Failed to find extension for file: {}",
            template_file_name.display()
        ))?;
    let target_file_name = format!(
        "{stem}_{id}.{extension}",
        stem = stem,
        id = id,
        extension = extension
    );
    Ok(std::path::PathBuf::from(target_file_name))
}

pub fn c_sorted_parameters(
    parameter_values: &Vec<String>,
    parameters: &Vec<params::Parameter>,
) -> Result<Vec<String>> {
    let mut sorted_params = vec!["".to_string(); parameter_values.len()];
    for (param, value) in parameters.iter().zip(parameter_values.iter()) {
        sorted_params[param.c_index] = value.clone();
    }
    Ok(sorted_params)
}
