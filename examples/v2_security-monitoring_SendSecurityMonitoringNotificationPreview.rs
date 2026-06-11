// Test a notification rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::CreateNotificationRuleParameters;
use datadog_api_client::datadogV2::model::CreateNotificationRuleParametersData;
use datadog_api_client::datadogV2::model::CreateNotificationRuleParametersDataAttributes;
use datadog_api_client::datadogV2::model::NotificationRulesType;
use datadog_api_client::datadogV2::model::RuleSeverity;
use datadog_api_client::datadogV2::model::RuleTypesItems;
use datadog_api_client::datadogV2::model::Selectors;
use datadog_api_client::datadogV2::model::TriggerSource;

#[tokio::main]
async fn main() {
    let body =
        CreateNotificationRuleParameters::new().data(CreateNotificationRuleParametersData::new(
            CreateNotificationRuleParametersDataAttributes::new(
                "Rule 1".to_string(),
                Selectors::new(TriggerSource::SECURITY_SIGNALS)
                    .query("env:prod".to_string())
                    .rule_types(vec![RuleTypesItems::LOG_DETECTION])
                    .severities(vec![RuleSeverity::CRITICAL]),
                vec!["@john.doe@email.com".to_string()],
            )
            .enabled(true),
            NotificationRulesType::NOTIFICATION_RULES,
        ));
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .send_security_monitoring_notification_preview(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
