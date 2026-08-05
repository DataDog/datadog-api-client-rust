// List control detections returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_governance_console::GovernanceConsoleAPI;
use datadog_api_client::datadogV2::api_governance_console::ListGovernanceControlDetectionsOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListGovernanceControlDetections", true);
    let api = GovernanceConsoleAPI::with_config(configuration);
    let resp = api
        .list_governance_control_detections(
            "detection_type".to_string(),
            ListGovernanceControlDetectionsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
