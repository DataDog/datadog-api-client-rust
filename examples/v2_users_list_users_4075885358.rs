// List all users returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_users::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = UsersAPI::with_config(configuration);
    let resp = api
        .list_users(ListUsersOptionalParams::default().page_size(2))
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}