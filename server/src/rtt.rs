
struct RttStats {
    max_samples: usize,
    rtts : Vec<f64>,
    pub average : f64,
    pub max : f64,

}
impl RttStats {
    pub fn new(max_samples: usize) -> Self {
        Self {
            max_samples,
            rtts : vec![],
            average: 0.0,
            max : 0.0,
        }
    }

    pub fn reset(&mut self) {
        panic!("ksjakjskajs")
    }

    pub fn add_rtt(&mut self,  rtt : f64, _time : u64 ) {

        self.rtts.push(rtt);
            
        if self.rtts.len() > self.max_samples {
            self.rtts.resize(self.max_samples, 0.0)
        }

        let sum : f64 = self.rtts.iter().sum();
        let avg = sum / self.rtts.len() as f64;
        self.average = avg;
    }
}
