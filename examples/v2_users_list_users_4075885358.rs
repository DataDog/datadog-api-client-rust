// List all users returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_users::*;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = UsersAPI::with_config(configuration);
    let response = api.list_users_with_pagination(ListUsersOptionalParams::default().page_size(2));
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}