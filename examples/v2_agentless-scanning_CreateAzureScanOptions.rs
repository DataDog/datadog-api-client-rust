// Create azure scan options returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_agentless_scanning::AgentlessScanningAPI;
use datadog_api_client::datadogV2::model::AzureScanOptions;
use datadog_api_client::datadogV2::model::AzureScanOptionsData;
use datadog_api_client::datadogV2::model::AzureScanOptionsDataAttributes;
use datadog_api_client::datadogV2::model::AzureScanOptionsDataType;

#[tokio::main]
async fn main() {
    let body = AzureScanOptions::new().data(
        AzureScanOptionsData::new(
            "12345678-90ab-cdef-1234-567890abcdef".to_string(),
            AzureScanOptionsDataType::AZURE_SCAN_OPTIONS,
        )
        .attributes(
            AzureScanOptionsDataAttributes::new()
                .vuln_containers_os(true)
                .vuln_host_os(true),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = AgentlessScanningAPI::with_config(configuration);
    let resp = api.create_azure_scan_options(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
