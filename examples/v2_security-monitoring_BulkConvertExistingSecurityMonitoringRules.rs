// Bulk convert rules to Terraform returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleConvertBulkAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleConvertBulkData;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleConvertBulkDataType;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleConvertBulkPayload;

#[tokio::main]
async fn main() {
    let body = SecurityMonitoringRuleConvertBulkPayload::new(
        SecurityMonitoringRuleConvertBulkData::new(
            SecurityMonitoringRuleConvertBulkAttributes::new(vec![
                "def-000-u7q".to_string(),
                "def-000-7dd".to_string(),
            ]),
            SecurityMonitoringRuleConvertBulkDataType::SECURITY_MONITORING_RULES_CONVERT_BULK,
        )
        .id("convert_bulk".to_string()),
    );
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .bulk_convert_existing_security_monitoring_rules(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
