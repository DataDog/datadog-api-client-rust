// Delete Custom Ruleset returns "Successfully deleted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_static_analysis::StaticAnalysisAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteCustomRuleset", true);
    let api = StaticAnalysisAPI::with_config(configuration);
    let resp = api.delete_custom_ruleset("ruleset_name".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
