// Create Linear issues for security findings returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::CaseManagementProject;
use datadog_api_client::datadogV2::model::CaseManagementProjectData;
use datadog_api_client::datadogV2::model::CaseManagementProjectDataType;
use datadog_api_client::datadogV2::model::CasePriority;
use datadog_api_client::datadogV2::model::CreateLinearIssueRequestArray;
use datadog_api_client::datadogV2::model::CreateLinearIssueRequestData;
use datadog_api_client::datadogV2::model::CreateLinearIssueRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateLinearIssueRequestDataRelationships;
use datadog_api_client::datadogV2::model::FindingData;
use datadog_api_client::datadogV2::model::FindingDataType;
use datadog_api_client::datadogV2::model::Findings;
use datadog_api_client::datadogV2::model::LinearIssuesDataType;

#[tokio::main]
async fn main() {
    let body = CreateLinearIssueRequestArray::new(vec![CreateLinearIssueRequestData::new(
        LinearIssuesDataType::LINEAR_ISSUES,
    )
    .attributes(
        CreateLinearIssueRequestDataAttributes::new()
            .assignee_id("f315bdaf-9ee7-4808-a9c1-99c15bf0f4d0".to_string())
            .description("A description of the Linear issue.".to_string())
            .label_ids(vec!["a1b2c3d4-5e6f-7a8b-9c0d-1e2f3a4b5c6d".to_string()])
            .linear_project_id("d4c3b2a1-6f5e-8b7a-0d9c-2f1e4a3b6c5d".to_string())
            .priority(CasePriority::NOT_DEFINED)
            .title("A title for the Linear issue.".to_string()),
    )
    .relationships(CreateLinearIssueRequestDataRelationships::new(
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
    configuration.set_unstable_operation_enabled("v2.CreateLinearIssues", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.create_linear_issues(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
