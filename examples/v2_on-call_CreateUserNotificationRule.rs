// Create an On-Call notification rule for a user returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;
use datadog_api_client::datadogV2::model::CreateOnCallNotificationRuleRequest;
use datadog_api_client::datadogV2::model::CreateOnCallNotificationRuleRequestData;
use datadog_api_client::datadogV2::model::NotificationChannelType;
use datadog_api_client::datadogV2::model::OnCallNotificationRuleCategory;
use datadog_api_client::datadogV2::model::OnCallNotificationRuleChannelRelationship;
use datadog_api_client::datadogV2::model::OnCallNotificationRuleChannelRelationshipData;
use datadog_api_client::datadogV2::model::OnCallNotificationRuleRelationships;
use datadog_api_client::datadogV2::model::OnCallNotificationRuleRequestAttributes;
use datadog_api_client::datadogV2::model::OnCallNotificationRuleType;

#[tokio::main]
async fn main() {
    // there is a valid "user" in the system
    let user_data_id = std::env::var("USER_DATA_ID").unwrap();

    // there is a valid "oncall_email_notification_channel" in the system
    let oncall_email_notification_channel_data_id =
        std::env::var("ONCALL_EMAIL_NOTIFICATION_CHANNEL_DATA_ID").unwrap();
    let body = CreateOnCallNotificationRuleRequest::new(
        CreateOnCallNotificationRuleRequestData::new(
            OnCallNotificationRuleType::NOTIFICATION_RULES,
        )
        .attributes(
            OnCallNotificationRuleRequestAttributes::new()
                .category(OnCallNotificationRuleCategory::HIGH_URGENCY)
                .delay_minutes(0),
        )
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
        .create_user_notification_rule(user_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
