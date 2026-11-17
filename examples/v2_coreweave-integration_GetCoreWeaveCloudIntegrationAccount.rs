// Get a CoreWeave integration account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_core_weave_integration::CoreWeaveIntegrationAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.get_core_weave_cloud_integration_account", true);
    let api = CoreWeaveIntegrationAPI::with_config(configuration);
    let resp = api
        .get_core_weave_cloud_integration_account("account_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
