// Update a mute rule returns "Mute rule successfully updated" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::ActionMute;
use datadog_api_client::datadogV2::model::CreateMuteRuleParametersDataAttributes;
use datadog_api_client::datadogV2::model::IssueType;
use datadog_api_client::datadogV2::model::MuteReason;
use datadog_api_client::datadogV2::model::MuteRulesType;
use datadog_api_client::datadogV2::model::Rule;
use datadog_api_client::datadogV2::model::RuleSeverity;
use datadog_api_client::datadogV2::model::RuleTypesItems;
use datadog_api_client::datadogV2::model::UpdateMuteRuleParameters;
use datadog_api_client::datadogV2::model::UpdateMuteRuleParametersData;

#[tokio::main]
async fn main() {
    // there is a valid "valid_mute_rule" in the system
    let valid_mute_rule_data_id =
        uuid::Uuid::parse_str(&std::env::var("VALID_MUTE_RULE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = UpdateMuteRuleParameters::new().data(UpdateMuteRuleParametersData::new(
        CreateMuteRuleParametersDataAttributes::new(
            ActionMute::new(MuteReason::DUPLICATE)
                .expire_at(1893452400000)
                .reason_description("Muting for a while".to_string()),
            "Rule 1".to_string(),
            Rule::new(
                IssueType::VULNERABILITY,
                vec![RuleTypesItems::APPLICATION_CODE_VULNERABILITY],
            )
            .query("key:val".to_string())
            .rule_ids(vec!["rule-id-1".to_string()])
            .severities(vec![RuleSeverity::CRITICAL]),
        )
        .enabled(true),
        valid_mute_rule_data_id.clone(),
        MuteRulesType::MUTE_RULES,
    ));
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .update_mute_rule(valid_mute_rule_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
