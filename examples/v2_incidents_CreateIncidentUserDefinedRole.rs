// Create an incident user-defined role returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::CreateIncidentUserDefinedRoleOptionalParams;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentUserDefinedRoleDataAttributesRequest;
use datadog_api_client::datadogV2::model::IncidentUserDefinedRoleDataRequest;
use datadog_api_client::datadogV2::model::IncidentUserDefinedRoleIncidentTypeRelationship;
use datadog_api_client::datadogV2::model::IncidentUserDefinedRoleIncidentTypeRelationshipData;
use datadog_api_client::datadogV2::model::IncidentUserDefinedRolePolicy;
use datadog_api_client::datadogV2::model::IncidentUserDefinedRoleRelationshipsRequest;
use datadog_api_client::datadogV2::model::IncidentUserDefinedRoleRequest;
use datadog_api_client::datadogV2::model::IncidentUserDefinedRoleType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = IncidentUserDefinedRoleRequest::new(IncidentUserDefinedRoleDataRequest::new(
        IncidentUserDefinedRoleDataAttributesRequest::new(
            "Tech Lead".to_string(),
            IncidentUserDefinedRolePolicy::new(true),
        )
        .description(Some("The technical lead for the incident.".to_string())),
        IncidentUserDefinedRoleRelationshipsRequest::new(
            IncidentUserDefinedRoleIncidentTypeRelationship::new(
                IncidentUserDefinedRoleIncidentTypeRelationshipData::new(
                    Uuid::parse_str("00000000-0000-0000-0000-000000000001").expect("invalid UUID"),
                    "incident_types".to_string(),
                ),
            ),
        ),
        IncidentUserDefinedRoleType::INCIDENT_USER_DEFINED_ROLES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentUserDefinedRole", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .create_incident_user_defined_role(
            body,
            CreateIncidentUserDefinedRoleOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
