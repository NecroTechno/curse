use cursive::logger::log;
use log::{Level, Record};

pub fn curse_log(log_info: &str) {
    log(&Record::builder()
        .args(format_args!("{}", log_info))
        .level(Level::Debug)
        .build());
}
