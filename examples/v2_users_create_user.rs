// Create a user returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_users::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body = UserCreateRequest::new(UserCreateData::new(
        UserCreateAttributes::new("Example-User@datadoghq.com".to_string())
            .name("Datadog API Client Python".to_string()),
        UsersType::USERS,
    ));
    let configuration = Configuration::new();
    let api = UsersAPI::with_config(configuration);
    let resp = api.create_user(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
