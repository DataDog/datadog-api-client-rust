// Ruleset get multiple returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::GetMultipleRulesetsRequest;
use datadog_api_client::datadogV2::model::GetMultipleRulesetsRequestData;
use datadog_api_client::datadogV2::model::GetMultipleRulesetsRequestDataAttributes;
use datadog_api_client::datadogV2::model::GetMultipleRulesetsRequestDataType;

#[tokio::main]
async fn main() {
    let body = GetMultipleRulesetsRequest::new().data(
        GetMultipleRulesetsRequestData::new(
            GetMultipleRulesetsRequestDataType::GET_MULTIPLE_RULESETS_REQUEST,
        )
        .attributes(GetMultipleRulesetsRequestDataAttributes::new().rulesets(vec![])),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListMultipleRulesets", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.list_multiple_rulesets(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
