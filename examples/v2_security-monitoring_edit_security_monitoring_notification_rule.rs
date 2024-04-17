// Update a notification rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringNotificationRuleSelectors;
use datadog_api_client::datadogV2::model::SecurityMonitoringNotificationRuleType;
use datadog_api_client::datadogV2::model::SecurityMonitoringNotificationRuleUpdateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringNotificationRuleUpdateData;
use datadog_api_client::datadogV2::model::SecurityMonitoringNotificationRuleUpdateRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleSeverity;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleTypes;

#[tokio::main]
async fn main() {
    // there is a valid "notification_rule" in the system
    let notification_rule_data_id = std::env::var("NOTIFICATION_RULE_DATA_ID").unwrap();
    let body = SecurityMonitoringNotificationRuleUpdateRequest::new(
        SecurityMonitoringNotificationRuleUpdateData::new(
            SecurityMonitoringNotificationRuleUpdateAttributes::new(
                false,
                "Example-Security-Monitoring".to_string(),
                SecurityMonitoringNotificationRuleSelectors::new(
                    vec!["fim:true".to_string()],
                    false,
                    vec!["fim:true".to_string()],
                    vec![
                        SecurityMonitoringRuleTypes::LOG_DETECTION,
                        SecurityMonitoringRuleTypes::CLOUD_CONFIGURATION,
                    ],
                    vec![SecurityMonitoringRuleSeverity::HIGH],
                    vec![],
                ),
                vec!["test2".to_string()],
                1,
            ),
            SecurityMonitoringNotificationRuleType::NOTIFICATION_RULES,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .edit_security_monitoring_notification_rule(notification_rule_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
