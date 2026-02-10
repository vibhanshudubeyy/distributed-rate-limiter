use std::time::Instant;

#[derive(Debug)]
pub struct TokenBucket {
    capacity: f64,
    tokens: f64,
    refill_rate: f64, // tokens per second
    last_refill: Instant,
}

impl TokenBucket {
    pub fn new(capacity: f64, refill_rate: f64) -> Self {
        Self {
            capacity,
            tokens: capacity,
            refill_rate,
            last_refill: Instant::now(),
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        let refill_tokens = elapsed * self.refill_rate;

        self.tokens = (self.tokens + refill_tokens).min(self.capacity);
        self.last_refill = now;
    }

    pub fn allow(&mut self, cost: f64) -> bool {
        self.refill();
        if self.tokens >= cost {
            self.tokens -= cost;
            true
        } else {
            false
        }
    }
}
