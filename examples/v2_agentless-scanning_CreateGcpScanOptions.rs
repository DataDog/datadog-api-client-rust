// Post GCP Scan Options returns "Agentless scan options enabled successfully."
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_agentless_scanning::AgentlessScanningAPI;
use datadog_api_client::datadogV2::model::GcpScanOptions;
use datadog_api_client::datadogV2::model::GcpScanOptionsData;
use datadog_api_client::datadogV2::model::GcpScanOptionsDataAttributes;
use datadog_api_client::datadogV2::model::GcpScanOptionsDataType;

#[tokio::main]
async fn main() {
    let body = GcpScanOptions::new().data(
        GcpScanOptionsData::new(
            "new-project".to_string(),
            GcpScanOptionsDataType::GCP_SCAN_OPTIONS,
        )
        .attributes(
            GcpScanOptionsDataAttributes::new()
                .vuln_containers_os(true)
                .vuln_host_os(true),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = AgentlessScanningAPI::with_config(configuration);
    let resp = api.create_gcp_scan_options(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
