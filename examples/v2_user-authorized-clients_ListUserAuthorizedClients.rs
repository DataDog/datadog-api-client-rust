// List user authorized clients returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_user_authorized_clients::ListUserAuthorizedClientsOptionalParams;
use datadog_api_client::datadogV2::api_user_authorized_clients::UserAuthorizedClientsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = UserAuthorizedClientsAPI::with_config(configuration);
    let resp = api
        .list_user_authorized_clients(ListUserAuthorizedClientsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
