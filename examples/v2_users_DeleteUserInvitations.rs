// Delete a pending user's invitations returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_users::UsersAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = UsersAPI::with_config(configuration);
    let resp = api
        .delete_user_invitations(
            Uuid::parse_str("4dee724d-00cc-11ea-a77b-570c9d03c6c5").expect("invalid UUID"),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
