// List Custom Rule Revisions returns "Successful response" response with
// pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_static_analysis::ListCustomRuleRevisionsOptionalParams;
use datadog_api_client::datadogV2::api_static_analysis::StaticAnalysisAPI;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListCustomRuleRevisions", true);
    let api = StaticAnalysisAPI::with_config(configuration);
    let response = api.list_custom_rule_revisions_with_pagination(
        "ruleset_name".to_string(),
        "rule_name".to_string(),
        ListCustomRuleRevisionsOptionalParams::default(),
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
