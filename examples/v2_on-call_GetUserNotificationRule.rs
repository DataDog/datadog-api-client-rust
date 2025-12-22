// Get an On-Call notification rule for a user returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::GetUserNotificationRuleOptionalParams;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;

#[tokio::main]
async fn main() {
    // there is a valid "user" in the system
    let user_data_id = std::env::var("USER_DATA_ID").unwrap();

    // there is a valid "oncall_email_notification_rule" in the system
    let oncall_email_notification_rule_data_id =
        std::env::var("ONCALL_EMAIL_NOTIFICATION_RULE_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .get_user_notification_rule(
            user_data_id.clone(),
            oncall_email_notification_rule_data_id.clone(),
            GetUserNotificationRuleOptionalParams::default().include("channel".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
