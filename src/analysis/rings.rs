/*
 * Rings, aka sliding windows
*/

use crate::analysis::{Ring, Trade};

impl<const N: usize> Ring<N> {
    pub fn initialize(size: usize) -> Self {
        let ring: [Trade; N] = [Trade::default(); N];
        Self {
            ring,
            sum: 0f64,
            average: 0f64,
            index: 0usize,
            full: false,
            size,
        }
    }
    
    pub fn update(&mut self, price: f64, timestamp: u64) {
        let trade = Trade::new(price, timestamp);
        self.ring[self.index % self.size] = trade;
        if self.full {
            self.sum -= self.oldest().price;
            self.sum += price;
        }
        else { self.sum += price; }
        self.average = self.sum/self.size as f64;
        self.index += 1;
        if self.index == self.size {
            self.full = true;
        }
    }

    pub fn full(&self) -> bool {
        self.full
    }

    pub fn endpoints(&self) -> (Trade, Trade) {
        assert!(self.index != 0);
        (self.ring[self.index % self.size], self.ring[(self.index-1) % self.size])
    }

     pub fn oldest(&self) -> &Trade {
        if self.index == self.size { return &self.ring[0]; }
        &self.ring[(self.index+1) % self.size]
    }

    pub fn avg_price_change(&self) -> f64 {
        let (back, front) = self.endpoints();
        (front.price - back.price)/((front.timestamp - back.timestamp) as f64)
    }

    pub fn estimate_avg_second_derivative(&self) -> f64 {

        let step = self.size/2;
        assert!(step != 0);

        let mut index = self.index;

        let delta_time_1 = self.ring[(index + step) % self.size].timestamp - self.ring[index % self.size].timestamp; // Oldest
        let delta_price_1 = self.ring[(index + step) % self.size].price - self.ring[index % self.size].price;
        let avg_time_1 = (self.ring[(index + step) % self.size].timestamp + self.ring[index % self.size].timestamp)/2;
        let dp_dt_1 = delta_price_1/(delta_time_1 as f64);
        
        index -= step+1;

        let delta_time_2 = self.ring[(index + step) % self.size].timestamp - self.ring[index % self.size].timestamp; // Newest
        let delta_price_2 = self.ring[(index + step) % self.size].price - self.ring[index % self.size].price;
        let avg_time_2 = (self.ring[(index + step) % self.size].timestamp + self.ring[index % self.size].timestamp)/2;
        let dp_dt_2 = delta_price_2/(delta_time_2 as f64);

        (dp_dt_2 - dp_dt_1)/((avg_time_2 - avg_time_1) as f64)
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn most_recent_price(&self) -> f64 {
        self.ring[(self.index-1) % self.size].price
    }

}

impl Trade {

    pub fn new(price: f64, timestamp: u64) -> Self {
        Self {
            price,
            timestamp,
        }
    }
}

impl Default for Trade {

    fn default() -> Self {
        Self {
            price: 0f64,
            timestamp: 0u64,
        }
    }
}

