// Update Azure scan options returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_agentless_scanning::AgentlessScanningAPI;
use datadog_api_client::datadogV2::model::AzureScanOptionsInputUpdate;
use datadog_api_client::datadogV2::model::AzureScanOptionsInputUpdateData;
use datadog_api_client::datadogV2::model::AzureScanOptionsInputUpdateDataType;

#[tokio::main]
async fn main() {
    let body = AzureScanOptionsInputUpdate::new().data(AzureScanOptionsInputUpdateData::new(
        "12345678-90ab-cdef-1234-567890abcdef".to_string(),
        AzureScanOptionsInputUpdateDataType::AZURE_SCAN_OPTIONS,
    ));
    let configuration = datadog::Configuration::new();
    let api = AgentlessScanningAPI::with_config(configuration);
    let resp = api
        .update_azure_scan_options("12345678-90ab-cdef-1234-567890abcdef".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
