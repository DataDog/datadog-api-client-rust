// List insights returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_governance_console::GovernanceConsoleAPI;
use datadog_api_client::datadogV2::api_governance_console::ListGovernanceInsightsOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListGovernanceInsights", true);
    let api = GovernanceConsoleAPI::with_config(configuration);
    let resp = api
        .list_governance_insights(ListGovernanceInsightsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
