// Create a new mute rule returns "Successfully created the mute rule" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::ActionMute;
use datadog_api_client::datadogV2::model::AutomationRule;
use datadog_api_client::datadogV2::model::CreateMuteRuleParameters;
use datadog_api_client::datadogV2::model::CreateMuteRuleParametersData;
use datadog_api_client::datadogV2::model::CreateMuteRuleParametersDataAttributes;
use datadog_api_client::datadogV2::model::IssueType;
use datadog_api_client::datadogV2::model::MuteReason;
use datadog_api_client::datadogV2::model::MuteRulesType;
use datadog_api_client::datadogV2::model::SecurityRuleSeverity;
use datadog_api_client::datadogV2::model::SecurityRuleTypesItems;

#[tokio::main]
async fn main() {
    let body = CreateMuteRuleParameters::new().data(CreateMuteRuleParametersData::new(
        CreateMuteRuleParametersDataAttributes::new(
            ActionMute::new(MuteReason::DUPLICATE)
                .expire_at(1893452400000)
                .reason_description("Muting for a while".to_string()),
            "Rule 1".to_string(),
            AutomationRule::new(
                IssueType::VULNERABILITY,
                vec![SecurityRuleTypesItems::APPLICATION_CODE_VULNERABILITY],
            )
            .query("key:val".to_string())
            .rule_ids(vec!["rule-id-1".to_string()])
            .severities(vec![SecurityRuleSeverity::CRITICAL]),
        )
        .enabled(true),
        MuteRulesType::MUTE_RULES,
    ));
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.create_mute_rule(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
