// Update a team with partial update returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_teams::TeamsAPI;
use datadog_api_client::datadogV2::model::TeamType;
use datadog_api_client::datadogV2::model::TeamUpdate;
use datadog_api_client::datadogV2::model::TeamUpdateAttributes;
use datadog_api_client::datadogV2::model::TeamUpdateRequest;

#[tokio::main]
async fn main() {
    // there is a valid "dd_team" in the system
    let dd_team_data_attributes_handle = std::env::var("DD_TEAM_DATA_ATTRIBUTES_HANDLE").unwrap();
    let dd_team_data_id = std::env::var("DD_TEAM_DATA_ID").unwrap();
    let body = TeamUpdateRequest::new(TeamUpdate::new(
        TeamUpdateAttributes::new(
            dd_team_data_attributes_handle.clone(),
            "Example Team updated".to_string(),
        ),
        TeamType::TEAM,
    ));
    let configuration = datadog::Configuration::new();
    let api = TeamsAPI::with_config(configuration);
    let resp = api.update_team(dd_team_data_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
