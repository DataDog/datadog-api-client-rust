// Show Custom Rule Revision returns "Successful response" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_static_analysis::StaticAnalysisAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetCustomRuleRevision", true);
    let api = StaticAnalysisAPI::with_config(configuration);
    let resp = api
        .get_custom_rule_revision(
            "ruleset_name".to_string(),
            "rule_name".to_string(),
            "id".to_string(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
