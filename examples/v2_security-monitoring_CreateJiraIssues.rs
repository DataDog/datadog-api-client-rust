// Create Jira issues for security findings returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::CaseManagementProject;
use datadog_api_client::datadogV2::model::CaseManagementProjectData;
use datadog_api_client::datadogV2::model::CaseManagementProjectDataType;
use datadog_api_client::datadogV2::model::CasePriority;
use datadog_api_client::datadogV2::model::CreateJiraIssueRequestArray;
use datadog_api_client::datadogV2::model::CreateJiraIssueRequestData;
use datadog_api_client::datadogV2::model::CreateJiraIssueRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateJiraIssueRequestDataRelationships;
use datadog_api_client::datadogV2::model::FindingData;
use datadog_api_client::datadogV2::model::FindingDataType;
use datadog_api_client::datadogV2::model::Findings;
use datadog_api_client::datadogV2::model::JiraIssuesDataType;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = CreateJiraIssueRequestArray::new(vec![CreateJiraIssueRequestData::new(
        JiraIssuesDataType::JIRA_ISSUES,
    )
    .attributes(
        CreateJiraIssueRequestDataAttributes::new()
            .assignee_id("f315bdaf-9ee7-4808-a9c1-99c15bf0f4d0".to_string())
            .description("A description of the Jira issue.".to_string())
            .fields(BTreeMap::from([("key1".to_string(), Value::from("value"))]))
            .priority(CasePriority::NOT_DEFINED)
            .title("A title for the Jira issue.".to_string()),
    )
    .relationships(CreateJiraIssueRequestDataRelationships::new(
        Findings::new().data(vec![FindingData::new(
            "ZGVmLTAwcC1pZXJ-aS0wZjhjNjMyZDNmMzRlZTgzNw==".to_string(),
            FindingDataType::FINDINGS,
        )]),
        CaseManagementProject::new(CaseManagementProjectData::new(
            "aeadc05e-98a8-11ec-ac2c-da7ad0900001".to_string(),
            CaseManagementProjectDataType::PROJECTS,
        )),
    ))]);
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateJiraIssues", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.create_jira_issues(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
