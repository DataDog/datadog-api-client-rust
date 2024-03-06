// Update a user returns "User updated" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_users::*;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    let body =
        User::new()
            .access_role(Some(AccessRole::READ_ONLY))
            .disabled(false)
            .email("test@datadoghq.com".to_string())
            .handle("test@datadoghq.com".to_string())
            .name("test user".to_string());
    let configuration = Configuration::new();
    let api = UsersAPI::with_config(configuration);
    let resp = api.update_user("test@datadoghq.com".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
