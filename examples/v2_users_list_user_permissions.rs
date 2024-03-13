// Get a user permissions returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_users::*;

#[tokio::main]
async fn main() {
    // there is a valid "user" in the system
    let user_data_id = std::env::var("USER_DATA_ID").unwrap();
    let configuration = Configuration::new();
    let api = UsersAPI::with_config(configuration);
    let resp = api.list_user_permissions(user_data_id.clone()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}