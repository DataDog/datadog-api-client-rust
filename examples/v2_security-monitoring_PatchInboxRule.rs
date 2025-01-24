// Patch an inbox rule returns "Inbox rule successfully patched" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::ActionInbox;
use datadog_api_client::datadogV2::model::InboxRulesType;
use datadog_api_client::datadogV2::model::IssueType;
use datadog_api_client::datadogV2::model::PatchInboxRulesParameters;
use datadog_api_client::datadogV2::model::PatchInboxRulesParametersData;
use datadog_api_client::datadogV2::model::PatchInboxRulesParametersDataAttributes;
use datadog_api_client::datadogV2::model::Rule;
use datadog_api_client::datadogV2::model::RuleSeverity;
use datadog_api_client::datadogV2::model::RuleTypesItems;

#[tokio::main]
async fn main() {
    // there is a valid "valid_inbox_rule" in the system
    let valid_inbox_rule_data_id =
        uuid::Uuid::parse_str(&std::env::var("VALID_INBOX_RULE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = PatchInboxRulesParameters::new().data(PatchInboxRulesParametersData::new(
        PatchInboxRulesParametersDataAttributes::new()
            .action(
                ActionInbox::new()
                    .reason_description("We want to focus on these items.".to_string()),
            )
            .enabled(true)
            .name("Rule 1".to_string())
            .rule(
                Rule::new(
                    IssueType::VULNERABILITY,
                    vec![RuleTypesItems::APPLICATION_CODE_VULNERABILITY],
                )
                .query("key:val".to_string())
                .rule_ids(vec!["rule-id-1".to_string()])
                .severities(vec![RuleSeverity::CRITICAL]),
            ),
        valid_inbox_rule_data_id.clone(),
        InboxRulesType::INBOX_RULES,
    ));
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .patch_inbox_rule(valid_inbox_rule_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
