// Update a variant returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_feature_flags::FeatureFlagsAPI;
use datadog_api_client::datadogV2::model::UpdateVariantRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = UpdateVariantRequest::new()
        .name("Variant ABC123 Updated".to_string())
        .value("new_value".to_string());
    let configuration = datadog::Configuration::new();
    let api = FeatureFlagsAPI::with_config(configuration);
    let resp = api
        .update_variant_for_feature_flag(
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").expect("invalid UUID"),
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440002").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
