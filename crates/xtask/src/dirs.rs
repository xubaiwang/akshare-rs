use std::path::{Path, PathBuf};

pub fn workspace_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
}

/// 「數據字典」目錄
pub fn data_doc_dir() -> PathBuf {
    workspace_dir().join("python").join("docs").join("data")
}

/// 需要寫入的 `akshare/src/ops.rs` 文件位置
pub fn ops_file_path() -> PathBuf {
    workspace_dir()
        .join("crates")
        .join("akshare")
        .join("src")
        .join("ops.rs")
}
