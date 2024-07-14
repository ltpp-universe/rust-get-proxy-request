use crate::log::log;
use crate::utils::time;
use std::{
    fmt, sync,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
// 时间获取出错
const GET_TIME_FAIL: &str = "GET_TIME_FAIL";
// 绿色
pub const GREEN: &'static str = "\x1B[32m";
// 红色
pub const RED: &'static str = "\x1B[31m";
// 黄色
pub const YELLOW: &'static str = "\x1B[33m";
// 蓝色
pub const BLUE: &'static str = "\x1B[34m";
// 洋红色
pub const MAGENTA: &'static str = "\x1B[35m";
// 青色
pub const CYAN: &'static str = "\x1B[36m";
// 白色
pub const WHITE: &'static str = "\x1B[37m";
// 结束
const END: &'static str = "\x1B[0m";
// 锁
const PRINTLN_MUTEX: sync::Mutex<()> = sync::Mutex::new(());

/**
 * 输出
 */
fn base_print<T: fmt::Display + fmt::Debug>(str: &T, color: &str, has_br: bool) {
    let mut print_msg: String = String::new();
    let mut log_msg: String = String::new();
    let now: Duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect(GET_TIME_FAIL);
    let now: String = time::format_now_time();
    let print_mutex: sync::Mutex<()> = PRINTLN_MUTEX;
    let mut lock: sync::MutexGuard<()> = match print_mutex.lock() {
        Ok(tem_lock) => tem_lock,
        _ => {
            return;
        }
    };
    if has_br {
        print_msg = format!("{}[{}]\n{}{}{}{}\n", GREEN, now, END, color, *str, END);
        log_msg = format!("[{}]\n{}\n", now, *str);
    } else {
        print_msg = format!("{}[{}]\n{}{}{}{}", GREEN, now, END, color, *str, END);
        log_msg = format!("[{}]\n{}", now, *str);
    }
    print!("{}", print_msg);
    drop(lock);
    log::write(&log_msg);
}

/**
 * 不含换行输出
 */
pub fn print<T: fmt::Display + fmt::Debug>(str: &T, color: &str) {
    base_print(str, color, false);
}

/**
 * 含换行输出
 */
pub fn println<T: fmt::Display + fmt::Debug>(str: &T, color: &str) {
    base_print(str, color, true);
}
