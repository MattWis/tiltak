use serde::Deserialize;
use serde::Serialize;
use std::time::Duration;

#[cfg(feature = "aws-lambda-client")]
pub mod client;
#[cfg(feature = "aws-lambda-runtime")]
pub mod server;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TimeControl {
    FixedNodes(u64),
    Time(Duration, Duration), // Total time left, increment
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Event {
    pub size: usize,
    pub tps: Option<String>,
    pub moves: Vec<String>,
    pub time_control: TimeControl,
    pub dirichlet_noise: Option<f32>,
    pub rollout_depth: u16,
    pub rollout_temperature: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Output {
    pub pv: Vec<String>,
    pub score: f32,
}
