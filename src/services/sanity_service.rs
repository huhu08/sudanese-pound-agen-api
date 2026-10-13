use reqwest::Client;

use serde_json::{
    json,
    Value
};

pub async fn get_sources_from_sanity()
-> Result<Value, reqwest::Error> {

    let project_id =
        std::env::var("SANITY_PROJECT_ID")
            .expect("SANITY_PROJECT_ID missing");

    let dataset =
        std::env::var("SANITY_DATASET")
            .expect("SANITY_DATASET missing");

    let token =
        std::env::var("SANITY_TOKEN")
            .expect("SANITY_TOKEN missing");

    let query =
        "*[_type == 'source'] | order(_updatedAt desc)";

    let url = format!(
        "https://{}.api.sanity.io/v2021-10-21/data/query/{}?query={}",
        project_id,
        dataset,
        query
    );

    println!("SANITY URL => {}", url);

    let result =
        Client::new()
            .get(url)
            .bearer_auth(token)
            .send()
            .await?
            .json::<Value>()
            .await?;

    println!(
        "SANITY RESULT => {}",
        result
    );

    Ok(result)
}

pub async fn save_question(
    question: String,
    answer: String
)
-> Result<(), reqwest::Error> {

    let project_id =
        std::env::var("SANITY_PROJECT_ID")
            .expect("SANITY_PROJECT_ID missing");

    let dataset =
        std::env::var("SANITY_DATASET")
            .expect("SANITY_DATASET missing");

    let token =
        std::env::var("SANITY_TOKEN")
            .expect("SANITY_TOKEN missing");

    let url = format!(
        "https://{}.api.sanity.io/v2021-10-21/data/mutate/{}",
        project_id,
        dataset
    );

    let body = json!({
        "mutations": [
            {
                "create": {
                    "_type": "conversation",
                    "question": question,
                    "answer": answer
                }
            }
        ]
    });

    Client::new()
        .post(url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;

    Ok(())
}

pub async fn save_market_snapshot(
    accepted_rate: f64,
    confidence: f64,
    source_count: usize
)
-> Result<(), reqwest::Error> {

    let project_id =
        std::env::var("SANITY_PROJECT_ID")
            .expect("SANITY_PROJECT_ID missing");

    let dataset =
        std::env::var("SANITY_DATASET")
            .expect("SANITY_DATASET missing");

    let token =
        std::env::var("SANITY_TOKEN")
            .expect("SANITY_TOKEN missing");

    let url = format!(
        "https://{}.api.sanity.io/v2021-10-21/data/mutate/{}",
        project_id,
        dataset
    );

    let body = json!({
        "mutations": [
            {
                "create": {
                    "_type": "rateHistory",

                    "acceptedRate": accepted_rate,

                    "confidence": confidence,

                    "sourceCount": source_count,

                    "createdAt":
                        chrono::Utc::now()
                            .to_rfc3339(),

                    "debugAcceptedRate":
                        accepted_rate,

                    "debugConfidence":
                        confidence
                }
            }
        ]
    });

    println!(
        "SNAPSHOT BODY => {}",
        body
    );

    Client::new()
        .post(url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;

    Ok(())
}

pub async fn update_source_rate(
    source_id: String,
    usd_rate: f64
)
-> Result<(), reqwest::Error> {

    let project_id =
        std::env::var("SANITY_PROJECT_ID")
            .expect("SANITY_PROJECT_ID missing");

    let dataset =
        std::env::var("SANITY_DATASET")
            .expect("SANITY_DATASET missing");

    let token =
        std::env::var("SANITY_TOKEN")
            .expect("SANITY_TOKEN missing");

    let url = format!(
        "https://{}.api.sanity.io/v2021-10-21/data/mutate/{}",
        project_id,
        dataset
    );

    let body = json!({
        "mutations": [
            {
                "patch": {
                    "id": source_id,
                    "set": {
                        "usd_rate": usd_rate,
                        "last_status": "updated",
                        "last_updated":
                            chrono::Utc::now()
                                .to_rfc3339()
                    }
                }
            }
        ]
    });

    println!(
        "UPDATE SOURCE BODY => {}",
        body
    );

    Client::new()
        .post(url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;

    Ok(())
}
