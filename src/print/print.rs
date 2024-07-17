use crate::log::log;
use crate::utils::time;
use std::{fmt, sync};

// 绿色
pub const GREEN: &'static str = "\x1B[32m";
// 红色
pub const RED: &'static str = "\x1B[31m";
// 黄色
pub const YELLOW: &'static str = "\x1B[33m";
// 蓝色
pub const BLUE: &'static str = "\x1B[34m";
// 青色
pub const CYAN: &'static str = "\x1B[36m";
// 结束
const END: &'static str = "\x1B[0m";
// 锁
const PRINTLN_MUTEX: sync::Mutex<()> = sync::Mutex::new(());

/**
 * 输出
 */
fn base_print<T: fmt::Display + fmt::Debug>(str: &T, color: &str, has_br: bool) {
    let mut _print_msg: String = String::new();
    let mut _log_msg: String = String::new();
    let now: String = time::format_now_time();
    let print_mutex: sync::Mutex<()> = PRINTLN_MUTEX;
    let lock: sync::MutexGuard<()> = match print_mutex.lock() {
        Ok(tem_lock) => tem_lock,
        _ => {
            return;
        }
    };
    if has_br {
        _print_msg = format!("{}[{}]\n{}{}{}{}\n", GREEN, now, END, color, *str, END);
        _log_msg = format!("[{}]\n{}\n", now, *str);
    } else {
        _print_msg = format!("{}[{}]\n{}{}{}{}", GREEN, now, END, color, *str, END);
        _log_msg = format!("[{}]\n{}", now, *str);
    }
    print!("{}", _print_msg);
    drop(lock);
    log::write(&_log_msg);
}

/**
 * 含换行输出
 */
pub fn println<T: fmt::Display + fmt::Debug>(str: &T, color: &str) {
    base_print(str, color, true);
}
