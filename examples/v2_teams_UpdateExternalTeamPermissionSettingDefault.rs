// Update an external team permission default returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_teams::TeamsAPI;
use datadog_api_client::datadogV2::model::ExternalTeamPermissionSettingDefaultUpdateAttributes;
use datadog_api_client::datadogV2::model::ExternalTeamPermissionSettingDefaultUpdateData;
use datadog_api_client::datadogV2::model::ExternalTeamPermissionSettingDefaultUpdateRequest;
use datadog_api_client::datadogV2::model::ExternalTeamPermissionSettingDefaultsSource;
use datadog_api_client::datadogV2::model::ExternalTeamPermissionSettingDefaultsType;

#[tokio::main]
async fn main() {
    let body = ExternalTeamPermissionSettingDefaultUpdateRequest::new(
        ExternalTeamPermissionSettingDefaultUpdateData::new(
            ExternalTeamPermissionSettingDefaultUpdateAttributes::new(
                "user_access_manage".to_string(),
            ),
            "scim:add_and_remove_subteam".to_string(),
            ExternalTeamPermissionSettingDefaultsType::EXTERNAL_TEAM_PERMISSION_SETTING_DEFAULTS,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = TeamsAPI::with_config(configuration);
    let resp = api
        .update_external_team_permission_setting_default(
            ExternalTeamPermissionSettingDefaultsSource::SCIM,
            "add_and_remove_subteam".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
