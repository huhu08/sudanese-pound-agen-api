use axum::Json;

use crate::models::ai::{
    AskRequest,
    AskResponse
};

use crate::services::deepseek_service::ask_deepseek;

use crate::services::market_analyzer::analyze_market;

use crate::services::parallel_rate_collector::
    collect_parallel_rates;

use crate::services::sanity_service::
    save_question;

pub async fn ask_agent(
    Json(req): Json<AskRequest>
)
-> Json<AskResponse> {

    let rates =
        collect_parallel_rates()
            .await;

    let report =
        analyze_market(rates);

    let answer =
    match ask_deepseek(
        req.question.clone(),
        report.accepted_rate,
        report.confidence,
        report.summary.clone()
    )
    .await {

        Ok(answer) => answer,

        Err(err) => {

            println!(
                "DEEPSEEK ERROR: {:?}",
                err
            );

            format!(
                "DeepSeek Error: {:?}",
                err
            )
        }

    };

    Json(
        AskResponse {
            answer
        }
    )

}
