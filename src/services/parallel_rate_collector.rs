use crate::models::rate::SourceRate;

use crate::services::sanity_service::
    get_sources_from_sanity;

pub async fn collect_parallel_rates()
-> Vec<SourceRate> {

    let data =
        match get_sources_from_sanity().await {

            Ok(data) => data,

            Err(err) => {

                println!(
                    "SANITY ERROR => {:?}",
                    err
                );

                return Vec::new();
            }
        };

    println!(
        "SANITY DATA => {}",
        data
    );

    let results =
        data["result"]
            .as_array()
            .cloned()
            .unwrap_or_default();

    let mut rates =
        Vec::<SourceRate>::new();

    for item in results {

        println!(
            "ITEM => {:?}",
            item
        );

        let source_name =
            item["title"]
                .as_str()
                .unwrap_or("Unknown")
                .to_string();

        let usd_rate =
            item["usd_rate"]
                .as_f64()
                .unwrap_or(0.0);

        let confidence =
            item["confidence"]
                .as_f64()
                .unwrap_or(50.0);

        println!(
            "{} => {}",
            source_name,
            usd_rate
        );

        rates.push(
            SourceRate {

                source_name,

                usd_rate,

                confidence,

            }
        );
    }

    rates
}