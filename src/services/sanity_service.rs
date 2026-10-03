use serde_json::Value;
use reqwest::Client;

pub async fn get_sources_from_sanity()
-> Result<Value, reqwest::Error> {

    let project_id =
        std::env::var("SANITY_PROJECT_ID")
            .unwrap();

    let dataset =
        std::env::var("SANITY_DATASET")
            .unwrap();

    let token =
        std::env::var("SANITY_TOKEN")
            .unwrap();

    let query =
        "*[_type == 'source']";

    let url = format!(
        "https://{}.api.sanity.io/v2021-10-21/data/query/{}?query={}",
        project_id,
        dataset,
        query
    );

    let result =
        Client::new()
            .get(url)
            .bearer_auth(token)
            .send()
            .await?
            .json::<Value>()
            .await?;

    Ok(result)
}