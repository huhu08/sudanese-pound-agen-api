use regex::Regex;
use reqwest::Client;

pub async fn fetch_rate(
    url: &str,
) -> Option<f64> {

    let html =
        Client::new()
            .get(url)
            .header(
                "User-Agent",
                "Mozilla/5.0"
            )
            .send()
            .await
            .ok()?
            .text()
            .await
            .ok()?;

    println!(
        "SCRAPED FROM => {}",
        url
    );

    println!(
        "HTML LENGTH => {}",
        html.len()
    );

    //
    // JEEBLAY
    //
    if url.contains("jeeblay.com") {

        let re =
            Regex::new(
                r"الدولار بيع\s*([0-9,]+)"
            )
            .ok()?;

        if let Some(cap) =
            re.captures(&html)
        {
            let rate =
                cap[1]
                    .replace(",", "")
                    .parse::<f64>()
                    .ok()?;

            println!(
                "JEEBLAY RATE => {}",
                rate
            );

            return Some(rate);
        }
    }

    //
    // ETCURRENCY
    //
    if url.contains("etcurrency.com") {

        let re =
            Regex::new(
                r"1 دولار\s*=\s*([0-9,]+)"
            )
            .ok()?;

        if let Some(cap) =
            re.captures(&html)
        {
            let rate =
                cap[1]
                    .replace(",", "")
                    .parse::<f64>()
                    .ok()?;

            println!(
                "ETCURRENCY RATE => {}",
                rate
            );

            return Some(rate);
        }
    }

    //
    // CBOS
    //
    

      if url.contains("cbos.gov.sd") {

    let re =
        Regex::new(
            r"([0-9]{3}\.[0-9]+)"
        )
        .ok()?;

    let mut rates =
        Vec::<f64>::new();

    for cap in re.captures_iter(&html) {

        if let Ok(v) =
            cap[1].parse::<f64>()
        {
            if v > 300.0
                && v < 1000.0
            {
                rates.push(v);
            }
        }
    }

    println!(
        "CBOS REAL RATES => {:?}",
        rates
    );

    return rates.first().cloned();
}
    //
    // OMDURMAN NATIONAL BANK
    //
    // if url.contains("onb-sd.com") {

    //     let re =
    //         Regex::new(
    //             r"([0-9]{4,5})"
    //         )
    //         .ok()?;

    //     let mut candidates =
    //         Vec::<f64>::new();

    //     for cap in re.captures_iter(&html) {

    //         if let Ok(val) =
    //             cap[1].parse::<f64>()
    //         {
    //             if val > 1000.0
    //                 && val < 10000.0
    //             {
    //                 candidates.push(val);
    //             }
    //         }
    //     }

    //     println!(
    //         "ONB CANDIDATES => {:?}",
    //         candidates
    //     );

    //     return candidates.first().cloned();
    // }
    if url.contains("onb-sd.com") {

    println!(
        "========== ONB HTML START =========="
    );

    println!(
        "{}",
        html.chars()
            .take(20000)
            .collect::<String>()
    );

    println!(
        "========== ONB HTML END =========="
    );

    return None;
}

    //
    // ALMASHHAD
    //
    if url.contains("almashhadalsudani.com") {

        let re =
            Regex::new(
                r"([0-9]{4,5})"
            )
            .ok()?;

        let mut candidates =
            Vec::<f64>::new();

        for cap in re.captures_iter(&html) {

            if let Ok(val) =
                cap[1].parse::<f64>()
            {
                if val > 5000.0
                    && val < 10000.0
                {
                    candidates.push(val);
                }
            }
        }

        println!(
            "ALMASHHAD CANDIDATES => {:?}",
            candidates
        );

        return candidates.first().cloned();
    }

    //
    // X / TWITTER
    //
    if url.contains("x.com") {

        println!(
            "TWITTER SOURCE SKIPPED"
        );

        return None;
    }
    if url.contains("bankofkhartoum.com") {

    println!(
        "========== BOK HTML START =========="
    );

    println!(
        "{}",
        html.chars()
            .take(20000)
            .collect::<String>()
    );

    println!(
        "========== BOK HTML END =========="
    );

    return None;
}

    //
    // GENERIC FALLBACK
    //
    let re =
        Regex::new(
            r"([0-9]{4,5})"
        )
        .ok()?;

    let mut candidates =
        Vec::<f64>::new();

    for cap in re.captures_iter(&html) {

        if let Ok(val) =
            cap[1]
                .parse::<f64>()
        {
            if val > 5000.0
                && val < 10000.0
            {
                candidates.push(val);
            }
        }
    }

    println!(
        "FALLBACK CANDIDATES => {:?}",
        candidates
    );

    candidates.first().cloned()
}
