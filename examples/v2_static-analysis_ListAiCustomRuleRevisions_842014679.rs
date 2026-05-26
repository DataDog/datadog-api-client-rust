// List AI custom rule revisions returns "Successful response" response with
// pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_static_analysis::ListAiCustomRuleRevisionsOptionalParams;
use datadog_api_client::datadogV2::api_static_analysis::StaticAnalysisAPI;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListAiCustomRuleRevisions", true);
    let api = StaticAnalysisAPI::with_config(configuration);
    let response = api.list_ai_custom_rule_revisions_with_pagination(
        "my-ai-ruleset".to_string(),
        "my-ai-rule".to_string(),
        ListAiCustomRuleRevisionsOptionalParams::default(),
    );
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
