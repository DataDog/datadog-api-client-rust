// Update GCP scan options returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_agentless_scanning::AgentlessScanningAPI;
use datadog_api_client::datadogV2::model::GcpScanOptionsInputUpdate;
use datadog_api_client::datadogV2::model::GcpScanOptionsInputUpdateData;
use datadog_api_client::datadogV2::model::GcpScanOptionsInputUpdateDataAttributes;
use datadog_api_client::datadogV2::model::GcpScanOptionsInputUpdateDataType;

#[tokio::main]
async fn main() {
    let body = GcpScanOptionsInputUpdate::new().data(
        GcpScanOptionsInputUpdateData::new(
            "api-spec-test".to_string(),
            GcpScanOptionsInputUpdateDataType::GCP_SCAN_OPTIONS,
        )
        .attributes(GcpScanOptionsInputUpdateDataAttributes::new().vuln_containers_os(false)),
    );
    let configuration = datadog::Configuration::new();
    let api = AgentlessScanningAPI::with_config(configuration);
    let resp = api
        .update_gcp_scan_options("api-spec-test".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
