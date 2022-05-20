use std::{sync::{Arc, Mutex, mpsc::{self, Receiver, Sender}, atomic::{AtomicBool, Ordering}}, thread::{self, JoinHandle}, time::{Duration, Instant}};
use lazy_static::lazy_static;
use crate::{data::{INSTANCES, TEMPLATES, template::Template}, plog, pwarn, perr};

lazy_static! {
    static ref SAVE_DIR: String = std::env::var("DIR").unwrap_or("./backup/".to_string());
}

pub struct SaveWorker {
    shutdown: Arc<AtomicBool>,
    handle: JoinHandle<()>,
}

impl SaveWorker {

    pub fn new() -> Self {
        let shutdown = Arc::new(AtomicBool::new(false));
        let arc = Arc::clone(&shutdown);
        let handle = thread::spawn(move || SaveWorker::background(arc));
        Self { 
            shutdown,
            handle,
        }
    }

    fn load() {
        let instances_res = std::fs::read_to_string(format!("{}/instances.json", SAVE_DIR.to_string()));
        let templates_res = std::fs::read_to_string(format!("{}/templates.json", SAVE_DIR.to_string()));

        match instances_res {
            Ok(json_string) => {
                match serde_json::from_str::<Vec<Template>>(&json_string) {
                    Ok(instances) => {
                        let mut mutex = INSTANCES.lock().unwrap();
                        *mutex = instances;
                        plog!("Successfully loaded instance backup!")
                    },
                    Err(_) => pwarn!("No previous backup file was invalid!"),
                }
            },
            Err(_) => pwarn!("No previous backup file for templates!"),
        }

        match templates_res {
            Ok(json_string) => {
                match serde_json::from_str::<Vec<Template>>(&json_string) {
                    Ok(templates) => {
                        let mut mutex = TEMPLATES.lock().unwrap();
                        *mutex = templates;
                        plog!("Successfully loaded template backup!")
                    },
                    Err(_) => pwarn!("No previous backup file was invalid!"),
                }
            },
            Err(_) => pwarn!("No previous backup file for instances!"),
        }
    }

    fn save() {
        let instances_mutex = INSTANCES.lock().unwrap();
        let templates_mutex = TEMPLATES.lock().unwrap();
        std::fs::write(format!("{}/instances.json", SAVE_DIR.to_string()), serde_json::to_string_pretty(&*instances_mutex).unwrap()).expect("Failed to write backup.");
        std::fs::write(format!("{}/templates.json", SAVE_DIR.to_string()), serde_json::to_string_pretty(&*templates_mutex).unwrap()).expect("Failed to write backup.");
    }

    fn background(shutdown: Arc<AtomicBool>) {
        let interval_time = u64::from_str_radix(std::env::var("SAVE_FREQ").unwrap_or("120".to_string()).as_str(), 10).unwrap_or(120);
        SaveWorker::load();

        plog!("Started background process, save interval is {}s.", interval_time);

        let mut last_instant = Instant::now();
        while !shutdown.load(Ordering::SeqCst) {
            let current_instant = Instant::now();
            // Time is up
            if current_instant.duration_since(last_instant).as_secs() >= interval_time {
                SaveWorker::save();
                last_instant = Instant::now();
            }
        }
        plog!("Shutting down background process...");
        SaveWorker::save();

    }

    pub fn shutdown(self) {
        self.shutdown.store(true, Ordering::SeqCst);
        match self.handle.join() {
            Ok(_) => plog!("Successfully shut down background process!"),
            Err(_) => perr!("Something went wrong while background process was shutting down!"),
        };
    }
}