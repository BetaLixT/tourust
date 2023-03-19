use std::{sync::mpsc, thread};

fn main() {
    println!("Hello, world!");
    let lgr = RLog {};
    lgr.debug("debug log!");
    lgr.info("info log!");
    lgr.warn("warn log!");
    lgr.error("error log!");
}

struct RLog {
    queue_processor_thread_join: thread::JoinHandle<()>,
}

impl RLog {
    fn debug(&self, log: &str) {}
    fn info(&self, log: &str) {}
    fn warn(&self, log: &str) {}
    fn error(&self, log: &str) {}

    fn process_queue(&self) {
        let (sender, receiver): (mpsc::Sender<&str>, mpsc::Receiver<&str>) = mpsc::channel();
        self.queue_processor_thread_join = thread::spawn(|| {});
    }
    fn close(&self) {
        self.queue_processor_thread_join.join();
    }
}
