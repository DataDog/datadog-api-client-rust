// Create Jira issues for security findings returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::CaseDataType;
use datadog_api_client::datadogV2::model::CaseManagementProject;
use datadog_api_client::datadogV2::model::CaseManagementProjectData;
use datadog_api_client::datadogV2::model::CaseManagementProjectDataType;
use datadog_api_client::datadogV2::model::CreateCaseRequestData;
use datadog_api_client::datadogV2::model::CreateCaseRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateCaseRequestDataRelationships;
use datadog_api_client::datadogV2::model::CreateJiraIssueRequestArray;
use datadog_api_client::datadogV2::model::CreateJiraIssueRequestArrayIncluded;
use datadog_api_client::datadogV2::model::CreateJiraIssueRequestData;
use datadog_api_client::datadogV2::model::CreateJiraIssueRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateJiraIssueRequestDataRelationships;
use datadog_api_client::datadogV2::model::CreateJiraIssueRequestDataRelationshipsCase;
use datadog_api_client::datadogV2::model::CreateJiraIssueRequestDataRelationshipsCaseData;
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
                    .attributes(CreateJiraIssueRequestDataAttributes::new())
                    .relationships(
                        CreateJiraIssueRequestDataRelationships::new(
                            CreateJiraIssueRequestDataRelationshipsCase::new(
                                CreateJiraIssueRequestDataRelationshipsCaseData::new(
                                    "53e242c6-a7d6-46ad-9680-b8d14753f716".to_string(),
                                    CaseDataType::CASES,
                                ),
                            ),
                        ),
                    ),
                CreateJiraIssueRequestData::new(JiraIssuesDataType::JIRA_ISSUES)
                    .attributes(CreateJiraIssueRequestDataAttributes::new())
                    .relationships(
                        CreateJiraIssueRequestDataRelationships::new(
                            CreateJiraIssueRequestDataRelationshipsCase::new(
                                CreateJiraIssueRequestDataRelationshipsCaseData::new(
                                    "195772b2-1f53-41d2-b81e-48c8e6c21d33".to_string(),
                                    CaseDataType::CASES,
                                ),
                            ),
                        ),
                    )
            ],
        ).included(
            vec![
                CreateJiraIssueRequestArrayIncluded::CreateCaseRequestData(
                    Box::new(
                        CreateCaseRequestData::new(CaseDataType::CASES)
                            .attributes(
                                CreateCaseRequestDataAttributes::new()
                                    .description("A description".to_string())
                                    .title("A title".to_string()),
                            )
                            .id("53e242c6-a7d6-46ad-9680-b8d14753f716".to_string())
                            .relationships(
                                CreateCaseRequestDataRelationships::new(
                                    Findings
                                    ::new().data(
                                        vec![
                                            FindingData::new(
                                                "OTQ3NjJkMmYwMTIzMzMxNTc1Y2Q4MTA5NWU0NTBmMDl-ZjE3NjMxZWVkYzBjZGI1NDY2NWY2OGQxZDk4MDY4MmI=".to_string(),
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
                    ),
                ),
                CreateJiraIssueRequestArrayIncluded::CreateCaseRequestData(
                    Box::new(
                        CreateCaseRequestData::new(CaseDataType::CASES)
                            .attributes(
                                CreateCaseRequestDataAttributes::new()
                                    .description("A description".to_string())
                                    .title("A title".to_string()),
                            )
                            .id("195772b2-1f53-41d2-b81e-48c8e6c21d33".to_string())
                            .relationships(
                                CreateCaseRequestDataRelationships::new(
                                    Findings
                                    ::new().data(
                                        vec![
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
                    ),
                ),
                CreateJiraIssueRequestArrayIncluded::CaseManagementProjectData(
                    Box::new(
                        CaseManagementProjectData::new(
                            "959a6f71-bac8-4027-b1d3-2264f569296f".to_string(),
                            CaseManagementProjectDataType::PROJECTS,
                        ),
                    ),
                ),
                CreateJiraIssueRequestArrayIncluded::FindingData(
                    Box::new(
                        FindingData::new(
                            "OTQ3NjJkMmYwMTIzMzMxNTc1Y2Q4MTA5NWU0NTBmMDl-ZjE3NjMxZWVkYzBjZGI1NDY2NWY2OGQxZDk4MDY4MmI=".to_string(),
                            FindingDataType::FINDINGS,
                        ),
                    ),
                ),
                CreateJiraIssueRequestArrayIncluded::FindingData(
                    Box::new(
                        FindingData::new(
                            "MTNjN2ZmYWMzMDIxYmU1ZDFiZDRjNWUwN2I1NzVmY2F-YTA3MzllMTUzNWM3NmEyZjdiNzEzOWM5YmViZTMzOGM=".to_string(),
                            FindingDataType::FINDINGS,
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
