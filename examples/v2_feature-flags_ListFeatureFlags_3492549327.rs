// List feature flags excludes unknown saved filters
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_feature_flags::FeatureFlagsAPI;
use datadog_api_client::datadogV2::api_feature_flags::ListFeatureFlagsOptionalParams;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = FeatureFlagsAPI::with_config(configuration);
    let resp = api
        .list_feature_flags(
            ListFeatureFlagsOptionalParams::default()
                .tags(vec!["list-test:example-feature-flag".to_string()])
                .saved_filter_id(
                    Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
                ),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
