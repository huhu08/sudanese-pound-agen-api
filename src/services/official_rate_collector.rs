use crate::models::rate::SourceRate;

pub async fn collect_official_rates()
-> Vec<SourceRate> {

    vec![

        SourceRate {
            source_name: "CBOS".to_string(),
            confidence: 100.0,
            usd_rate: 2450.0,
        },

        SourceRate {
            source_name: "Bank Of Khartoum".to_string(),
            confidence: 95.0,
            usd_rate: 2452.0,
        },

        SourceRate {
            source_name: "ONB".to_string(),
            confidence: 95.0,
            usd_rate: 2448.0,
        },

    ]
}