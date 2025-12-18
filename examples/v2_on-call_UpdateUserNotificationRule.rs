// Update an On-Call notification rule for a user returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;
use datadog_api_client::datadogV2::api_on_call::UpdateUserNotificationRuleOptionalParams;
use datadog_api_client::datadogV2::model::NotificationChannelType;
use datadog_api_client::datadogV2::model::OnCallNotificationRuleCategory;
use datadog_api_client::datadogV2::model::OnCallNotificationRuleChannelRelationship;
use datadog_api_client::datadogV2::model::OnCallNotificationRuleChannelRelationshipData;
use datadog_api_client::datadogV2::model::OnCallNotificationRuleRelationships;
use datadog_api_client::datadogV2::model::OnCallNotificationRuleType;
use datadog_api_client::datadogV2::model::UpdateOnCallNotificationRuleRequest;
use datadog_api_client::datadogV2::model::UpdateOnCallNotificationRuleRequestAttributes;
use datadog_api_client::datadogV2::model::UpdateOnCallNotificationRuleRequestData;

#[tokio::main]
async fn main() {
    // there is a valid "user" in the system
    let user_data_id = std::env::var("USER_DATA_ID").unwrap();

    // there is a valid "oncall_email_notification_rule" in the system
    let oncall_email_notification_rule_data_id =
        std::env::var("ONCALL_EMAIL_NOTIFICATION_RULE_DATA_ID").unwrap();

    // there is a valid "oncall_email_notification_channel" in the system
    let oncall_email_notification_channel_data_id =
        std::env::var("ONCALL_EMAIL_NOTIFICATION_CHANNEL_DATA_ID").unwrap();
    let body = UpdateOnCallNotificationRuleRequest::new(
        UpdateOnCallNotificationRuleRequestData::new(
            OnCallNotificationRuleType::NOTIFICATION_RULES,
        )
        .attributes(
            UpdateOnCallNotificationRuleRequestAttributes::new()
                .category(OnCallNotificationRuleCategory::HIGH_URGENCY)
                .delay_minutes(1),
        )
        .id(oncall_email_notification_rule_data_id.clone())
        .relationships(
            OnCallNotificationRuleRelationships::new().channel(
                OnCallNotificationRuleChannelRelationship::new(
                    OnCallNotificationRuleChannelRelationshipData::new()
                        .id(oncall_email_notification_channel_data_id.clone())
                        .type_(NotificationChannelType::NOTIFICATION_CHANNELS),
                ),
            ),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .update_user_notification_rule(
            user_data_id.clone(),
            oncall_email_notification_rule_data_id.clone(),
            body,
            UpdateUserNotificationRuleOptionalParams::default().include("channel".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
