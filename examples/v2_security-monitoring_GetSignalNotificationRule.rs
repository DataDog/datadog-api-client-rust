// Get details of a signal-based rule returns "Notification rule details." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;

#[tokio::main]
async fn main() {
    // there is a valid "valid_signal_notification_rule" in the system
    let valid_signal_notification_rule_data_id =
        std::env::var("VALID_SIGNAL_NOTIFICATION_RULE_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .get_signal_notification_rule(valid_signal_notification_rule_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
