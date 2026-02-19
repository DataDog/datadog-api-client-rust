// Create a change request returns "Created" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_change_management::ChangeManagementAPI;
use datadog_api_client::datadogV2::model::ChangeRequestChangeType;
use datadog_api_client::datadogV2::model::ChangeRequestCreateAttributes;
use datadog_api_client::datadogV2::model::ChangeRequestCreateData;
use datadog_api_client::datadogV2::model::ChangeRequestCreateRequest;
use datadog_api_client::datadogV2::model::ChangeRequestResourceType;
use datadog_api_client::datadogV2::model::ChangeRequestRiskLevel;

#[tokio::main]
async fn main() {
    let body = ChangeRequestCreateRequest::new(ChangeRequestCreateData::new(
        ChangeRequestCreateAttributes::new("Deploy new payment service".to_string())
            .change_request_linked_incident_uuid("00000000-0000-0000-0000-000000000000".to_string())
            .change_request_maintenance_window_query("".to_string())
            .change_request_plan(
                "1. Deploy to staging 2. Run tests 3. Deploy to production".to_string(),
            )
            .change_request_risk(ChangeRequestRiskLevel::LOW)
            .change_request_type(ChangeRequestChangeType::NORMAL)
            .description("Deploying new payment service v2.1".to_string())
            .end_date(
                DateTime::parse_from_rfc3339("2024-01-02T15:00:00+00:00")
                    .expect("Failed to parse datetime")
                    .with_timezone(&Utc),
            )
            .project_id("d4bbe1af-f36e-42f1-87c1-493ca35c320e".to_string())
            .requested_teams(vec!["team-handle-1".to_string()])
            .start_date(
                DateTime::parse_from_rfc3339("2024-01-01T03:00:00+00:00")
                    .expect("Failed to parse datetime")
                    .with_timezone(&Utc),
            ),
        ChangeRequestResourceType::CHANGE_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateChangeRequest", true);
    let api = ChangeManagementAPI::with_config(configuration);
    let resp = api.create_change_request(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
