use std::{sync::{Arc, Mutex, mpsc::{self, Receiver, Sender}}, thread, time::{Duration, Instant}};
use lazy_static::lazy_static;
use tokio::time::interval;
use crate::data::{INSTANCES, TEMPLATES, template::Template};

enum TransmitionState {
    Shutdown,
    Force,
}

pub struct SaveWorker {
    dir: String,
    instances: Arc<Mutex<Vec<Template>>>, 
    templates: Arc<Mutex<Vec<Template>>>,

}

impl SaveWorker {

    pub fn new() -> Self {
        let dir = std::env::var("DIR").unwrap_or("./backup/".to_string());
        std::fs::create_dir(&dir);
        //let (tx, rx) = mpsc::channel();
        let instances = Arc::clone(&INSTANCES);
        let templates = Arc::clone(&TEMPLATES);
        
        Self { 
            dir, instances, templates
        }
    }

    pub fn init(self) -> Arc<Self> {
        let this = Arc::new(self);
        let arc = Arc::clone(&this);
        let handle = thread::spawn(move || {
            let interval_time = u64::from_str_radix(std::env::var("SAVE_FREQ").unwrap_or("120".to_string()).as_str(), 10).unwrap_or(120);

            let mut last_instant = Instant::now();
            loop {
                let current_instant = Instant::now();
                // Time is up
                if current_instant.duration_since(last_instant).as_secs() >= interval_time {
                    arc.save();
                    last_instant = Instant::now();
                }
            }
        });
        this
    }

    fn save(&self) {
        let instances_mutex = self.instances.lock().unwrap();
        let templates_mutex = self.templates.lock().unwrap();
        std::fs::write(format!("{}/instances.json", self.dir), serde_json::to_string_pretty(&*instances_mutex).unwrap()).expect("Failed to write backup.");
        std::fs::write(format!("{}/templates.json", self.dir), serde_json::to_string_pretty(&*templates_mutex).unwrap()).expect("Failed to write backup.");
    }

    pub fn shutdown(&self) {

    }
}