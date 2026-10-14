// List feature flags combines teams with ordinary tags
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_feature_flags::FeatureFlagsAPI;
use datadog_api_client::datadogV2::api_feature_flags::ListFeatureFlagsOptionalParams;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = FeatureFlagsAPI::with_config(configuration);
    let resp = api
        .list_feature_flags(
            ListFeatureFlagsOptionalParams::default()
                .tags(vec![
                    "list-test:example-feature-flag".to_string(),
                    "color:blue".to_string(),
                ])
                .teams(vec![
                    "api-client-test-a".to_string(),
                    "api-client-test-b".to_string(),
                ]),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
