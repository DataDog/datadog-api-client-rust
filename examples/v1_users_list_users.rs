// List all users returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_users::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = UsersAPI::with_config(configuration);
    let resp = api.list_users().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}