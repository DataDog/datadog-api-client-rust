// Update current user returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_users::UsersAPI;
use datadog_api_client::datadogV2::model::UserUpdateAttributes;
use datadog_api_client::datadogV2::model::UserUpdateData;
use datadog_api_client::datadogV2::model::UserUpdateRequest;
use datadog_api_client::datadogV2::model::UsersType;

#[tokio::main]
async fn main() {
    let body = UserUpdateRequest::new(UserUpdateData::new(
        UserUpdateAttributes::new().title(None),
        "00000000-0000-feed-0000-000000000000".to_string(),
        UsersType::USERS,
    ));
    let configuration = datadog::Configuration::new();
    let api = UsersAPI::with_config(configuration);
    let resp = api.update_current_user(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
