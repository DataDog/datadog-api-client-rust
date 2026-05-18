// Create a flag suggestion returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_feature_flags::FeatureFlagsAPI;
use datadog_api_client::datadogV2::model::CreateFlagSuggestionAttributes;
use datadog_api_client::datadogV2::model::CreateFlagSuggestionData;
use datadog_api_client::datadogV2::model::CreateFlagSuggestionRequest;
use datadog_api_client::datadogV2::model::FlagSuggestionAction;
use datadog_api_client::datadogV2::model::FlagSuggestionDataType;
use datadog_api_client::datadogV2::model::FlagSuggestionProperty;
use datadog_api_client::datadogV2::model::SuggestionMetadata;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = CreateFlagSuggestionRequest::new(CreateFlagSuggestionData::new(
        CreateFlagSuggestionAttributes::new(
            FlagSuggestionAction::ARCHIVED,
            vec!["user@example.com".to_string()],
            FlagSuggestionProperty::FLAG,
        )
        .comment("Archive this deprecated flag".to_string())
        .environment_id(
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").expect("invalid UUID"),
        )
        .suggestion("ENABLED".to_string())
        .suggestion_metadata(
            SuggestionMetadata::new()
                .variant_id("550e8400-e29b-41d4-a716-446655440005".to_string()),
        ),
        FlagSuggestionDataType::FLAG_SUGGESTIONS,
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
