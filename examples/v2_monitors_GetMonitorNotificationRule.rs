// Get a monitor notification rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_monitors::GetMonitorNotificationRuleOptionalParams;
use datadog_api_client::datadogV2::api_monitors::MonitorsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "monitor_notification_rule" in the system
    let monitor_notification_rule_data_id =
        std::env::var("MONITOR_NOTIFICATION_RULE_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetMonitorNotificationRule", true);
    let api = MonitorsAPI::with_config(configuration);
    let resp = api
        .get_monitor_notification_rule(
            monitor_notification_rule_data_id.clone(),
            GetMonitorNotificationRuleOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
