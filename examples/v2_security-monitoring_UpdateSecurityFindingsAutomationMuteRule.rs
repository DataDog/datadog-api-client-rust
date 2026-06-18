// Update a mute rule returns "Successfully updated the mute rule" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::AutomationRuleScope;
use datadog_api_client::datadogV2::model::MuteReason;
use datadog_api_client::datadogV2::model::MuteRuleAction;
use datadog_api_client::datadogV2::model::MuteRuleAttributesCreate;
use datadog_api_client::datadogV2::model::MuteRuleDataCreate;
use datadog_api_client::datadogV2::model::MuteRuleType;
use datadog_api_client::datadogV2::model::MuteRuleUpdateRequest;
use datadog_api_client::datadogV2::model::SecurityFindingType;

#[tokio::main]
async fn main() {
    // there is a valid "valid_mute_rule" in the system
    let valid_mute_rule_data_id =
        uuid::Uuid::parse_str(&std::env::var("VALID_MUTE_RULE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = MuteRuleUpdateRequest::new(MuteRuleDataCreate::new(
        MuteRuleAttributesCreate::new(
            MuteRuleAction::new(MuteReason::FALSE_POSITIVE),
            "Example-Security-Monitoring".to_string(),
            AutomationRuleScope::new(vec![SecurityFindingType::MISCONFIGURATION])
                .query("env:staging".to_string()),
        )
        .enabled(false),
        MuteRuleType::MUTE_RULES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.UpdateSecurityFindingsAutomationMuteRule", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .update_security_findings_automation_mute_rule(valid_mute_rule_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
