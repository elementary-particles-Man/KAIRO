use std::time::Duration;

/// Adaptive rate controller adjusting send rate based on network metrics.
pub struct RateController {
    min_rate: f64,
    max_rate: f64,
    current_rate: f64,
}

impl RateController {
    /// Create a new controller with given rate bounds.
    pub fn new(initial: f64, min_rate: f64, max_rate: f64) -> Self {
        Self { min_rate, max_rate, current_rate: initial.clamp(min_rate, max_rate) }
    }

    /// Update the sending rate using observed packet loss and RTT.
    /// Higher loss or longer RTT reduces the rate while low loss increases it.
    pub fn update(&mut self, loss: f64, rtt: Duration) {
        let loss_factor = (1.0 - loss).clamp(0.0, 1.0);
        let rtt_factor = 1.0 / (1.0 + rtt.as_secs_f64());
        self.current_rate *= loss_factor * rtt_factor;
        self.current_rate = self.current_rate.clamp(self.min_rate, self.max_rate);
    }

    /// Retrieve the current sending rate in bytes per second (or arbitrary units).
    pub fn rate(&self) -> f64 {
        self.current_rate
    }
}
