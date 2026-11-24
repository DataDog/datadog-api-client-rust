// Delete an external team permission default returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_teams::TeamsAPI;
use datadog_api_client::datadogV2::model::ExternalTeamPermissionSettingDefaultsSource;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = TeamsAPI::with_config(configuration);
    let resp = api
        .delete_external_team_permission_setting_default(
            ExternalTeamPermissionSettingDefaultsSource::SCIM,
            "add_and_remove_subteam".to_string(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
