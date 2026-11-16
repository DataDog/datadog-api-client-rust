// Create an environment with full evaluation data false returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_feature_flags::FeatureFlagsAPI;
use datadog_api_client::datadogV2::model::CreateEnvironmentAttributes;
use datadog_api_client::datadogV2::model::CreateEnvironmentData;
use datadog_api_client::datadogV2::model::CreateEnvironmentDataType;
use datadog_api_client::datadogV2::model::CreateEnvironmentRequest;

#[tokio::main]
async fn main() {
    let body = CreateEnvironmentRequest::new(CreateEnvironmentData::new(
        CreateEnvironmentAttributes::new(
            "Evaluation Privacy Example-Feature-Flag".to_string(),
            vec!["privacy-example-feature-flag".to_string()],
        )
        .observe_full_evaluation_data(false),
        CreateEnvironmentDataType::ENVIRONMENTS,
    ));
    let configuration = datadog::Configuration::new();
    let api = FeatureFlagsAPI::with_config(configuration);
    let resp = api.create_feature_flags_environment(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
