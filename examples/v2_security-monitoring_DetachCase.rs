// Detach security findings from their case returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::CaseDataType;
use datadog_api_client::datadogV2::model::DetachCaseRequest;
use datadog_api_client::datadogV2::model::DetachCaseRequestData;
use datadog_api_client::datadogV2::model::DetachCaseRequestDataRelationships;
use datadog_api_client::datadogV2::model::FindingData;
use datadog_api_client::datadogV2::model::FindingDataType;
use datadog_api_client::datadogV2::model::Findings;

#[tokio::main]
async fn main() {
    let body =
        DetachCaseRequest
        ::new().data(
            DetachCaseRequestData::new(
                CaseDataType::CASES,
            ).relationships(
                DetachCaseRequestDataRelationships::new(
                    Findings
                    ::new().data(
                        vec![
                            FindingData::new(
                                "YzM2MTFjYzcyNmY0Zjg4MTAxZmRlNjQ1MWU1ZGQwYzR-YzI5NzE5Y2Y4MzU4ZjliNzhkNjYxNTY0ODIzZDQ2YTM=".to_string(),
                                FindingDataType::FINDINGS,
                            )
                        ],
                    ),
                ),
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.detach_case(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
