// Update Jira issue template returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_jira_integration::JiraIntegrationAPI;
use datadog_api_client::datadogV2::model::JiraIssueTemplateType;
use datadog_api_client::datadogV2::model::JiraIssueTemplateUpdateRequest;
use datadog_api_client::datadogV2::model::JiraIssueTemplateUpdateRequestAttributes;
use datadog_api_client::datadogV2::model::JiraIssueTemplateUpdateRequestData;
use std::collections::BTreeMap;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = JiraIssueTemplateUpdateRequest::new(JiraIssueTemplateUpdateRequestData::new(
        JiraIssueTemplateUpdateRequestAttributes::new()
            .fields(BTreeMap::from([]))
            .name("test_template_updated".to_string()),
        JiraIssueTemplateType::JIRA_ISSUE_TEMPLATE,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateJiraIssueTemplate", true);
    let api = JiraIntegrationAPI::with_config(configuration);
    let resp = api
        .update_jira_issue_template(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
