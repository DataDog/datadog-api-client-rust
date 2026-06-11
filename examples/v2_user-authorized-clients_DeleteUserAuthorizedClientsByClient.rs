// Delete all user authorized clients for a client returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_user_authorized_clients::UserAuthorizedClientsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = UserAuthorizedClientsAPI::with_config(configuration);
    let resp = api
        .delete_user_authorized_clients_by_client(
            "00000000-0000-0000-0000-000000000010".to_string(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
