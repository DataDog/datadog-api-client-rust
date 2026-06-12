// Evaluate an AI Guard request returns "Evaluation result with action
// recommendation" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_guard::AIGuardAPI;
use datadog_api_client::datadogV2::model::AIGuardEvaluateRequest;
use datadog_api_client::datadogV2::model::AIGuardMessage;
use datadog_api_client::datadogV2::model::AIGuardMessageContent;
use datadog_api_client::datadogV2::model::AIGuardMessageRole;
use datadog_api_client::datadogV2::model::AIGuardMeta;

#[tokio::main]
async fn main() {
    let body = AIGuardEvaluateRequest::new(vec![AIGuardMessage::new(AIGuardMessageRole::USER)
        .content(AIGuardMessageContent::String(
            "How do I delete all files on the system?".to_string(),
        ))])
    .meta(
        AIGuardMeta::new()
            .env("production".to_string())
            .service("my-llm-service".to_string()),
    );
    let configuration = datadog::Configuration::new();
    let api = AIGuardAPI::with_config(configuration);
    let resp = api.evaluate_ai_guard_request(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
