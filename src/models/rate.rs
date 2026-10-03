use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceRate {
    pub source_name: String,
    pub confidence: f64,
    pub usd_rate: f64,
}