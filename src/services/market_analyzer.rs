
use crate::models::rate::SourceRate;
use crate::models::report::ParallelMarketReport;

pub fn analyze_market(
    rates: Vec<SourceRate>,
) -> ParallelMarketReport {

    let valid_rates: Vec<&SourceRate> =
        rates
            .iter()
            .filter(|r| r.usd_rate > 0.0)
            .collect();

    if valid_rates.is_empty() {

        return ParallelMarketReport {

            accepted_rate: 0.0,

            confidence: 0.0,

            source_count: 0,

            summary:
                "لا توجد أسعار صالحة للتحليل"
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
            

    ParallelMarketReport {

        accepted_rate,

        confidence,

        source_count:
            valid_rates.len(),

        summary:
            "تم تحليل الأسعار بنجاح"
                .to_string(),
                

    }
    
}