// Create Jira issue for case returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::JiraIssueCreateAttributes;
use datadog_api_client::datadogV2::model::JiraIssueCreateData;
use datadog_api_client::datadogV2::model::JiraIssueCreateRequest;
use datadog_api_client::datadogV2::model::JiraIssueResourceType;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = JiraIssueCreateRequest::new(JiraIssueCreateData::new(
        JiraIssueCreateAttributes::new("10001".to_string(), "1234".to_string(), "5678".to_string())
            .fields(BTreeMap::from([])),
        JiraIssueResourceType::ISSUES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateCaseJiraIssue", true);
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api
        .create_case_jira_issue("case_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
