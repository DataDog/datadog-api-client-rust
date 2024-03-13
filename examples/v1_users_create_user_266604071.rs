// Create a user returns null access role
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_users::*;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    let body = User::new()
        .access_role(None)
        .disabled(false)
        .email("test@datadoghq.com".to_string())
        .handle("test@datadoghq.com".to_string())
        .name("test user".to_string());
    let configuration = Configuration::new();
    let api = UsersAPI::with_config(configuration);
    let resp = api.create_user(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
