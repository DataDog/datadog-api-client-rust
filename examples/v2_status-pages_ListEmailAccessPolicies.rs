// List email access policies returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::ListEmailAccessPoliciesOptionalParams;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .list_email_access_policies(
            Uuid::parse_str("1234abcd-12ab-34cd-56ef-123456abcdef").expect("invalid UUID"),
            ListEmailAccessPoliciesOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
