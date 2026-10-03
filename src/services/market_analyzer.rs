use crate::models::rate::SourceRate;
use crate::models::report::ParallelMarketReport;

pub fn analyze_market(
    rates: Vec<SourceRate>,
) -> ParallelMarketReport {

    let accepted_rate =
        rates.iter()
            .map(|r| r.usd_rate)
            .sum::<f64>()
            / rates.len() as f64;

    let confidence =
        rates.iter()
            .map(|r| r.confidence)
            .sum::<f64>()
            / rates.len() as f64;

    ParallelMarketReport {
        accepted_rate,
        confidence,
        source_count: rates.len(),
        summary:
            "تم تحليل الأسعار بنجاح"
                .to_string(),
    }
}
