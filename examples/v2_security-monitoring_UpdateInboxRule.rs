// Update an inbox rule returns "Inbox rule successfully updated" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::ActionInbox;
use datadog_api_client::datadogV2::model::AutomationRule;
use datadog_api_client::datadogV2::model::CreateInboxRuleParametersDataAttributes;
use datadog_api_client::datadogV2::model::InboxRulesType;
use datadog_api_client::datadogV2::model::IssueType;
use datadog_api_client::datadogV2::model::SecurityRuleSeverity;
use datadog_api_client::datadogV2::model::SecurityRuleTypesItems;
use datadog_api_client::datadogV2::model::UpdateInboxRuleParameters;
use datadog_api_client::datadogV2::model::UpdateInboxRuleParametersData;

#[tokio::main]
async fn main() {
    // there is a valid "valid_inbox_rule" in the system
    let valid_inbox_rule_data_id =
        uuid::Uuid::parse_str(&std::env::var("VALID_INBOX_RULE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = UpdateInboxRuleParameters::new().data(UpdateInboxRuleParametersData::new(
        CreateInboxRuleParametersDataAttributes::new(
            ActionInbox::new().reason_description("We want to focus on these items.".to_string()),
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
        valid_inbox_rule_data_id.clone(),
        InboxRulesType::INBOX_RULES,
    ));
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .update_inbox_rule(valid_inbox_rule_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
