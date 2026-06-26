// Update a governance control detection returns "OK" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_governance_controls::GovernanceControlsAPI;
use datadog_api_client::datadogV2::model::GovernanceControlDetectionResourceType;
use datadog_api_client::datadogV2::model::GovernanceControlDetectionUpdateAttributes;
use datadog_api_client::datadogV2::model::GovernanceControlDetectionUpdateData;
use datadog_api_client::datadogV2::model::GovernanceControlDetectionUpdateRequest;
use datadog_api_client::datadogV2::model::GovernanceControlDetectionUpdateState;

#[tokio::main]
async fn main() {
    let body = GovernanceControlDetectionUpdateRequest::new(
        GovernanceControlDetectionUpdateData::new(
            GovernanceControlDetectionResourceType::GOVERNANCE_CONTROL_DETECTION,
        )
        .attributes(
            GovernanceControlDetectionUpdateAttributes::new()
                .assigned_team("platform-security".to_string())
                .assigned_to("11111111-2222-3333-4444-555555555555".to_string())
                .mitigate_after(
                    DateTime::parse_from_rfc3339("2024-03-15T00:00:00+00:00")
                        .expect("Failed to parse datetime")
                        .with_timezone(&Utc),
                )
                .state(GovernanceControlDetectionUpdateState::EXCEPTION),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateGovernanceControlDetection", true);
    let api = GovernanceControlsAPI::with_config(configuration);
    let resp = api
        .update_governance_control_detection(
            "detection_type".to_string(),
            "detection_id".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
