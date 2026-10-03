use axum::Json;

use crate::models::rate::SourceRate;
use crate::models::report::ParallelMarketReport;

use crate::services::market_analyzer::analyze_market;
use crate::services::parallel_rate_collector::collect_parallel_rates;

pub async fn parallel_market_report()
-> Json<ParallelMarketReport> {

    let rates =
        collect_parallel_rates()
            .await;

    let report =
        analyze_market(rates);

    Json(report)
}

pub async fn parallel_rates()
-> Json<Vec<SourceRate>> {

    let rates =
        collect_parallel_rates()
            .await;

    Json(rates)
}
use crate::services::official_rate_collector::
    collect_official_rates;

pub async fn official_rates()
-> Json<Vec<SourceRate>> {

    let rates =
        collect_official_rates()
            .await;

    Json(rates)
}