// Update governance notification settings returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_governance_settings::GovernanceSettingsAPI;
use datadog_api_client::datadogV2::model::GovernanceNotificationSettingsResourceType;
use datadog_api_client::datadogV2::model::GovernanceNotificationSettingsUpdateAttributes;
use datadog_api_client::datadogV2::model::GovernanceNotificationSettingsUpdateData;
use datadog_api_client::datadogV2::model::GovernanceNotificationSettingsUpdateRequest;

#[tokio::main]
async fn main() {
    let body = GovernanceNotificationSettingsUpdateRequest::new(
        GovernanceNotificationSettingsUpdateData::new(
            GovernanceNotificationSettingsResourceType::GOVERNANCE_NOTIFICATION_SETTINGS,
        )
        .attributes(
            GovernanceNotificationSettingsUpdateAttributes::new()
                .assignment_notifications_enabled(true),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateGovernanceNotificationSettings", true);
    let api = GovernanceSettingsAPI::with_config(configuration);
    let resp = api.update_governance_notification_settings(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
