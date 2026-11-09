// Delete an email access policy returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;

#[tokio::main]
async fn main() {
    // there is a valid "private_status_page" in the system
    let private_status_page_data_id =
        uuid::Uuid::parse_str(&std::env::var("PRIVATE_STATUS_PAGE_DATA_ID").unwrap())
            .expect("Invalid UUID");

    // there is a valid "email_access_policy" in the system
    let email_access_policy_data_id =
        uuid::Uuid::parse_str(&std::env::var("EMAIL_ACCESS_POLICY_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .delete_email_access_policy(
            private_status_page_data_id.clone(),
            email_access_policy_data_id.clone(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
