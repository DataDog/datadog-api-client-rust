// Update an On-Call email for a user returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;
use datadog_api_client::datadogV2::model::EmailAttributes;
use datadog_api_client::datadogV2::model::EmailData;
use datadog_api_client::datadogV2::model::EmailFormatType;
use datadog_api_client::datadogV2::model::EmailType;
use datadog_api_client::datadogV2::model::EmailUpdateRequest;

#[tokio::main]
async fn main() {
    // there is a valid "user" in the system
    let user_data_id = std::env::var("USER_DATA_ID").unwrap();

    // there is a valid "oncall_email" in the system
    let oncall_email_data_attributes_address =
        std::env::var("ONCALL_EMAIL_DATA_ATTRIBUTES_ADDRESS").unwrap();
    let oncall_email_data_id = std::env::var("ONCALL_EMAIL_DATA_ID").unwrap();
    let body = EmailUpdateRequest::new().data(
        EmailData::new(EmailType::EMAILS)
            .attributes(
                EmailAttributes::new()
                    .active(true)
                    .address(oncall_email_data_attributes_address.clone())
                    .alias("Example-On-Call-alias".to_string())
                    .formats(vec![EmailFormatType::HTML]),
            )
            .id(oncall_email_data_id.clone()),
    );
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .update_user_email_notification_channel(
            user_data_id.clone(),
            oncall_email_data_id.clone(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
