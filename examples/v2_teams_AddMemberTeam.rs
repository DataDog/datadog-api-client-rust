// Add a member team returns "Added" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_teams::TeamsAPI;
use datadog_api_client::datadogV2::model::AddMemberTeamRequest;
use datadog_api_client::datadogV2::model::MemberTeam;
use datadog_api_client::datadogV2::model::MemberTeamType;

#[tokio::main]
async fn main() {
    let body = AddMemberTeamRequest::new(MemberTeam::new(
        "aeadc05e-98a8-11ec-ac2c-da7ad0900001".to_string(),
        MemberTeamType::MEMBER_TEAMS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.AddMemberTeam", true);
    let api = TeamsAPI::with_config(configuration);
    let resp = api.add_member_team("super_team_id".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
