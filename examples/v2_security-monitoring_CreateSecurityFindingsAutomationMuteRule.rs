// Create a mute rule returns "Successfully created the mute rule" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::AutomationRuleScope;
use datadog_api_client::datadogV2::model::MuteReason;
use datadog_api_client::datadogV2::model::MuteRuleAction;
use datadog_api_client::datadogV2::model::MuteRuleAttributesCreate;
use datadog_api_client::datadogV2::model::MuteRuleCreateRequest;
use datadog_api_client::datadogV2::model::MuteRuleDataCreate;
use datadog_api_client::datadogV2::model::MuteRuleType;
use datadog_api_client::datadogV2::model::SecurityFindingType;

#[tokio::main]
async fn main() {
    let body = MuteRuleCreateRequest::new(MuteRuleDataCreate::new(
        MuteRuleAttributesCreate::new(
            MuteRuleAction::new(MuteReason::RISK_ACCEPTED),
            "Example-Security-Monitoring".to_string(),
            AutomationRuleScope::new(vec![SecurityFindingType::MISCONFIGURATION])
                .query("env:staging".to_string()),
        )
        .enabled(true),
        MuteRuleType::MUTE_RULES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.CreateSecurityFindingsAutomationMuteRule", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .create_security_findings_automation_mute_rule(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
