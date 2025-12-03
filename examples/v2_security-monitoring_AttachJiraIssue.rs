// Attach security findings to a Jira issue returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::AttachJiraIssueRequest;
use datadog_api_client::datadogV2::model::AttachJiraIssueRequestData;
use datadog_api_client::datadogV2::model::AttachJiraIssueRequestDataAttributes;
use datadog_api_client::datadogV2::model::AttachJiraIssueRequestDataRelationships;
use datadog_api_client::datadogV2::model::CaseManagementProject;
use datadog_api_client::datadogV2::model::CaseManagementProjectData;
use datadog_api_client::datadogV2::model::CaseManagementProjectDataType;
use datadog_api_client::datadogV2::model::FindingData;
use datadog_api_client::datadogV2::model::FindingDataType;
use datadog_api_client::datadogV2::model::Findings;
use datadog_api_client::datadogV2::model::JiraIssuesDataType;

#[tokio::main]
async fn main() {
    let body =
        AttachJiraIssueRequest
        ::new().data(
            AttachJiraIssueRequestData::new(JiraIssuesDataType::JIRA_ISSUES)
                .attributes(
                    AttachJiraIssueRequestDataAttributes::new(
                        "https://datadoghq-sandbox-538.atlassian.net/browse/CSMSEC-105476".to_string(),
                    ),
                )
                .relationships(
                    AttachJiraIssueRequestDataRelationships::new(
                        Findings
                        ::new().data(
                            vec![
                                FindingData::new(
                                    "OTQ3NjJkMmYwMTIzMzMxNTc1Y2Q4MTA5NWU0NTBmMDl-ZjE3NjMxZWVkYzBjZGI1NDY2NWY2OGQxZDk4MDY4MmI=".to_string(),
                                    FindingDataType::FINDINGS,
                                ),
                                FindingData::new(
                                    "MTNjN2ZmYWMzMDIxYmU1ZDFiZDRjNWUwN2I1NzVmY2F-YTA3MzllMTUzNWM3NmEyZjdiNzEzOWM5YmViZTMzOGM=".to_string(),
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
                ),
        );
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.attach_jira_issue(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
