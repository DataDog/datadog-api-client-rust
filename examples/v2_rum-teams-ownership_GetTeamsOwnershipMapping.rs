// Get a teams ownership mapping returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_teams_ownership::RumTeamsOwnershipAPI;

#[tokio::main]
async fn main() {
    // there is a valid "teams_ownership_mapping" in the system
    let teams_ownership_mapping_data_id = std::env::var("TEAMS_OWNERSHIP_MAPPING_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = RumTeamsOwnershipAPI::with_config(configuration);
    let resp = api
        .get_teams_ownership_mapping(teams_ownership_mapping_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
