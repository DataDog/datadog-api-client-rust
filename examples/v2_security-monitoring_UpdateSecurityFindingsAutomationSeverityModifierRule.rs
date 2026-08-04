// Update a severity modifier rule returns "Successfully updated the severity
// modifier rule" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::AutomationRuleScope;
use datadog_api_client::datadogV2::model::SecurityFindingType;
use datadog_api_client::datadogV2::model::SeverityModifierRuleAction;
use datadog_api_client::datadogV2::model::SeverityModifierRuleAttributesCreate;
use datadog_api_client::datadogV2::model::SeverityModifierRuleDataCreate;
use datadog_api_client::datadogV2::model::SeverityModifierRuleSetAction;
use datadog_api_client::datadogV2::model::SeverityModifierRuleSetActionType;
use datadog_api_client::datadogV2::model::SeverityModifierRuleType;
use datadog_api_client::datadogV2::model::SeverityModifierRuleUpdateRequest;
use datadog_api_client::datadogV2::model::SeverityModifierSeverity;

#[tokio::main]
async fn main() {
    // there is a valid "valid_severity_modifier_rule" in the system
    let valid_severity_modifier_rule_data_id =
        uuid::Uuid::parse_str(&std::env::var("VALID_SEVERITY_MODIFIER_RULE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = SeverityModifierRuleUpdateRequest::new(SeverityModifierRuleDataCreate::new(
        SeverityModifierRuleAttributesCreate::new(
            SeverityModifierRuleAction::SeverityModifierRuleSetAction(Box::new(
                SeverityModifierRuleSetAction::new(
                    SeverityModifierSeverity::LOW,
                    SeverityModifierRuleSetActionType::SET,
                )
                .description("Lower severity for dev environment noise".to_string()),
            )),
            "Downgrade misconfigurations in dev".to_string(),
            AutomationRuleScope::new(vec![SecurityFindingType::MISCONFIGURATION])
                .query("env:prod team:platform".to_string()),
        )
        .enabled(true),
        SeverityModifierRuleType::SEVERITY_MODIFIER_RULES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.UpdateSecurityFindingsAutomationSeverityModifierRule",
        true,
    );
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .update_security_findings_automation_severity_modifier_rule(
            valid_severity_modifier_rule_data_id.clone(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
