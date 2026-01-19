// Create Jira issue template returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_jira_integration::JiraIntegrationAPI;
use datadog_api_client::datadogV2::model::JiraIssueTemplateCreateRequest;
use datadog_api_client::datadogV2::model::JiraIssueTemplateCreateRequestAttributes;
use datadog_api_client::datadogV2::model::JiraIssueTemplateCreateRequestAttributesJiraAccount;
use datadog_api_client::datadogV2::model::JiraIssueTemplateCreateRequestData;
use datadog_api_client::datadogV2::model::JiraIssueTemplateType;
use std::collections::BTreeMap;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = JiraIssueTemplateCreateRequest::new().data(
        JiraIssueTemplateCreateRequestData::new()
            .attributes(
                JiraIssueTemplateCreateRequestAttributes::new()
                    .fields(BTreeMap::from([]))
                    .issue_type_id("12730".to_string())
                    .jira_account(JiraIssueTemplateCreateRequestAttributesJiraAccount::new(
                        Uuid::parse_str("80f16d40-1fba-486e-b1fc-983e6ca19bec")
                            .expect("invalid UUID"),
                    ))
                    .name("test-template".to_string())
                    .project_id("10772".to_string()),
            )
            .type_(JiraIssueTemplateType::JIRA_ISSUE_TEMPLATE),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateJiraIssueTemplate", true);
    let api = JiraIntegrationAPI::with_config(configuration);
    let resp = api.create_jira_issue_template(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
