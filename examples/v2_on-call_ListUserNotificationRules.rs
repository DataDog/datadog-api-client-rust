// List On-Call notification rules for a user returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::ListUserNotificationRulesOptionalParams;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;

#[tokio::main]
async fn main() {
    // there is a valid "user" in the system
    let user_data_id = std::env::var("USER_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .list_user_notification_rules(
            user_data_id.clone(),
            ListUserNotificationRulesOptionalParams::default().include("channel".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
