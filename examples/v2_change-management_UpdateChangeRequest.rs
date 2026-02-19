// Update a change request returns "OK" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_change_management::ChangeManagementAPI;
use datadog_api_client::datadogV2::model::ChangeRequestChangeType;
use datadog_api_client::datadogV2::model::ChangeRequestDecisionCreateAttributes;
use datadog_api_client::datadogV2::model::ChangeRequestDecisionCreateItem;
use datadog_api_client::datadogV2::model::ChangeRequestDecisionCreateRelationships;
use datadog_api_client::datadogV2::model::ChangeRequestDecisionRelationshipData;
use datadog_api_client::datadogV2::model::ChangeRequestDecisionResourceType;
use datadog_api_client::datadogV2::model::ChangeRequestDecisionStatusType;
use datadog_api_client::datadogV2::model::ChangeRequestDecisionsRelationship;
use datadog_api_client::datadogV2::model::ChangeRequestResourceType;
use datadog_api_client::datadogV2::model::ChangeRequestRiskLevel;
use datadog_api_client::datadogV2::model::ChangeRequestUpdateAttributes;
use datadog_api_client::datadogV2::model::ChangeRequestUpdateData;
use datadog_api_client::datadogV2::model::ChangeRequestUpdateRelationships;
use datadog_api_client::datadogV2::model::ChangeRequestUpdateRequest;
use datadog_api_client::datadogV2::model::ChangeRequestUserRelationship;
use datadog_api_client::datadogV2::model::ChangeRequestUserRelationshipData;

#[tokio::main]
async fn main() {
    let body = ChangeRequestUpdateRequest::new(
        ChangeRequestUpdateData::new(ChangeRequestResourceType::CHANGE_REQUEST)
            .attributes(
                ChangeRequestUpdateAttributes::new()
                    .change_request_plan("Updated deployment plan".to_string())
                    .change_request_risk(ChangeRequestRiskLevel::LOW)
                    .change_request_type(ChangeRequestChangeType::NORMAL)
                    .end_date(
                        DateTime::parse_from_rfc3339("2024-01-02T15:00:00+00:00")
                            .expect("Failed to parse datetime")
                            .with_timezone(&Utc),
                    )
                    .id("CHM-1234".to_string())
                    .start_date(
                        DateTime::parse_from_rfc3339("2024-01-01T03:00:00+00:00")
                            .expect("Failed to parse datetime")
                            .with_timezone(&Utc),
                    ),
            )
            .relationships(
                ChangeRequestUpdateRelationships::new().change_request_decisions(
                    ChangeRequestDecisionsRelationship::new(vec![
                        ChangeRequestDecisionRelationshipData::new(
                            "decision-id-0".to_string(),
                            ChangeRequestDecisionResourceType::CHANGE_REQUEST_DECISION,
                        ),
                    ]),
                ),
            ),
    )
    .included(vec![ChangeRequestDecisionCreateItem::new(
        "decision-id-0".to_string(),
        ChangeRequestDecisionResourceType::CHANGE_REQUEST_DECISION,
    )
    .attributes(
        ChangeRequestDecisionCreateAttributes::new()
            .change_request_status(ChangeRequestDecisionStatusType::REQUESTED)
            .request_reason("Please review and approve this change".to_string()),
    )
    .relationships(
        ChangeRequestDecisionCreateRelationships::new().requested_user(
            ChangeRequestUserRelationship::new(Some(ChangeRequestUserRelationshipData::new(
                "00000000-0000-0000-0000-000000000000".to_string(),
                "user".to_string(),
            ))),
        ),
    )]);
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateChangeRequest", true);
    let api = ChangeManagementAPI::with_config(configuration);
    let resp = api
        .update_change_request("change_request_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
