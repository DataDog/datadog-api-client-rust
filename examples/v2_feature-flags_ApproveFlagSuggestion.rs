// Approve a flag change suggestion returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_feature_flags::FeatureFlagsAPI;
use datadog_api_client::datadogV2::model::ApproveFlagSuggestionAttributes;
use datadog_api_client::datadogV2::model::ApproveFlagSuggestionData;
use datadog_api_client::datadogV2::model::ApproveFlagSuggestionRequest;
use datadog_api_client::datadogV2::model::ReviewFlagSuggestionDataType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = ApproveFlagSuggestionRequest::new(ApproveFlagSuggestionData::new(
        ApproveFlagSuggestionAttributes::new()
            .apply(false)
            .comment("Approved for rollout.".to_string()),
        ReviewFlagSuggestionDataType::FLAG_SUGGESTION_EVENTS,
    ));
    let configuration = datadog::Configuration::new();
    let api = FeatureFlagsAPI::with_config(configuration);
    let resp = api
        .approve_flag_suggestion(
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440020").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
