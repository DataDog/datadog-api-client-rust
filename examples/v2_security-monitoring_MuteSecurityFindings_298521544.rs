// Unmute security findings returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::FindingData;
use datadog_api_client::datadogV2::model::FindingDataType;
use datadog_api_client::datadogV2::model::Findings;
use datadog_api_client::datadogV2::model::MuteDataType;
use datadog_api_client::datadogV2::model::MuteFindingsMuteAttributes;
use datadog_api_client::datadogV2::model::MuteFindingsReason;
use datadog_api_client::datadogV2::model::MuteFindingsRequest;
use datadog_api_client::datadogV2::model::MuteFindingsRequestData;
use datadog_api_client::datadogV2::model::MuteFindingsRequestDataAttributes;
use datadog_api_client::datadogV2::model::MuteFindingsRequestDataRelationships;

#[tokio::main]
async fn main() {
    let body = MuteFindingsRequest::new(MuteFindingsRequestData::new(
        MuteFindingsRequestDataAttributes::new(
            MuteFindingsMuteAttributes::new(false, MuteFindingsReason::NO_PENDING_FIX)
                .description("Resolved.".to_string()),
        ),
        MuteFindingsRequestDataRelationships::new(Findings::new().data(vec![FindingData::new(
            "ZGVmLTAwMC0wYmd-MDE4NjcyMDJkMzE4MDE5ODY5MGE4ZmQ2MmFlMjg0Y2M=".to_string(),
            FindingDataType::FINDINGS,
        )])),
        MuteDataType::MUTE,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.MuteSecurityFindings", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.mute_security_findings(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
