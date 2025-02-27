use lazy_static::lazy_static;
use std::env;
use std::sync::Mutex;

const DEFAULT_PORT: u16 = 80;
const DEFAULT_LOG_DIR_PATH: &str = "./logs";

lazy_static! {
    static ref PORT: Mutex<u16> = Mutex::new(get_port_internal());
    static ref LOG_DIR_PATH: Mutex<String> = Mutex::new(get_log_dir_path_internal());
}

/**
 * 计算端口
 */
pub fn get_port_internal() -> u16 {
    let args: Vec<String> = env::args().collect();
    let port: u16 = args
        .get(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(DEFAULT_PORT);
    port
}

/**
 * 获取端口
 */
pub fn get_port() -> u16 {
    if let Ok(res) = PORT.lock() {
        res.clone()
    } else {
        DEFAULT_PORT
    }
}

/**
 *计算日志路径
 */
pub fn get_log_dir_path_internal() -> String {
    let args: Vec<String> = env::args().collect();
    let log_dir_path: String = args
        .get(2)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(DEFAULT_LOG_DIR_PATH.to_owned());
    log_dir_path
}

/**
 * 获取日志路径
 */
pub fn get_log_dir_path() -> String {
    if let Ok(res) = LOG_DIR_PATH.lock() {
        res.clone()
    } else {
        DEFAULT_LOG_DIR_PATH.to_owned()
    }
}
