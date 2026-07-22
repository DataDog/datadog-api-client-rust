// Update governance control notification settings returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_governance_controls::GovernanceControlsAPI;
use datadog_api_client::datadogV2::model::ControlNotificationEventSetting;
use datadog_api_client::datadogV2::model::ControlNotificationSettingsResourceType;
use datadog_api_client::datadogV2::model::ControlNotificationSettingsUpdateAttributes;
use datadog_api_client::datadogV2::model::ControlNotificationSettingsUpdateData;
use datadog_api_client::datadogV2::model::ControlNotificationSettingsUpdateRequest;
use datadog_api_client::datadogV2::model::ControlNotificationTarget;
use datadog_api_client::datadogV2::model::ControlNotificationTargetType;

#[tokio::main]
async fn main() {
    let body = ControlNotificationSettingsUpdateRequest::new(
        ControlNotificationSettingsUpdateData::new(
            ControlNotificationSettingsResourceType::CONTROL_NOTIFICATION_SETTINGS,
        )
        .attributes(
            ControlNotificationSettingsUpdateAttributes::new().event_settings(vec![
                ControlNotificationEventSetting::new(
                    true,
                    "new_detection".to_string(),
                    vec![ControlNotificationTarget::new(
                        "#governance-alerts".to_string(),
                        ControlNotificationTargetType::SLACK,
                    )],
                ),
            ]),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.UpdateGovernanceControlNotificationSettings", true);
    let api = GovernanceControlsAPI::with_config(configuration);
    let resp = api
        .update_governance_control_notification_settings("detection_type".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
