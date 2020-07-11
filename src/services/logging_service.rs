use crate::services::settings_service::SettingsService;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::render::RenderState;
use lazy_static::lazy_static;
use specs::{Read, System};
lazy_static! {
    pub static ref LOG_BUFFER: LoggingQueue = Mutex::new(Vec::new());
}

/// Handles logging to console, log buffering and writing logs to files
/// Running the log macros add item to this log file list, which gets flushed to the log file and at the end of the frame.
pub struct LoggingService {
    log_file: Arc<Mutex<File>>,
}

#[macro_export]
macro_rules! log_error {
    ( $str:expr ) => {
        crate::services::logging_service::LOG_BUFFER
            .lock()
            .unwrap()
            .push((true, String::from($str)));
    };
    ( $str:expr, $data:expr ) => {
        crate::services::logging_service::LOG_BUFFER
            .lock()
            .unwrap()
            .push((true, format!($str, $data)));
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! log {
    ( $str:expr ) => {
        crate::services::logging_service::LOG_BUFFER
            .lock()
            .unwrap()
            .push((false, String::from($str)));
    };
    ( $str:expr, $data:expr ) => {
        crate::services::logging_service::LOG_BUFFER
            .lock()
            .unwrap()
            .push((false, format!($str, $data)));
    };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! log {
    ( $str:expr ) => {
        crate::services::logging_service::LOG_BUFFER
            .lock()
            .unwrap()
            .push((false, String::from($str)));
    };
    ( $str:expr, $data:expr ) => {
        crate::services::logging_service::LOG_BUFFER
            .lock()
            .unwrap()
            .push((false, format!($str, $data)));
    };
}

pub type LoggingQueue = Mutex<Vec<(bool, String)>>;

impl LoggingService {
    pub fn new(settings: &SettingsService) -> LoggingService {
        let info_file = File::create(format!("{}info.log", settings.path))
            .expect(&format!("Cannot open log file {}info.log", settings.path));

        LoggingService {
            log_file: Arc::new(Mutex::new(info_file)),
        }
    }

    pub fn flush_buffer(&self) {
        let file = self.log_file.clone();

        // Read buffer
        thread::spawn(move || {
            let mut data = LOG_BUFFER.lock().unwrap();
            let mut file = file.lock().unwrap();

            for (error, message) in data.as_slice() {
                let log_type = if *error { "ERROR" } else { "INFO" };

                println!("[{}] {}", log_type, message);
                if let Err(e) = file.write_all(format!("[{}] {}\n", log_type, message).as_bytes()) {
                    println!("\n\n\n\nERROR: CANNOT WRITE TO LOG FILE info.log IN MAIN APPLICATION FOLDER {} \n\n\n\n", e);
                }
            }

            if let Err(e) = file.flush() {
                println!("\n\n\n\nERROR: CANNOT WRITE TO LOG FILE info.log IN MAIN APPLICATION FOLDER {} \n\n\n\n", e);
            }
            data.clear();
        });
    }
}

impl Default for LoggingService {
    fn default() -> Self {
        unimplemented!()
    }
}

pub struct LoggingSystem;

impl<'a> System<'a> for LoggingSystem {
    type SystemData = (Read<'a, LoggingService>, Read<'a, RenderState>);

    fn run(&mut self, (logging_service, render_state): Self::SystemData) {
        // Frames counter is at 0 once per second
        if render_state.frames != 0 {
            return;
        }

        logging_service.flush_buffer();
    }
}
