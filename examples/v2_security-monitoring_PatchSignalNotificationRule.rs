// Patch a signal-based rule returns "Notification rule successfully patched."
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::NotificationRulesType;
use datadog_api_client::datadogV2::model::PatchNotificationRuleParameters;
use datadog_api_client::datadogV2::model::PatchNotificationRuleParametersData;
use datadog_api_client::datadogV2::model::PatchNotificationRuleParametersDataAttributes;
use datadog_api_client::datadogV2::model::RuleSeverity;
use datadog_api_client::datadogV2::model::RuleTypesItems;
use datadog_api_client::datadogV2::model::Selectors;
use datadog_api_client::datadogV2::model::TriggerSource;

#[tokio::main]
async fn main() {
    // there is a valid "valid_signal_notification_rule" in the system
    let valid_signal_notification_rule_data_id =
        std::env::var("VALID_SIGNAL_NOTIFICATION_RULE_DATA_ID").unwrap();
    let body =
        PatchNotificationRuleParameters::new().data(PatchNotificationRuleParametersData::new(
            PatchNotificationRuleParametersDataAttributes::new()
                .enabled(true)
                .name("Rule 1".to_string())
                .selectors(
                    Selectors::new(TriggerSource::SECURITY_FINDINGS)
                        .query("(source:production_service OR env:prod)".to_string())
                        .rule_types(vec![
                            RuleTypesItems::MISCONFIGURATION,
                            RuleTypesItems::ATTACK_PATH,
                        ])
                        .severities(vec![RuleSeverity::CRITICAL]),
                )
                .targets(vec!["@john.doe@email.com".to_string()])
                .time_aggregation(86400)
                .version(1),
            valid_signal_notification_rule_data_id.clone(),
            NotificationRulesType::NOTIFICATION_RULES,
        ));
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .patch_signal_notification_rule(valid_signal_notification_rule_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
