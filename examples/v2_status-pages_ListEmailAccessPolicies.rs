// List email access policies returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::ListEmailAccessPoliciesOptionalParams;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;

#[tokio::main]
async fn main() {
    // there is a valid "private_status_page" in the system
    let private_status_page_data_id =
        uuid::Uuid::parse_str(&std::env::var("PRIVATE_STATUS_PAGE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .list_email_access_policies(
            private_status_page_data_id.clone(),
            ListEmailAccessPoliciesOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
