// Get the rule-based view of compliance findings returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_compliance::ComplianceAPI;
use datadog_api_client::datadogV2::api_compliance::GetRuleBasedViewOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetRuleBasedView", true);
    let api = ComplianceAPI::with_config(configuration);
    let resp = api
        .get_rule_based_view(1739982278000, GetRuleBasedViewOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
