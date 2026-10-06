use reqwest::Client;
use serde_json::json;

pub async fn ask_deepseek(
    question: String,
    market_rate: f64,
    confidence: f64,
    summary: String,
) -> Result<String, Box<dyn std::error::Error>> {

    let api_key =
        std::env::var("DEEPSEEK_API_KEY")?;

    let client = Client::new();

    let response = client
        .post("https://api.deepseek.com/chat/completions")
        .bearer_auth(api_key)
        .json(&json!({
            "model": "deepseek-chat",
            "messages": [
                {
                    "role": "system",
                    "content":
                    "أنت محلل سوق العملات السوداني. قدم تحليلاً موجزاً ومفيداً."
                },
                {
                    "role": "user",
                    "content": format!(
                        "
السعر المرجعي: {}

درجة الثقة: {}%

ملخص السوق:
{}

سؤال المستخدم:
{}
                        ",
                        market_rate,
                        confidence,
                        summary,
                        question
                    )
                }
            ],
            "temperature": 0.7
        }))
        .send()
        .await?;

    let value: serde_json::Value =
        response.json().await?;

    let answer = value["choices"][0]
        ["message"]["content"]
        .as_str()
        .unwrap_or("تعذر إنشاء التحليل")
        .to_string();

    Ok(answer)
}