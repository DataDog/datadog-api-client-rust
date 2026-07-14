// Update an incident user-defined role returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::api_incidents::UpdateIncidentUserDefinedRoleOptionalParams;
use datadog_api_client::datadogV2::model::IncidentUserDefinedRolePatchDataAttributesRequest;
use datadog_api_client::datadogV2::model::IncidentUserDefinedRolePatchDataRequest;
use datadog_api_client::datadogV2::model::IncidentUserDefinedRolePatchRequest;
use datadog_api_client::datadogV2::model::IncidentUserDefinedRolePolicy;
use datadog_api_client::datadogV2::model::IncidentUserDefinedRoleType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = IncidentUserDefinedRolePatchRequest::new(
        IncidentUserDefinedRolePatchDataRequest::new(
            Uuid::parse_str("00000000-0000-0000-0000-000000000002").expect("invalid UUID"),
            IncidentUserDefinedRoleType::INCIDENT_USER_DEFINED_ROLES,
        )
        .attributes(
            IncidentUserDefinedRolePatchDataAttributesRequest::new()
                .description(Some("The technical lead for the incident.".to_string()))
                .name("Tech Lead".to_string())
                .policy(IncidentUserDefinedRolePolicy::new(true)),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateIncidentUserDefinedRole", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .update_incident_user_defined_role(
            Uuid::parse_str("00000000-0000-0000-0000-000000000002").expect("invalid UUID"),
            body,
            UpdateIncidentUserDefinedRoleOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
