// Mitigate governance detections returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_governance_controls::GovernanceControlsAPI;
use datadog_api_client::datadogV2::model::GovernanceControlDetectionResourceType;
use datadog_api_client::datadogV2::model::GovernanceMitigationRequest;
use datadog_api_client::datadogV2::model::GovernanceMitigationRequestAttributes;
use datadog_api_client::datadogV2::model::GovernanceMitigationRequestData;

#[tokio::main]
async fn main() {
    let body = GovernanceMitigationRequest::new(
        GovernanceMitigationRequestData::new(
            GovernanceControlDetectionResourceType::GOVERNANCE_CONTROL_DETECTION,
        )
        .attributes(
            GovernanceMitigationRequestAttributes::new()
                .detection_ids(vec!["3f9b2c1a-8d4e-4a6b-9c2f-1e7d5a0b3c4d".to_string()])
                .detection_type("unused_api_keys".to_string())
                .mitigation_type("revoke_api_key".to_string()),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateGovernanceMitigation", true);
    let api = GovernanceControlsAPI::with_config(configuration);
    let resp = api.create_governance_mitigation(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
