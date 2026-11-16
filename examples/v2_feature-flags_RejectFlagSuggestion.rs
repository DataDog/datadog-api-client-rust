// Reject a flag change suggestion returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_feature_flags::FeatureFlagsAPI;
use datadog_api_client::datadogV2::model::RejectFlagSuggestionAttributes;
use datadog_api_client::datadogV2::model::RejectFlagSuggestionData;
use datadog_api_client::datadogV2::model::RejectFlagSuggestionRequest;
use datadog_api_client::datadogV2::model::ReviewFlagSuggestionDataType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = RejectFlagSuggestionRequest::new(RejectFlagSuggestionData::new(
        RejectFlagSuggestionAttributes::new()
            .comment("Keep the existing configuration.".to_string()),
        ReviewFlagSuggestionDataType::FLAG_SUGGESTION_EVENTS,
    ));
    let configuration = datadog::Configuration::new();
    let api = FeatureFlagsAPI::with_config(configuration);
    let resp = api
        .reject_flag_suggestion(
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
