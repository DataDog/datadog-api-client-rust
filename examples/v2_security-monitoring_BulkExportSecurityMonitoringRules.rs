// Bulk export security monitoring rules returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleBulkExportAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleBulkExportData;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleBulkExportDataType;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleBulkExportPayload;

#[tokio::main]
async fn main() {
    // there is a valid "security_rule" in the system
    let security_rule_id = std::env::var("SECURITY_RULE_ID").unwrap();
    let body =
        SecurityMonitoringRuleBulkExportPayload::new(SecurityMonitoringRuleBulkExportData::new(
            SecurityMonitoringRuleBulkExportAttributes::new(vec![security_rule_id.clone()]),
            SecurityMonitoringRuleBulkExportDataType::SECURITY_MONITORING_RULES_BULK_EXPORT,
        ));
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.bulk_export_security_monitoring_rules(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
