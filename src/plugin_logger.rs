#![allow(dead_code)]
use chrono::Local;
use std::io::BufRead;
use std::thread::{self, JoinHandle};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};


fn log(line: &str, plug_name: &str) {
    println!("[{}] [{}] {}", plug_name, Local::now().format("%Y-%m-%d %H:%M:%S"), line);
}

pub struct PluginLogger {
    stopper: Arc<AtomicBool>,
    routines: Vec<JoinHandle<()>>
}

impl PluginLogger {
    /// Creates an instance of `PluginLogger`
    /// Immediately starts reading from streams
    pub fn new(
        plug_name: &str,
        streams: Vec<Box<dyn BufRead + Send>>
    ) -> PluginLogger {
        let mut routines: Vec<JoinHandle<()>> = Vec::new();
        let stopper = Arc::new(AtomicBool::new(false));

        for stream in streams {
            let name_copy = plug_name.clone().to_string();
            let stopper = stopper.clone();
            let handler = thread::spawn(move|| stream_logger(&name_copy, stream, stopper, log));
            routines.push(handler);
        }

        return Self {stopper, routines}
    }

    pub fn stop(self) {
        self.stopper.store(true, Ordering::Relaxed);
        for routine in self.routines {
            routine.join().unwrap();
        };
    }
}


fn stream_logger<F>(plug_name: &str, stream: Box<dyn BufRead>, stop_event: Arc<AtomicBool>, logger: F)
where F: Fn(&str, &str)
{
    for line in stream.lines() {
        match line {
            Ok(line) if line.len() > 0 => logger(&line, plug_name),
            Err(e) => println!("Cannot read data: {:?}", e),
            _ => {},
        }
        if stop_event.load(Ordering::Relaxed) {
            break;
        }
    }
}


