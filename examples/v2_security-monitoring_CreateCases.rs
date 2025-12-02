// Create cases for security findings returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::CaseDataType;
use datadog_api_client::datadogV2::model::CaseManagementProject;
use datadog_api_client::datadogV2::model::CaseManagementProjectData;
use datadog_api_client::datadogV2::model::CaseManagementProjectDataType;
use datadog_api_client::datadogV2::model::CreateCaseRequestArray;
use datadog_api_client::datadogV2::model::CreateCaseRequestData;
use datadog_api_client::datadogV2::model::CreateCaseRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateCaseRequestDataRelationships;
use datadog_api_client::datadogV2::model::FindingData;
use datadog_api_client::datadogV2::model::FindingDataType;
use datadog_api_client::datadogV2::model::Findings;

#[tokio::main]
async fn main() {
    let body =
        CreateCaseRequestArray::new(
            vec![
                CreateCaseRequestData::new(CaseDataType::CASES)
                    .attributes(
                        CreateCaseRequestDataAttributes::new()
                            .description("A description".to_string())
                            .title("A title".to_string()),
                    )
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
                CreateCaseRequestData::new(CaseDataType::CASES)
                    .attributes(
                        CreateCaseRequestDataAttributes::new()
                            .description("A description".to_string())
                            .title("A title".to_string()),
                    )
                    .relationships(
                        CreateCaseRequestDataRelationships::new(
                            Findings
                            ::new().data(
                                vec![
                                    FindingData::new(
                                        "MzZkNTMxODNmOGZlZmJiYzIyMDg4NzhmM2QyMDExZjB-ZmY5NzUwNDQzYTE0MGIyNDM1MTg4YjkxZDNmMDU4OGU=".to_string(),
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
    let resp = api.create_cases(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
