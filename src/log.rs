use std::path::PathBuf;

use directories::BaseDirs;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;

pub fn init() {
    std::env::set_var("RUST_LOG", "actix_server=trace,actix_web=trace");

    if let Err(_) = log4rs::init_file("config/log4rs.yaml", Default::default()) {
        log4rs::init_config(default_config()).unwrap();
    }
}

fn default_config() -> Config {
    let stdout = ConsoleAppender::builder().build();

    const LOG_SIZE_LIMIT_KB: u64 = 50_000;
    let trigger = SizeTrigger::new(LOG_SIZE_LIMIT_KB);

    const ROLLING_FILE_COUNT: u32 = 100;
    let roller = FixedWindowRoller::builder()
        .build(
            default_log_dir().join("logfile{}.log").to_str().unwrap(),
            ROLLING_FILE_COUNT,
        )
        .unwrap();

    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));

    let logfile = RollingFileAppender::builder()
        .build(default_log_path(), Box::new(policy))
        .unwrap();

    Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("logfile")
                .build(LevelFilter::Debug),
        )
        .unwrap()
}

#[cfg(windows)]
fn default_log_dir() -> PathBuf {
    BaseDirs::new()
        .unwrap()
        .data_local_dir()
        .join("Remote Control/log")
}

#[cfg(windows)]
fn default_log_path() -> PathBuf {
    default_log_dir().join("logfile.log")
}
