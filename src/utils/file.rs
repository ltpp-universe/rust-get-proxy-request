use crate::print::print::{self, GREEN, RED};
use std::path::Path;
use std::{fs, io};

/**
 * 路径是否存在
 */
pub fn dir_exists(path: &str) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}

/**
 * 创建目录和子级目录
 */
pub fn judge_creat_dir(dir_path: &str) -> bool {
    let mut res: bool = false;
    if !dir_exists(dir_path) {
        // 递归创建目录
        if let Ok(_) = fs::create_dir_all(dir_path) {
            res = true;
        }
    }
    res
}
