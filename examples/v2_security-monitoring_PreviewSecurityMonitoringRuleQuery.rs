// Preview a rule query with applied filters returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleDetectionMethod;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleLivetailRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleTypeRead;

#[tokio::main]
async fn main() {
    let body = SecurityMonitoringRuleLivetailRequest::new(
        "logs".to_string(),
        SecurityMonitoringRuleDetectionMethod::THRESHOLD,
        "source:cloudtrail".to_string(),
        0,
        SecurityMonitoringRuleTypeRead::LOG_DETECTION,
    )
    .distinct_fields(vec![])
    .filters(vec![])
    .group_by_fields(vec![]);
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.preview_security_monitoring_rule_query(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
