// Get a segment returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_audience_management::RumAudienceManagementAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.get_product_analytics_segment", true);
    let api = RumAudienceManagementAPI::with_config(configuration);
    let resp = api
        .get_product_analytics_segment("8a1b2c3d-4e5f-6a7b-8c9d-0e1f2a3b4c5d".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
