// Update a monitor notification rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_monitors::MonitorsAPI;
use datadog_api_client::datadogV2::model::MonitorNotificationRuleAttributes;
use datadog_api_client::datadogV2::model::MonitorNotificationRuleFilter;
use datadog_api_client::datadogV2::model::MonitorNotificationRuleFilterTags;
use datadog_api_client::datadogV2::model::MonitorNotificationRuleResourceType;
use datadog_api_client::datadogV2::model::MonitorNotificationRuleUpdateRequest;
use datadog_api_client::datadogV2::model::MonitorNotificationRuleUpdateRequestData;

#[tokio::main]
async fn main() {
    // there is a valid "monitor_notification_rule" in the system
    let monitor_notification_rule_data_id =
        std::env::var("MONITOR_NOTIFICATION_RULE_DATA_ID").unwrap();
    let body = MonitorNotificationRuleUpdateRequest::new(
        MonitorNotificationRuleUpdateRequestData::new(
            MonitorNotificationRuleAttributes::new(
                "updated rule".to_string(),
                vec!["slack-test-channel".to_string()],
            )
            .filter(
                MonitorNotificationRuleFilter::MonitorNotificationRuleFilterTags(Box::new(
                    MonitorNotificationRuleFilterTags::new(vec![
                        "test:example-monitor".to_string(),
                        "host:abc".to_string(),
                    ]),
                )),
            ),
            monitor_notification_rule_data_id.clone(),
        )
        .type_(MonitorNotificationRuleResourceType::MONITOR_NOTIFICATION_RULE),
    );
    let configuration = datadog::Configuration::new();
    let api = MonitorsAPI::with_config(configuration);
    let resp = api
        .update_monitor_notification_rule(monitor_notification_rule_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
