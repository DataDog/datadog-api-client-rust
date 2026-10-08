// Send AI tool user activity returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_impact::AIImpactAPI;
use datadog_api_client::datadogV2::model::AIImpactUserActivityAttributes;
use datadog_api_client::datadogV2::model::AIImpactUserActivityData;
use datadog_api_client::datadogV2::model::AIImpactUserActivityRequest;
use datadog_api_client::datadogV2::model::AIImpactUserActivityType;

#[tokio::main]
async fn main() {
    let body = AIImpactUserActivityRequest::new(vec![AIImpactUserActivityData::new(
        AIImpactUserActivityAttributes::new(
            "2026-05-26".to_string(),
            true,
            vec!["Claude Code".to_string(), "Cursor".to_string()],
            "user@example.com".to_string(),
        )
        .models(vec!["claude-sonnet-4.5".to_string(), "gpt-5".to_string()]),
        AIImpactUserActivityType::AI_IMPACT_USER_ACTIVITY,
    )]);
    let configuration = datadog::Configuration::new();
    let api = AIImpactAPI::with_config(configuration);
    let resp = api.create_ai_impact_user_activity(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
