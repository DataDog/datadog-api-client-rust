// Create an On-Call notification channel for a user returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;
use datadog_api_client::datadogV2::model::CreateEmailNotificationChannelConfig;
use datadog_api_client::datadogV2::model::CreateNotificationChannelAttributes;
use datadog_api_client::datadogV2::model::CreateNotificationChannelConfig;
use datadog_api_client::datadogV2::model::CreateNotificationChannelData;
use datadog_api_client::datadogV2::model::CreateUserNotificationChannelRequest;
use datadog_api_client::datadogV2::model::NotificationChannelEmailConfigType;
use datadog_api_client::datadogV2::model::NotificationChannelEmailFormatType;
use datadog_api_client::datadogV2::model::NotificationChannelType;

#[tokio::main]
async fn main() {
    // there is a valid "user" in the system
    let user_data_id = std::env::var("USER_DATA_ID").unwrap();
    let body = CreateUserNotificationChannelRequest::new(
        CreateNotificationChannelData::new(NotificationChannelType::NOTIFICATION_CHANNELS)
            .attributes(CreateNotificationChannelAttributes::new().config(
                CreateNotificationChannelConfig::CreateEmailNotificationChannelConfig(Box::new(
                    CreateEmailNotificationChannelConfig::new(
                        "foo@bar.com".to_string(),
                        vec![NotificationChannelEmailFormatType::HTML],
                        NotificationChannelEmailConfigType::EMAIL,
                    ),
                )),
            )),
    );
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .create_user_notification_channel(user_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
