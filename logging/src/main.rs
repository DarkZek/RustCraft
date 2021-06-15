#![feature(once_cell)]

use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::thread;
use std::path::PathBuf;
use std::lazy::SyncOnceCell;

#[macro_use]
extern crate lazy_static;

pub static LOGGING_SERVICE: SyncOnceCell<LoggingService> = SyncOnceCell::new();

pub type LoggingQueue = Mutex<Vec<(u8, String)>>;

lazy_static! {
    pub static ref LOG_BUFFER: LoggingQueue = Mutex::new(Vec::new());
}

/// Handles logging to console, log buffering and writing logs to files
/// Running the log macros add item to this log file list, which gets flushed to the log file and at the end of the frame.
pub struct LoggingService {
    log_file: Arc<Mutex<File>>,
}

impl LoggingService {
    pub fn new(path: &PathBuf) {
        let mut path = path.clone();
        path.push("info");
        path.set_extension("log");

        let info_file = if let Result::Ok(val) = File::create(&path) {
            val
        } else {
            panic!("Cannot open log file {:?}", path.clone());
        };

        LOGGING_SERVICE.set(LoggingService {
            log_file: Arc::new(Mutex::new(info_file)),
        });
    }

    pub fn flush_buffer(&self) {
        let file = self.log_file.clone();

        // Read buffer
        thread::spawn(move || {
            let mut data = LOG_BUFFER.lock().unwrap();
            let mut file = file.lock().unwrap();

            for (error, message) in data.as_slice() {
                let msg = format_message(error, message);

                if let Err(e) = file.write_all(msg.add("\n").as_bytes()) {
                    println!("\n\n\n\nERROR: CANNOT WRITE TO LOG FILE info.log IN MAIN APPLICATION FOLDER {} \n\n\n\n", e);
                }
            }

            if let Err(e) = file.flush() {
                println!("\n\n\n\nERROR: CANNOT WRITE TO LOG FILE info.log IN MAIN APPLICATION FOLDER {} \n\n\n\n", e);
            }
            data.clear();
        });
    }

    pub fn flush_now(&self) {

        let mut data = LOG_BUFFER.lock().unwrap();
        let mut file = self.log_file.lock().unwrap();

        for (error, message) in data.as_slice() {
            let msg = format_message(error, message);

            if let Err(e) = file.write_all(msg.add("\n").as_bytes()) {
                println!("\n\n\n\nERROR: CANNOT WRITE TO LOG FILE info.log IN MAIN APPLICATION FOLDER {} \n\n\n\n", e);
            }
        }

        if let Err(e) = file.flush() {
            println!("\n\n\n\nERROR: CANNOT WRITE TO LOG FILE info.log IN MAIN APPLICATION FOLDER {} \n\n\n\n", e);
        }
        data.clear();
    }
}

fn format_message(error: &u8, message: &String) -> String {
    match *error {
        0 => {
            println!("{}[ERROR] {}{}", "\x1B[31m\x1B[1m", message, "\x1B[0m");
            format!("[ERROR] {}", message)
        }
        1 => {
            println!("{}[WARN] {}{}", "\x1B[33m", message, "\x1B[0m");
            format!("[WARN] {}", message)
        }
        _ => {
            println!("[INFO] {}", message);
            format!("[INFO] {}", message)
        }
    }
}

impl Default for LoggingService {
    fn default() -> Self {
        unimplemented!()
    }
}

#[macro_export]
macro_rules! flush_log {
    () => {
        rc_logging::LOGGING_SERVICE.get().unwrap().flush_buffer();
    };
}

#[macro_export]
macro_rules! log_error {
    ( $str:expr ) => {
        {
            rc_logging::LOG_BUFFER
                .lock()
                .unwrap()
                .push((0, String::from($str)));
            rc_logging::LOGGING_SERVICE.get().unwrap().flush_now();
        }
    };
    ( $str:expr, $data:expr ) => {
        {
            rc_logging::LOG_BUFFER
                .lock()
                .unwrap()
                .push((0, format!($str, $data)));
            rc_logging::LOGGING_SERVICE.get().unwrap().flush_now();
        }
    };
    ( $str:expr, $data:expr, $data2:expr ) => {
        {
            rc_logging::LOG_BUFFER
                .lock()
                .unwrap()
                .push((0, format!($str, $data, $data2)));
            rc_logging::LOGGING_SERVICE.get().unwrap().flush_now();
        }
    };
}

#[macro_export]
macro_rules! log_warn {
    ( $str:expr ) => {
        rc_logging::LOG_BUFFER
            .lock()
            .unwrap()
            .push((1, String::from($str)));
    };
    ( $str:expr, $data:expr ) => {
        rc_logging::LOG_BUFFER
            .lock()
            .unwrap()
            .push((1, format!($str, $data)));
    };
    ( $str:expr, $data:expr, $data2:expr ) => {
        rc_logging::LOG_BUFFER
            .lock()
            .unwrap()
            .push((1, format!($str, $data, $data2)));
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! log {
    ( $str:expr ) => {
        rc_logging::LOG_BUFFER
            .lock()
            .unwrap()
            .push((2, String::from($str)));
    };
    ( $str:expr, $data:expr ) => {
        rc_logging::LOG_BUFFER
            .lock()
            .unwrap()
            .push((2, format!($str, $data)));
    };
    ( $str:expr, $data:expr, $data2:expr ) => {
        rc_logging::LOG_BUFFER
            .lock()
            .unwrap()
            .push((2, format!($str, $data, $data2)));
    };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! log {
    ( $str:expr ) => {
        rc_logging::LOG_BUFFER
            .lock()
            .unwrap()
            .push((2, String::from($str)));
    };
    ( $str:expr, $data:expr ) => {
        rc_logging::LOG_BUFFER
            .lock()
            .unwrap()
            .push((2, format!($str, $data)));
    };
    ( $str:expr, $data:expr, $data2:expr ) => {
        rc_logging::LOG_BUFFER
            .lock()
            .unwrap()
            .push((2, format!($str, $data, $data2)));
    };
}
