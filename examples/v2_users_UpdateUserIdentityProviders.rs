// Update identity provider overrides for a user returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_users::UsersAPI;
use datadog_api_client::datadogV2::model::UpdateUserIdentityProvidersRequest;
use datadog_api_client::datadogV2::model::UserRelationshipIdentityProviderData;
use datadog_api_client::datadogV2::model::UserRelationshipIdentityProviderDataType;

#[tokio::main]
async fn main() {
    let body =
        UpdateUserIdentityProvidersRequest::new(vec![UserRelationshipIdentityProviderData::new(
            "00000000-0000-0000-0000-000000000001".to_string(),
            UserRelationshipIdentityProviderDataType::IDENTITY_PROVIDERS,
        )]);
    let configuration = datadog::Configuration::new();
    let api = UsersAPI::with_config(configuration);
    let resp = api
        .update_user_identity_providers("00000000-0000-9999-0000-000000000000".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
