use reqwest::Client;
use serde_json::{
    json,
    Value
};

pub async fn ask_deepseek(
    question: String,
    market_rate: f64,
    confidence: f64,
    summary: String,
) -> Result<String, Box<dyn std::error::Error>> {

    let api_key =
        std::env::var("DEEPSEEK_API_KEY")?;

    println!(
        "KEY LOADED = {}",
        !api_key.is_empty()
    );

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
                    "أنت محلل متخصص في سوق صرف الدولار مقابل الجنيه السوداني.

اعتمد على بيانات السوق الحالية فقط.

أعط إجابات عملية ومختصرة.

لا تطلب معلومات إضافية إلا إذا كانت ضرورية جداً.

عندما يسأل المستخدم:
هل الوقت مناسب للشراء؟

قم بتحليل وضع السوق مباشرة."
                },
                {
                    "role": "user",
                    "content": format!(
                        "
السعر المرجعي:
{} جنيه سوداني للدولار

درجة الثقة:
{}%

ملخص السوق:
{}

السؤال:
{}
                        ",
                        market_rate,
                        confidence,
                        summary,
                        question
                    )
                }
            ],
            "temperature": 0.7,
            "max_tokens": 800
        }))
        .send()
        .await?;

    println!(
        "STATUS => {}",
        response.status()
    );

    let body =
        response.text().await?;

    println!(
        "BODY => {}",
        body
    );

    let value: Value =
        serde_json::from_str(&body)?;

    let answer = value["choices"][0]
        ["message"]["content"]
        .as_str()
        .unwrap_or(
            "تعذر الحصول على رد من DeepSeek"
        )
        .to_string();

    Ok(answer)
}