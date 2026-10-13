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
                    "content": r#"
أنت وكيل ذكاء اصطناعي متخصص حصرياً في تحليل
سوق صرف الجنيه السوداني مقابل الدولار.

مهم جداً:

لا تعتبر السعر المرجعي هو السعر الحالي المؤكد.

السعر المرجعي هو قيمة تحليلية مستخرجة من عدة مصادر.

اعتمد على:
- السعر المرجعي
- درجة الثقة
- اتجاه آخر 7 أيام
- ملخص السوق

عند الإجابة:
1. قدم ملخصاً سريعاً للسوق.
2. وضح المخاطر.
3. وضح إن كانت البيانات تدعم الشراء أو الانتظار.
4. لا تطلب معلومات إضافية إذا كانت البيانات المتاحة كافية.
5. اجعل الإجابة قصيرة وعملية.
"#
                },
                {
                    "role": "user",
                    "content": format!(
r#"
السعر المرجعي التحليلي:
{} جنيه سوداني للدولار

هذا السعر تقديري مستخرج من عدة مصادر
وليس سعراً رسمياً أو فورياً للسوق.

درجة الثقة:
{}%

ملخص السوق:
{}

السؤال:
{}
"#,
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