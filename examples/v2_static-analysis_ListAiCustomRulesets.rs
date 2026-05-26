// List AI custom rulesets returns "Successful response" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_static_analysis::ListAiCustomRulesetsOptionalParams;
use datadog_api_client::datadogV2::api_static_analysis::StaticAnalysisAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListAiCustomRulesets", true);
    let api = StaticAnalysisAPI::with_config(configuration);
    let resp = api
        .list_ai_custom_rulesets(ListAiCustomRulesetsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
