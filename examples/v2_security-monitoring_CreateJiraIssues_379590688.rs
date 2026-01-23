// Create Jira issue for security finding returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::CaseManagementProject;
use datadog_api_client::datadogV2::model::CaseManagementProjectData;
use datadog_api_client::datadogV2::model::CaseManagementProjectDataType;
use datadog_api_client::datadogV2::model::CreateJiraIssueRequestArray;
use datadog_api_client::datadogV2::model::CreateJiraIssueRequestData;
use datadog_api_client::datadogV2::model::CreateJiraIssueRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateJiraIssueRequestDataRelationships;
use datadog_api_client::datadogV2::model::FindingData;
use datadog_api_client::datadogV2::model::FindingDataType;
use datadog_api_client::datadogV2::model::Findings;
use datadog_api_client::datadogV2::model::JiraIssuesDataType;

#[tokio::main]
async fn main() {
    let body =
        CreateJiraIssueRequestArray::new(
            vec![
                CreateJiraIssueRequestData::new(JiraIssuesDataType::JIRA_ISSUES)
                    .attributes(
                        CreateJiraIssueRequestDataAttributes::new()
                            .description("A description".to_string())
                            .title("A title".to_string()),
                    )
                    .relationships(
                        CreateJiraIssueRequestDataRelationships::new(
                            Findings
                            ::new().data(
                                vec![
                                    FindingData::new(
                                        "YmNlZmJhYTcyMDU5ZDk0ZDhiNjRmNGI0NDk4MDdiNzN-MDJlMjg0NzNmYzJiODY2MzJkNjU0OTI4NmVhZTUyY2U=".to_string(),
                                        FindingDataType::FINDINGS,
                                    )
                                ],
                            ),
                            CaseManagementProject::new(
                                CaseManagementProjectData::new(
                                    "959a6f71-bac8-4027-b1d3-2264f569296f".to_string(),
                                    CaseManagementProjectDataType::PROJECTS,
                                ),
                            ),
                        ),
                    )
            ],
        );
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.create_jira_issues(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
