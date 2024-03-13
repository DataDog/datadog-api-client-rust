// Update permission setting for team returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_teams::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    // there is a valid "dd_team" in the system
    let dd_team_data_id = std::env::var("DD_TEAM_DATA_ID").unwrap();
    let body = TeamPermissionSettingUpdateRequest::new(
        TeamPermissionSettingUpdate::new(TeamPermissionSettingType::TEAM_PERMISSION_SETTINGS)
            .attributes(
                TeamPermissionSettingUpdateAttributes::new()
                    .value(TeamPermissionSettingValue::ADMINS),
            ),
    );
    let configuration = Configuration::new();
    let api = TeamsAPI::with_config(configuration);
    let resp = api
        .update_team_permission_setting(
            dd_team_data_id.clone(),
            "manage_membership".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
