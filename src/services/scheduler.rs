 use crate::services::scraper_service::fetch_rate;

use crate::services::sanity_service::{
    get_sources_from_sanity,
    update_source_rate,
};

pub async fn update_market_data() {

    println!("Updating market data...");

    let sources =
        match get_sources_from_sanity().await {

            Ok(data) => data,

            Err(err) => {

                println!(
                    "SANITY ERROR => {:?}",
                    err
                );

                return;
            }
        };

    let results =
        sources["result"]
            .as_array()
            .cloned()
            .unwrap_or_default();

    for item in results {

        let id =
            item["_id"]
                .as_str()
                .unwrap_or("")
                .to_string();

        let title =
            item["title"]
                .as_str()
                .unwrap_or("Unknown");

        let raw_url =
            item["url"]
                .as_str()
                .unwrap_or("");

        println!(
            "SOURCE => {}",
            title
        );

        println!(
            "RAW URL => {}",
            raw_url
        );

        let clean_url =
            if let Some(start) =
                raw_url.find("https://") {

                let tail =
                    &raw_url[start..];

                if let Some(end) =
                    tail.find('"') {

                    tail[..end]
                        .to_string()

                } else {

                    tail.to_string()
                }

            } else {

                raw_url.to_string()
            };

        println!(
            "CLEAN URL => {}",
            clean_url
        );

        let rate =
            fetch_rate(&clean_url)
                .await;

        println!(
            "SCRAPER RESULT => {:?}",
            rate
        );

        if let Some(rate) = rate {

            let _ =
                update_source_rate(
                    id,
                    rate
                )
                .await;

            println!(
                "UPDATED {} => {}",
                title,
                rate
            );
        }
    }
}