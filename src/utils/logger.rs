use log::{info, warn, error, debug};

pub struct XLog;

impl XLog {
    pub fn i(tag: &str, msg: &str) {
        info!("[{}] {}", tag, msg);
    }
    pub fn w(tag: &str, msg: &str) {
        warn!("[{}] {}", tag, msg);
    }
    pub fn w_err(tag: &str, msg: &str, err: &dyn std::fmt::Display) {
        warn!("[{}] {}: {}", tag, msg, err);
    }
    pub fn e(tag: &str, msg: &str) {
        error!("[{}] {}", tag, msg);
    }
    pub fn e_err(tag: &str, msg: &str, err: &dyn std::fmt::Display) {
        error!("[{}] {}: {}", tag, msg, err);
    }
    pub fn d(tag: &str, msg: &str) {
        debug!("[{}] {}", tag, msg);
    }
}
