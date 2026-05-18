// Reject a flag suggestion returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_feature_flags::FeatureFlagsAPI;
use datadog_api_client::datadogV2::model::FlagSuggestionEventDataType;
use datadog_api_client::datadogV2::model::ReviewFlagSuggestionAttributes;
use datadog_api_client::datadogV2::model::ReviewFlagSuggestionData;
use datadog_api_client::datadogV2::model::ReviewFlagSuggestionRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = ReviewFlagSuggestionRequest::new(
        ReviewFlagSuggestionData::new(FlagSuggestionEventDataType::FLAG_SUGGESTION_EVENTS)
            .attributes(
                ReviewFlagSuggestionAttributes::new().comment("Looks good, approved!".to_string()),
            ),
    );
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
