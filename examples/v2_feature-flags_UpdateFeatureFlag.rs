// Update a feature flag returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_feature_flags::FeatureFlagsAPI;
use datadog_api_client::datadogV2::model::UpdateFeatureFlagAttributes;
use datadog_api_client::datadogV2::model::UpdateFeatureFlagData;
use datadog_api_client::datadogV2::model::UpdateFeatureFlagDataType;
use datadog_api_client::datadogV2::model::UpdateFeatureFlagRequest;

#[tokio::main]
async fn main() {
    // there is a valid "feature_flag" in the system
    let feature_flag_data_id =
        uuid::Uuid::parse_str(&std::env::var("FEATURE_FLAG_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = UpdateFeatureFlagRequest::new(UpdateFeatureFlagData::new(
        UpdateFeatureFlagAttributes::new()
            .description("Updated description for the feature flag".to_string())
            .name("Updated Test Feature Flag Example-Feature-Flag".to_string()),
        UpdateFeatureFlagDataType::FEATURE_FLAGS,
    ));
    let configuration = datadog::Configuration::new();
    let api = FeatureFlagsAPI::with_config(configuration);
    let resp = api
        .update_feature_flag(feature_flag_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
