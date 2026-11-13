// Create a flag change suggestion returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_feature_flags::FeatureFlagsAPI;
use datadog_api_client::datadogV2::model::CreateFlagSuggestionAttributes;
use datadog_api_client::datadogV2::model::CreateFlagSuggestionData;
use datadog_api_client::datadogV2::model::CreateFlagSuggestionRequest;
use datadog_api_client::datadogV2::model::FeatureFlagSuggestionDataType;
use datadog_api_client::datadogV2::model::FeatureFlagSuggestionMetadataRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = CreateFlagSuggestionRequest::new(CreateFlagSuggestionData::new(
        CreateFlagSuggestionAttributes::new(
            "updated".to_string(),
            vec!["requester@example.com".to_string()],
            "DISTRIBUTION_CHANNEL".to_string(),
        )
        .comment("Restrict this flag to server applications.".to_string())
        .environment_id(
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").expect("invalid UUID"),
        )
        .suggestion("SERVER".to_string())
        .suggestion_metadata(FeatureFlagSuggestionMetadataRequest::new().variant_id(
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440002").expect("invalid UUID"),
        )),
        FeatureFlagSuggestionDataType::FLAG_SUGGESTIONS,
    ));
    let configuration = datadog::Configuration::new();
    let api = FeatureFlagsAPI::with_config(configuration);
    let resp = api
        .create_flag_suggestion(
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
