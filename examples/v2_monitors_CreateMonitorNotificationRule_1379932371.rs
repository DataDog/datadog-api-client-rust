// Create a monitor notification rule with scope returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_monitors::MonitorsAPI;
use datadog_api_client::datadogV2::model::MonitorNotificationRuleAttributes;
use datadog_api_client::datadogV2::model::MonitorNotificationRuleCreateRequest;
use datadog_api_client::datadogV2::model::MonitorNotificationRuleCreateRequestData;
use datadog_api_client::datadogV2::model::MonitorNotificationRuleFilter;
use datadog_api_client::datadogV2::model::MonitorNotificationRuleFilterScope;
use datadog_api_client::datadogV2::model::MonitorNotificationRuleResourceType;

#[tokio::main]
async fn main() {
    let body = MonitorNotificationRuleCreateRequest::new(
        MonitorNotificationRuleCreateRequestData::new(
            MonitorNotificationRuleAttributes::new("test rule".to_string())
                .filter(
                    MonitorNotificationRuleFilter::MonitorNotificationRuleFilterScope(Box::new(
                        MonitorNotificationRuleFilterScope::new("test:example-monitor".to_string()),
                    )),
                )
                .recipients(vec![
                    "slack-test-channel".to_string(),
                    "jira-test".to_string(),
                ]),
        )
        .type_(MonitorNotificationRuleResourceType::MONITOR_NOTIFICATION_RULE),
    );
    let configuration = datadog::Configuration::new();
    let api = MonitorsAPI::with_config(configuration);
    let resp = api.create_monitor_notification_rule(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
