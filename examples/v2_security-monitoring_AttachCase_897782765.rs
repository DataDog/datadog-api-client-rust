// Attach security finding to a case returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::AttachCaseRequest;
use datadog_api_client::datadogV2::model::AttachCaseRequestData;
use datadog_api_client::datadogV2::model::AttachCaseRequestDataRelationships;
use datadog_api_client::datadogV2::model::CaseDataType;
use datadog_api_client::datadogV2::model::FindingData;
use datadog_api_client::datadogV2::model::FindingDataType;
use datadog_api_client::datadogV2::model::Findings;

#[tokio::main]
async fn main() {
    let body =
        AttachCaseRequest
        ::new().data(
            AttachCaseRequestData::new(
                "7d16945b-baf8-411e-ab2a-20fe43af1ea3".to_string(),
                CaseDataType::CASES,
            ).relationships(
                AttachCaseRequestDataRelationships::new(
                    Findings
                    ::new().data(
                        vec![
                            FindingData::new(
                                "ZGZhMDI3ZjdjMDM3YjJmNzcxNTlhZGMwMjdmZWNiNTZ-MTVlYTNmYWU3NjNlOTNlYTE2YjM4N2JmZmI4Yjk5N2Y=".to_string(),
                                FindingDataType::FINDINGS,
                            )
                        ],
                    ),
                ),
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .attach_case("7d16945b-baf8-411e-ab2a-20fe43af1ea3".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
