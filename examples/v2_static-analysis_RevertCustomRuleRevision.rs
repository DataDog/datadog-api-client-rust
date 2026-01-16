// Revert Custom Rule Revision returns "Successfully reverted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_static_analysis::StaticAnalysisAPI;
use datadog_api_client::datadogV2::model::RevertCustomRuleRevisionDataType;
use datadog_api_client::datadogV2::model::RevertCustomRuleRevisionRequest;
use datadog_api_client::datadogV2::model::RevertCustomRuleRevisionRequestData;
use datadog_api_client::datadogV2::model::RevertCustomRuleRevisionRequestDataAttributes;

#[tokio::main]
async fn main() {
    let body = RevertCustomRuleRevisionRequest::new().data(
        RevertCustomRuleRevisionRequestData::new()
            .attributes(RevertCustomRuleRevisionRequestDataAttributes::new())
            .type_(RevertCustomRuleRevisionDataType::REVERT_CUSTOM_RULE_REVISION_REQUEST),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.RevertCustomRuleRevision", true);
    let api = StaticAnalysisAPI::with_config(configuration);
    let resp = api
        .revert_custom_rule_revision("ruleset_name".to_string(), "rule_name".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
