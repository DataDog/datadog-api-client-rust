// Create an environment returns "Created" response
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
            "Test Environment Example-Feature-Flag".to_string(),
            vec![
                "test-Example-Feature-Flag".to_string(),
                "env-Example-Feature-Flag".to_string(),
            ],
        ),
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
