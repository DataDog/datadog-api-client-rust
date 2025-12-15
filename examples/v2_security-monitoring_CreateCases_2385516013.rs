// Create case for security finding returns "Created" response
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
                                        "YjdhNDM3N2QyNTFjYmUwYTY3NDdhMTg0YTk2Yjg5MDl-ZjNmMzAwOTFkZDNhNGQzYzI0MzgxNTk4MjRjZmE2NzE=".to_string(),
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
