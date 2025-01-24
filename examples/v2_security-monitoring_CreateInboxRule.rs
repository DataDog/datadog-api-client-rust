// Create a new inbox rule returns "Successfully created the inbox rule" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::ActionInbox;
use datadog_api_client::datadogV2::model::CreateInboxRuleParameters;
use datadog_api_client::datadogV2::model::CreateInboxRuleParametersData;
use datadog_api_client::datadogV2::model::CreateInboxRuleParametersDataAttributes;
use datadog_api_client::datadogV2::model::InboxRulesType;
use datadog_api_client::datadogV2::model::IssueType;
use datadog_api_client::datadogV2::model::Rule;
use datadog_api_client::datadogV2::model::RuleSeverity;
use datadog_api_client::datadogV2::model::RuleTypesItems;

#[tokio::main]
async fn main() {
    let body = CreateInboxRuleParameters::new().data(CreateInboxRuleParametersData::new(
        CreateInboxRuleParametersDataAttributes::new(
            ActionInbox::new().reason_description("We want to focus on these items.".to_string()),
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
        InboxRulesType::INBOX_RULES,
    ));
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.create_inbox_rule(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
