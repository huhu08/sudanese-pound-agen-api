
mod models;
mod services;
mod handlers;

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

#[tokio::main]
async fn main() {

    dotenv().ok();
println!(
    "KEY = {:?}",
    std::env::var("DEEPSEEK_API_KEY")
);
println!("Calling DeepSeek...");

    // println!(
    //     "PROJECT = {:?}",

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

   

    // let listener =
    //     tokio::net::TcpListener::bind(
    //         "0.0.0.0:3000"
    //     )
    //     .await
    //     .unwrap();
    let port =
    std::env::var("PORT")
        .unwrap_or("3000".to_string());

let listener =
    tokio::net::TcpListener::bind(
        format!("0.0.0.0:{port}")
    )
    .await
    .unwrap();

    // println!(
    //     "Server running on http://localhost:3000"
    // );

    axum::serve(
        listener,
        app
    )
    .await
    .unwrap();
}