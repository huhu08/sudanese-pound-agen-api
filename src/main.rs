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

use services::scheduler::update_market_data;

use tokio::time::{
    sleep,
    Duration,
};

#[tokio::main]
async fn main() {

    dotenv().ok();

    println!("Starting application...");

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
            get(|| async {
                "OFFICIAL OK"
            })
        )

        .route(
            "/ask",
            post(ask_agent)
        )

        .layer(
            CorsLayer::permissive()
        );

    let port =
        std::env::var("PORT")
            .unwrap_or(
                "3000".to_string()
            );

    let listener =
        tokio::net::TcpListener::bind(
            format!(
                "0.0.0.0:{}",
                port
            )
        )
        .await
        .unwrap();

    println!(
        "Running initial market update..."
    );

    update_market_data().await;

    tokio::spawn(async move {

        loop {

            println!(
                "Scheduled market update..."
            );

            update_market_data().await;

            sleep(
                Duration::from_secs(
                    60 * 60 * 8
                )
            )
            .await;
        }
    });

    println!(
        "Server listening on port {}",
        port
    );

    axum::serve(
        listener,
        app
    )
    .await
    .unwrap();
}