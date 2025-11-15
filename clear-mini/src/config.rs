#[derive(Clone, Debug)]
pub struct Config {
    pub witness_capacity: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            witness_capacity: 4096,
        }
    }
}
