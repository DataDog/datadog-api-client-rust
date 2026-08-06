// Get a control returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_governance_console::GovernanceConsoleAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetGovernanceControl", true);
    let api = GovernanceConsoleAPI::with_config(configuration);
    let resp = api
        .get_governance_control("detection_type".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
