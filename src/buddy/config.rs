use serde::Deserialize;
use crate::ais::assts::CreateConfig;


#[derive(Debug,Deserialize)]
pub(super) struct Config {
    pub name: String,
    pub model: String,
    pub instruction_file: String,
    pub file_bundles: Vec<FileBundle>
}
#[derive(Debug,Deserialize)]
pub(super) struct FileBundle {
    pub bundle_name: String,
    pub src_dir: String,
    pub dst_ext: String,
    pub src_globs: Vec<String>
}

impl From<&Config> for  CreateConfig {
    fn from(config: &Config) -> Self {
        Self {
            name: config.name.clone(),
            model: config.model.clone()
        }
    }
}
