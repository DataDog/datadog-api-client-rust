// Create an incident role assignment returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentRoleAssignmentDataAttributesRequest;
use datadog_api_client::datadogV2::model::IncidentRoleAssignmentDataRequest;
use datadog_api_client::datadogV2::model::IncidentRoleAssignmentRelationshipsRequest;
use datadog_api_client::datadogV2::model::IncidentRoleAssignmentRequest;
use datadog_api_client::datadogV2::model::IncidentRoleAssignmentResponderRelationship;
use datadog_api_client::datadogV2::model::IncidentRoleAssignmentResponderRelationshipData;
use datadog_api_client::datadogV2::model::IncidentRoleAssignmentRoleRelationship;
use datadog_api_client::datadogV2::model::IncidentRoleAssignmentRoleRelationshipData;
use datadog_api_client::datadogV2::model::IncidentRoleAssignmentType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = IncidentRoleAssignmentRequest::new(
        IncidentRoleAssignmentDataRequest::new(
            IncidentRoleAssignmentRelationshipsRequest::new(
                IncidentRoleAssignmentResponderRelationship::new(
                    IncidentRoleAssignmentResponderRelationshipData::new(
                        Uuid::parse_str("00000000-0000-0000-0000-000000000000")
                            .expect("invalid UUID"),
                        "users".to_string(),
                    ),
                ),
            )
            .reserved_role(IncidentRoleAssignmentRoleRelationship::new().data(
                IncidentRoleAssignmentRoleRelationshipData::new(
                    Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
                    "incident_reserved_roles".to_string(),
                ),
            ))
            .user_defined_role(IncidentRoleAssignmentRoleRelationship::new().data(
                IncidentRoleAssignmentRoleRelationshipData::new(
                    Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
                    "incident_reserved_roles".to_string(),
                ),
            )),
            IncidentRoleAssignmentType::INCIDENT_ROLE_ASSIGNMENTS,
        )
        .attributes(
            IncidentRoleAssignmentDataAttributesRequest::new().role(Some("commander".to_string())),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentRoleAssignment", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api.create_incident_role_assignment(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
