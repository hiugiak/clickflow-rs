use std::fs;

use crate::core::Task;

pub struct Engine {}

impl Engine {
    pub fn start_tasks(&self, file_paths: Vec<&str>) {
        let mut debug_msg =
            String::from("\n========================\nReading tasks\n========================");
        for path in file_paths {
            let data = fs::read_to_string(path).unwrap();
            let task: Task = serde_yaml::from_str(&data).unwrap();
            if log::log_enabled!(log::Level::Debug) {
                debug_msg += &format!(
                    "\n{}\n------------------------\n{:#?}\n========================",
                    path, task
                );
            }
        }
        if log::log_enabled!(log::Level::Debug) {
            log::debug!("{}", debug_msg);
        }
    }
}
