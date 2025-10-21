// crate::utils::log

use chrono::Local;
use std::fs::File;
use std::io::Write;

const LOGPATH: &str = "logs/latest.log";

pub fn init_logs() {
    let file_result = File::create(LOGPATH);
    match file_result {
        Ok(_) => {},
        Err(_) => {
            panic!("LOGERROR: Something went wrong while creating {}.", LOGPATH);
        }
    }
}

pub fn log(label: impl Into<String>, msg: impl Into<String>) {
    let label_ = label.into();
    let msg_ = msg.into();
    let datetime: String = format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"));
    let log:String = format!("{} [{}] {}", datetime, label_, msg_);
    println!("{}", log);
    let file_result = File::options().append(true).open(LOGPATH);
    let mut log_file = match file_result {
        Ok(_) => file_result.unwrap(),
        Err(_) => {
            panic!("LOGERROR: Something went wrong while opening {}.", LOGPATH);
        }
    };
    let write_result = writeln!(&mut log_file, "{}", log);
    match write_result {
        Ok(_) => {},
        Err(_) => {
            panic!("LOGERROR: Something went wrong while appending to {}", LOGPATH);
        }
    }
}