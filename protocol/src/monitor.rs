use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum MonitorMessage {
    HeartBeat,
    MachineReport(MachineReport),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MachineReport {
    pub cpu_usage: f64,
    pub memory_usage: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServiceStatus {
    Running,
    Dead,
}

crate::impl_codec_single!(MonitorMessage);
