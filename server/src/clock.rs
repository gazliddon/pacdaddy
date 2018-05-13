use std::time::SystemTime;

#[derive(Clone)]
pub struct Clock {
    start_time : SystemTime
}
pub fn as_secs(n : u64) -> f64 {
    (n as f64 / 1000000000.0)
}

impl Clock {
    pub fn now(&self) -> u64 {
        let dur = SystemTime::now().duration_since(self.start_time).unwrap();

        let secs = dur.as_secs() as u64;
        let ns = dur.subsec_nanos() as u64;
        secs * 1_000_000_000 + ns
    }

    pub fn now_secs(&self) -> f64 {
        as_secs(self.now())
    }

    pub fn new() -> Self {
        Self {
            start_time : SystemTime::now()
        }
    }

}

