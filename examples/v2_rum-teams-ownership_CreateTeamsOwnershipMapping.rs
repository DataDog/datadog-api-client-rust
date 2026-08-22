// Create a teams ownership mapping returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_teams_ownership::RumTeamsOwnershipAPI;
use datadog_api_client::datadogV2::model::TeamsOwnershipMappingCreateData;
use datadog_api_client::datadogV2::model::TeamsOwnershipMappingCreateDataAttributes;
use datadog_api_client::datadogV2::model::TeamsOwnershipMappingCreateRequest;
use datadog_api_client::datadogV2::model::TeamsOwnershipMappingType;
use datadog_api_client::datadogV2::model::TeamsOwnershipMatchType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = TeamsOwnershipMappingCreateRequest::new(TeamsOwnershipMappingCreateData::new(
        TeamsOwnershipMappingCreateDataAttributes::new(
            "team-rum".to_string(),
            "/checkout".to_string(),
        )
        .application_id(
            Uuid::parse_str("11111111-2222-3333-4444-555555555555").expect("invalid UUID"),
        )
        .match_type(TeamsOwnershipMatchType::EXACT)
        .service("web-checkout".to_string()),
        TeamsOwnershipMappingType::TEAMS_OWNERSHIP_MAPPINGS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateTeamsOwnershipMapping", true);
    let api = RumTeamsOwnershipAPI::with_config(configuration);
    let resp = api.create_teams_ownership_mapping(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
