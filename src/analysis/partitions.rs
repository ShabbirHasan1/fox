use crate::analysis::Partition;

impl Partition {

    pub fn new() -> Self {
        Self {
            sum: 0f64,
            average: 0f64,
            high: 0f64,
            low: f64::MAX,
            direction: 0f64,
            volume: 0usize,
        }
    }

    pub fn update(&mut self, price: f64) {
        self.sum += price;
        self.volume += 1usize;
        if price > self.high { self.high = price; }
        if price < self.low { self.low = price; }
        self.average = self.sum/self.volume as f64;
        if self.direction == 0f64 { self.direction = price; }
        else { self.direction = price - self.direction; }
    }
    
    pub fn wipe(&mut self) {
        self.sum = 0f64;
        self.average = 0f64;
        self.high = 0f64;
        self.low = f64::MAX;
        self.direction = 0f64;
        self.volume = 0usize;
    }

    pub fn average(&self) -> f64 {
        self.average
    }
}
