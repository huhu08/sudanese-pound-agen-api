use serde::Serialize;

#[derive(Serialize)]
pub struct ParallelMarketReport {
    pub accepted_rate: f64,
    pub confidence: f64,
    pub source_count: usize,
    pub summary: String,
}