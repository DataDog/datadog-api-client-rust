// Bulk delete security monitoring rules returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleBulkDeleteAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleBulkDeleteData;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleBulkDeletePayload;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleBulkDeleteRequestDataType;

#[tokio::main]
async fn main() {
    let body =
        SecurityMonitoringRuleBulkDeletePayload::new(SecurityMonitoringRuleBulkDeleteData::new(
            SecurityMonitoringRuleBulkDeleteAttributes::new(vec![
                "abc-000-u7q".to_string(),
                "abc-000-7dd".to_string(),
            ]),
            SecurityMonitoringRuleBulkDeleteRequestDataType::BULK_DELETE_RULES,
        ));
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.bulk_delete_security_monitoring_rules(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
