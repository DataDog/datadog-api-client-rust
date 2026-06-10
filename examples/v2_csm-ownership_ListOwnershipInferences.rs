// List ownership inferences for a resource returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_csm_ownership::CSMOwnershipAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListOwnershipInferences", true);
    let api = CSMOwnershipAPI::with_config(configuration);
    let resp = api
        .list_ownership_inferences("test-resource".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
