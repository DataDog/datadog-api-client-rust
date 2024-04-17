// Create a notification rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringNotificationRuleCreateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringNotificationRuleCreateData;
use datadog_api_client::datadogV2::model::SecurityMonitoringNotificationRuleCreateRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringNotificationRuleSelectors;
use datadog_api_client::datadogV2::model::SecurityMonitoringNotificationRuleType;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleSeverity;
use datadog_api_client::datadogV2::model::SecurityMonitoringRuleTypes;

#[tokio::main]
async fn main() {
    let body = SecurityMonitoringNotificationRuleCreateRequest::new(
        SecurityMonitoringNotificationRuleCreateData::new(
            SecurityMonitoringNotificationRuleCreateAttributes::new(
                true,
                "Example-Security-Monitoring".to_string(),
                SecurityMonitoringNotificationRuleSelectors::new(
                    vec!["test:123".to_string(), "env:prod".to_string()],
                    false,
                    vec!["test:123".to_string()],
                    vec![
                        SecurityMonitoringRuleTypes::APPLICATION_SECURITY,
                        SecurityMonitoringRuleTypes::LOG_DETECTION,
                    ],
                    vec![SecurityMonitoringRuleSeverity::HIGH],
                    vec!["env:prod".to_string()],
                ),
                vec!["@slack-test".to_string()],
            ),
            SecurityMonitoringNotificationRuleType::NOTIFICATION_RULES,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.create_security_monitoring_notification_rule(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
