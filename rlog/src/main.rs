use std::{str::FromStr, sync::mpsc, thread};

fn main() {
    println!("Hello, world!");
    let lgr = new_rlog();
    lgr.debug("debug log!");
    lgr.info("info log!");
    lgr.warn("warn log!");
    lgr.error("error log!");
    lgr.close();
}

struct RLog {
    queue_processor_thread_join: thread::JoinHandle<()>,
    sender: mpsc::Sender<String>,
}

fn new_rlog() -> RLog {
    let (sender, receiver): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
    RLog {
        queue_processor_thread_join: process_queue(receiver),
        sender,
    }
}

impl RLog {
    fn debug(&self, log: &str) {
        self.sender.send(String::from_str(log).unwrap());
    }
    fn info(&self, log: &str) {
        self.sender.send(String::from_str(log).unwrap());
    }
    fn warn(&self, log: &str) {
        self.sender.send(String::from_str(log).unwrap());
    }
    fn error(&self, log: &str) {
        self.sender.send(String::from_str(log).unwrap());
    }
    fn close(self) {
        self.queue_processor_thread_join.join();
    }
}

fn process_queue(recv: mpsc::Receiver<String>) -> thread::JoinHandle<()> {
    thread::spawn(|| {
        let log = recv.recv().unwrap();
        println!("{}", log);
    })
}
