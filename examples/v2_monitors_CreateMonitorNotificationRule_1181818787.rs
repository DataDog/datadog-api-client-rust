// Create a monitor notification rule with conditional recipients returns "OK"
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_monitors::MonitorsAPI;
use datadog_api_client::datadogV2::model::MonitorNotificationRuleAttributes;
use datadog_api_client::datadogV2::model::MonitorNotificationRuleCondition;
use datadog_api_client::datadogV2::model::MonitorNotificationRuleConditionalRecipients;
use datadog_api_client::datadogV2::model::MonitorNotificationRuleCreateRequest;
use datadog_api_client::datadogV2::model::MonitorNotificationRuleCreateRequestData;
use datadog_api_client::datadogV2::model::MonitorNotificationRuleFilter;
use datadog_api_client::datadogV2::model::MonitorNotificationRuleFilterTags;
use datadog_api_client::datadogV2::model::MonitorNotificationRuleResourceType;

#[tokio::main]
async fn main() {
    let body = MonitorNotificationRuleCreateRequest::new(
        MonitorNotificationRuleCreateRequestData::new(
            MonitorNotificationRuleAttributes::new("test rule".to_string())
                .conditional_recipients(MonitorNotificationRuleConditionalRecipients::new(vec![
                    MonitorNotificationRuleCondition::new(
                        vec!["slack-test-channel".to_string(), "jira-test".to_string()],
                        "transition_type:is_alert".to_string(),
                    ),
                ]))
                .filter(
                    MonitorNotificationRuleFilter::MonitorNotificationRuleFilterTags(Box::new(
                        MonitorNotificationRuleFilterTags::new(vec![
                            "test:example-monitor".to_string()
                        ]),
                    )),
                ),
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
