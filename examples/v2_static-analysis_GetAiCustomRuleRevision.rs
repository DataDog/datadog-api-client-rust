// Get an AI custom rule revision returns "Successful response" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_static_analysis::StaticAnalysisAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetAiCustomRuleRevision", true);
    let api = StaticAnalysisAPI::with_config(configuration);
    let resp = api
        .get_ai_custom_rule_revision(
            "my-ai-ruleset".to_string(),
            "my-ai-rule".to_string(),
            "revision-abc-123".to_string(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
