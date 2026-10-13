use reqwest::Client;

pub async fn fetch_rate(
    url: &str
) -> Option<f64> {

    let html =
        Client::new()
            .get(url)
            .send()
            .await
            .ok()?
            .text()
            .await
            .ok()?;

    println!(
        "SCRAPED FROM {}",
        url
    );

    // println!(
    //     "{}",
    //     html
    // );

    // None
    println!(
    "HTML LENGTH => {}",
    html.len()
);

println!(
    "{}",
    html
        .chars()
        .take(5000)
        .collect::<String>()
);

None
}