// Create an On-Call email for a user returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;
use datadog_api_client::datadogV2::model::EmailAttributes;
use datadog_api_client::datadogV2::model::EmailCreateRequest;
use datadog_api_client::datadogV2::model::EmailData;
use datadog_api_client::datadogV2::model::EmailFormatType;
use datadog_api_client::datadogV2::model::EmailType;

#[tokio::main]
async fn main() {
    // there is a valid "user" in the system
    let user_data_id = std::env::var("USER_DATA_ID").unwrap();
    let body = EmailCreateRequest::new().data(
        EmailData::new(EmailType::EMAILS).attributes(
            EmailAttributes::new()
                .active(true)
                .address("john.doe@datadoghq.com".to_string())
                .alias("".to_string())
                .formats(vec![EmailFormatType::HTML]),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .create_user_email_notification_channel(user_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
