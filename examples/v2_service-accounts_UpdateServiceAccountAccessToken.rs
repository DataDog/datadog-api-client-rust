// Update an access token for a service account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_service_accounts::ServiceAccountsAPI;
use datadog_api_client::datadogV2::model::ServiceAccessTokensType;
use datadog_api_client::datadogV2::model::ServiceAccountAccessTokenUpdateAttributes;
use datadog_api_client::datadogV2::model::ServiceAccountAccessTokenUpdateData;
use datadog_api_client::datadogV2::model::ServiceAccountAccessTokenUpdateRequest;

#[tokio::main]
async fn main() {
    // there is a valid "service_account_user" in the system
    let service_account_user_data_id = std::env::var("SERVICE_ACCOUNT_USER_DATA_ID").unwrap();

    // there is a valid "service_account_access_token" for "service_account_user"
    let service_account_access_token_data_id =
        std::env::var("SERVICE_ACCOUNT_ACCESS_TOKEN_DATA_ID").unwrap();
    let body =
        ServiceAccountAccessTokenUpdateRequest::new(ServiceAccountAccessTokenUpdateData::new(
            ServiceAccountAccessTokenUpdateAttributes::new()
                .name("My Access Token-updated".to_string()),
            service_account_access_token_data_id.clone(),
            ServiceAccessTokensType::SERVICE_ACCESS_TOKENS,
        ));
    let configuration = datadog::Configuration::new();
    let api = ServiceAccountsAPI::with_config(configuration);
    let resp = api
        .update_service_account_access_token(
            service_account_user_data_id.clone(),
            service_account_access_token_data_id.clone(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
