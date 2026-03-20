// Get an environment returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_feature_flags::FeatureFlagsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "environment" in the system
    let environment_data_id = uuid::Uuid::parse_str(&std::env::var("ENVIRONMENT_DATA_ID").unwrap())
        .expect("Invalid UUID");
    let configuration = datadog::Configuration::new();
    let api = FeatureFlagsAPI::with_config(configuration);
    let resp = api
        .get_feature_flags_environment(environment_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
