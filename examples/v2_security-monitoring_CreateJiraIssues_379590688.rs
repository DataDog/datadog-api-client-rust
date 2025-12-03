// Create Jira issue for security finding returns "Created" response
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
                                    "6a773295-8729-4034-aada-53b64cbe02e7".to_string(),
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
                            .id("6a773295-8729-4034-aada-53b64cbe02e7".to_string())
                            .relationships(
                                CreateCaseRequestDataRelationships::new(
                                    Findings
                                    ::new().data(
                                        vec![
                                            FindingData::new(
                                                "ZGZhMDI3ZjdjMDM3YjJmNzcxNTlhZGMwMjdmZWNiNTZ-MTVlYTNmYWU3NjNlOTNlYTE2YjM4N2JmZmI4Yjk5N2Y=".to_string(),
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
                            "ZGZhMDI3ZjdjMDM3YjJmNzcxNTlhZGMwMjdmZWNiNTZ-MTVlYTNmYWU3NjNlOTNlYTE2YjM4N2JmZmI4Yjk5N2Y=".to_string(),
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
