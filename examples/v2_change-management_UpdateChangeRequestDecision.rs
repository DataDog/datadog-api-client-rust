// Update a change request decision returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_change_management::ChangeManagementAPI;
use datadog_api_client::datadogV2::model::ChangeRequestDecisionCreateAttributes;
use datadog_api_client::datadogV2::model::ChangeRequestDecisionCreateItem;
use datadog_api_client::datadogV2::model::ChangeRequestDecisionCreateRelationships;
use datadog_api_client::datadogV2::model::ChangeRequestDecisionRelationshipData;
use datadog_api_client::datadogV2::model::ChangeRequestDecisionResourceType;
use datadog_api_client::datadogV2::model::ChangeRequestDecisionStatusType;
use datadog_api_client::datadogV2::model::ChangeRequestDecisionUpdateData;
use datadog_api_client::datadogV2::model::ChangeRequestDecisionUpdateDataAttributes;
use datadog_api_client::datadogV2::model::ChangeRequestDecisionUpdateDataRelationships;
use datadog_api_client::datadogV2::model::ChangeRequestDecisionUpdateRequest;
use datadog_api_client::datadogV2::model::ChangeRequestDecisionsRelationship;
use datadog_api_client::datadogV2::model::ChangeRequestResourceType;
use datadog_api_client::datadogV2::model::ChangeRequestUserRelationship;
use datadog_api_client::datadogV2::model::ChangeRequestUserRelationshipData;

#[tokio::main]
async fn main() {
    let body = ChangeRequestDecisionUpdateRequest::new(
        ChangeRequestDecisionUpdateData::new(ChangeRequestResourceType::CHANGE_REQUEST)
            .attributes(ChangeRequestDecisionUpdateDataAttributes::new().id("CHM-1234".to_string()))
            .relationships(ChangeRequestDecisionUpdateDataRelationships::new(
                ChangeRequestDecisionsRelationship::new(vec![
                    ChangeRequestDecisionRelationshipData::new(
                        "decision-id-0".to_string(),
                        ChangeRequestDecisionResourceType::CHANGE_REQUEST_DECISION,
                    ),
                ]),
            )),
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
    configuration.set_unstable_operation_enabled("v2.UpdateChangeRequestDecision", true);
    let api = ChangeManagementAPI::with_config(configuration);
    let resp = api
        .update_change_request_decision(
            "change_request_id".to_string(),
            "decision_id".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
