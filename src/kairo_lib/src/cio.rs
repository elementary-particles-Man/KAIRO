// KAIRO-CIO I/O Spec
#[derive(Debug, Clone)]
pub struct KairoCIO {
    pub input_channels: Vec<String>,
    pub output_channels: Vec<String>,
    pub max_bandwidth_per_channel: u32,
    pub io_mode: IOMode,
}

#[derive(Debug, Clone)]
pub enum IOMode {
    Interrupt,
    Polling,
    Hybrid,
}

impl KairoCIO {
    pub fn describe(&self) -> String {
        format!(
            "Inputs: {} / Outputs: {} / Mode: {:?} / Bandwidth: {}kbps",
            self.input_channels.len(),
            self.output_channels.len(),
            self.io_mode,
            self.max_bandwidth_per_channel
        )
    }
}
