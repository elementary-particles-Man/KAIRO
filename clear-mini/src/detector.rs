use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

pub struct Window {
    win: Duration,
    map: HashMap<u64, VecDeque<Instant>>,
}

impl Window {
    pub fn new(seconds: u64) -> Self {
        Self {
            win: Duration::from_secs(seconds),
            map: HashMap::new(),
        }
    }

    pub fn hit(&mut self, key: u64) -> usize {
        let now = Instant::now();
        let queue = self.map.entry(key).or_insert_with(VecDeque::new);
        queue.push_back(now);

        while let Some(&front) = queue.front() {
            if now.duration_since(front) > self.win {
                queue.pop_front();
            } else {
                break;
            }
        }

        queue.len()
    }
}
