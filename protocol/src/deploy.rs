use std::path::PathBuf;

pub struct BinPack {
    pub install_script: String,
    pub service_name: String,
    pub log_dir: PathBuf,
}
