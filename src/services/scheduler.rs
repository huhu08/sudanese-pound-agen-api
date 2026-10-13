// use crate::services::sanity_service::{
//     get_sources_from_sanity,
//     update_source_rate,
// };

// use crate::services::scraper_service::
//     fetch_rate;

// pub async fn update_market_data() {

//     println!("Updating market data...");

//     let data =
//         match get_sources_from_sanity().await {

//             Ok(v) => v,

//             Err(err) => {

//                 println!(
//                     "SANITY ERROR => {:?}",
//                     err
//                 );

//                 return;
//             }
//         };

//     let results =
//         data["result"]
//             .as_array()
//             .cloned()
//             .unwrap_or_default();

//     for item in results {

//         let id =
//             item["_id"]
//                 .as_str()
//                 .unwrap_or("")
//                 .to_string();

//         let title =
//             item["title"]
//                 .as_str()
//                 .unwrap_or("Unknown")
//                 .to_string();

//         let url =
//             item["url"]
//                 .as_str()
//                 .unwrap_or("")
//                 .to_string();

//         println!(
//             "SCRAPING => {}",
//             title
//         );

//         if let Some(rate) =
//             fetch_rate(&url).await {

//             println!(
//                 "{} => {}",
//                 title,
//                 rate
//             );

//             let _ =
//                 update_source_rate(
//                     id,
//                     rate
//                 )
//                 .await;
//         }
//     }
// }
use crate::services::scraper_service::fetch_rate;
use crate::services::sanity_service::{
    get_sources_from_sanity,
    update_source_rate,
};

pub async fn update_market_data() {
use crate::services::scraper_service::fetch_rate;

println!("TESTING JEEBLAY");

let rate =
    fetch_rate("https://jeeblay.com/fx")
        .await;

println!(
    "SCRAPER RESULT => {:?}",
    rate
);
    let sources =
        get_sources_from_sanity()
            .await
            .unwrap();

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

        let url =
            item["url"]
                .as_str()
                .unwrap_or("");

        let title =
            item["title"]
                .as_str()
                .unwrap_or("Unknown");

        println!("SCRAPING => {}", title);

        let rate =
            fetch_rate(url).await;

        println!(
            "RATE => {:?}",
            rate
        );

        if let Some(rate) = rate {

            let _ =
                update_source_rate(
                    id,
                    rate
                )
                .await;
        }
    }
}