fn main() {
    println!("Hello, world!");
    let lgr = RLog {};
    lgr.Debug("");
}

struct RLog {}

impl RLog {
    fn Debug(&self, log: &str) {}
    fn Info(&self) {}
    fn Warn(&self) {}
    fn Error(&self) {}
}
