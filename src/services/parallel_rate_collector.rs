use crate::models::rate::SourceRate;

pub async fn collect_parallel_rates()
-> Vec<SourceRate> {

    vec![

        SourceRate {
            source_name:
                "ETCurrency".to_string(),

            confidence: 90.0,

            usd_rate: 2860.0,
        },

        SourceRate {
            source_name:
                "Jeeblay".to_string(),

            confidence: 88.0,

            usd_rate: 2875.0,
        },

        SourceRate {
            source_name:
                "SudanCurrencies".to_string(),

            confidence: 85.0,

            usd_rate: 2855.0,
        },

    ]
}