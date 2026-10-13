
mod models;
mod services;
mod handlers;
use services::scheduler::
    update_market_data;
use axum::{
    Router,
    routing::{get, post},
};

use dotenv::dotenv;
use tower_http::cors::CorsLayer;
use handlers::source_handler::sources;
use handlers::report_handler::{
    parallel_market_report,
    parallel_rates,
    official_rates,
};
use handlers::ai_handler::ask_agent;
use tokio::time::{
    sleep,
    Duration
};
#[tokio::main]
async fn main() {
    dotenv().ok();


println!("Calling DeepSeek...");

    
    let app = Router::new()

    .route(
        "/sources",
        get(sources)
    )

    .route(
        "/parallel-market-report",
        get(parallel_market_report)
    )

    .route(
        "/parallel-rates",
        get(parallel_rates)
    )
    .route(
    "/official-rates",
    get(official_rates)
)
.route(
    "/test-official",
    get(|| async { "OFFICIAL OK" })
)

    .route(
    "/ask",
    post(ask_agent)
)
    .layer(
        CorsLayer::permissive()
    );

   

use tokio::time::{
    sleep,
    Duration
};

   
    let port =
    std::env::var("PORT")
        .unwrap_or("3000".to_string());

let listener =
    tokio::net::TcpListener::bind(
        format!("0.0.0.0:{port}")
    )
    .await
    .unwrap();

   

    axum::serve(
        listener,
        app
    )
    .await
    .unwrap();
    tokio::spawn(async move {

    loop {

        println!(
            "Updating market data..."
        );

        let _ =
            update_market_data()
                .await;

        sleep(
            Duration::from_secs(
                60 * 60 * 8
            )
        )
        .await;

    }

});
// println!(
//     "{} => {}",
//     source_name,
//     usd_rate
// );

tokio::spawn(async move {

    loop {

        update_market_data()
            .await;

        sleep(
            Duration::from_secs(
                60 * 60 * 8
            )
        )
        .await;

    }

});

}