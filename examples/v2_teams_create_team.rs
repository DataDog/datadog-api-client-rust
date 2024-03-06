// Create a team returns "CREATED" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_teams::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body =
        TeamCreateRequest::new(
            TeamCreate::new(
                TeamCreateAttributes::new(
                    "test-handle-a0fc0297eb519635".to_string(),
                    "test-name-a0fc0297eb519635".to_string(),
                ),
                TeamType::TEAM,
            ).relationships(TeamCreateRelationships::new().users(RelationshipToUsers::new(vec![]))),
        );
    let configuration = Configuration::new();
    let api = TeamsAPI::with_config(configuration);
    let resp = api.create_team(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
