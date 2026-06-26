// Get the governance console configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_governance_settings::GovernanceSettingsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetGovernanceConfig", true);
    let api = GovernanceSettingsAPI::with_config(configuration);
    let resp = api.get_governance_config().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
