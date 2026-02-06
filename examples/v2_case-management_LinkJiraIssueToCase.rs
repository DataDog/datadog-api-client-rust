// Link existing Jira issue to case returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::JiraIssueLinkAttributes;
use datadog_api_client::datadogV2::model::JiraIssueLinkData;
use datadog_api_client::datadogV2::model::JiraIssueLinkRequest;
use datadog_api_client::datadogV2::model::JiraIssueResourceType;

#[tokio::main]
async fn main() {
    let body = JiraIssueLinkRequest::new(JiraIssueLinkData::new(
        JiraIssueLinkAttributes::new("https://jira.example.com/browse/PROJ-123".to_string()),
        JiraIssueResourceType::ISSUES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.LinkJiraIssueToCase", true);
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api
        .link_jira_issue_to_case("case_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
