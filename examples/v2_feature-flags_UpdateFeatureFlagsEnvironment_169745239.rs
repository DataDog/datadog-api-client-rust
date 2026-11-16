// Update an environment to enable full evaluation data returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_feature_flags::FeatureFlagsAPI;
use datadog_api_client::datadogV2::model::UpdateEnvironmentAttributes;
use datadog_api_client::datadogV2::model::UpdateEnvironmentData;
use datadog_api_client::datadogV2::model::UpdateEnvironmentDataType;
use datadog_api_client::datadogV2::model::UpdateEnvironmentRequest;

#[tokio::main]
async fn main() {
    // there is a valid "environment" in the system
    let environment_data_id = uuid::Uuid::parse_str(&std::env::var("ENVIRONMENT_DATA_ID").unwrap())
        .expect("Invalid UUID");
    let body = UpdateEnvironmentRequest::new(UpdateEnvironmentData::new(
        UpdateEnvironmentAttributes::new()
            .name("Updated Privacy Example-Feature-Flag".to_string())
            .observe_full_evaluation_data(true)
            .queries(vec!["updated-privacy-example-feature-flag".to_string()]),
        UpdateEnvironmentDataType::ENVIRONMENTS,
    ));
    let configuration = datadog::Configuration::new();
    let api = FeatureFlagsAPI::with_config(configuration);
    let resp = api
        .update_feature_flags_environment(environment_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
