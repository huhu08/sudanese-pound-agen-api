use crate::models::rate::SourceRate;
use crate::models::report::ParallelMarketReport;

fn is_parallel_market(
    source_name: &str,
) -> bool {
    source_name.contains("السوق")
        || source_name.contains("جيب")
        || source_name.contains("ET")
        || source_name.contains("etcurrency")
        || source_name.contains("Jeeblay")
}

pub fn analyze_market(
    rates: Vec<SourceRate>,
) -> ParallelMarketReport {

    let valid_rates: Vec<&SourceRate> =
        rates
            .iter()
            .filter(|r| {
                r.usd_rate > 0.0
                && is_parallel_market(
                    &r.source_name
                )
            })
            .collect();

    if valid_rates.is_empty() {

        return ParallelMarketReport {

            accepted_rate: 0.0,

            confidence: 0.0,

            source_count: 0,

            summary:
                "لا توجد أسعار سوق موازي صالحة للتحليل"
                    .to_string(),
        };
    }

    let accepted_rate =
        valid_rates
            .iter()
            .map(|r| r.usd_rate)
            .sum::<f64>()
            / valid_rates.len() as f64;

    let confidence =
        valid_rates
            .iter()
            .map(|r| r.confidence)
            .sum::<f64>()
            / valid_rates.len() as f64;

    println!(
        "PARALLEL MARKET AVG => {}",
        accepted_rate
    );

    ParallelMarketReport {

        accepted_rate,

        confidence,

        source_count:
            valid_rates.len(),

        summary:
            format!(
                "تم احتساب متوسط السوق الموازي من {} مصدر",
                valid_rates.len()
            ),
    }
}