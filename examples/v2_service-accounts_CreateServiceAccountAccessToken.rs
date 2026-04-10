// Create an access token for a service account returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_service_accounts::ServiceAccountsAPI;
use datadog_api_client::datadogV2::model::PersonalAccessTokensType;
use datadog_api_client::datadogV2::model::ServiceAccountAccessTokenCreateAttributes;
use datadog_api_client::datadogV2::model::ServiceAccountAccessTokenCreateData;
use datadog_api_client::datadogV2::model::ServiceAccountAccessTokenCreateRequest;

#[tokio::main]
async fn main() {
    // there is a valid "service_account_user" in the system
    let service_account_user_data_id = std::env::var("SERVICE_ACCOUNT_USER_DATA_ID").unwrap();
    let body =
        ServiceAccountAccessTokenCreateRequest::new(ServiceAccountAccessTokenCreateData::new(
            ServiceAccountAccessTokenCreateAttributes::new(
                "Example-Service-Account".to_string(),
                vec!["dashboards_read".to_string()],
            ),
            PersonalAccessTokensType::PERSONAL_ACCESS_TOKENS,
        ));
    let configuration = datadog::Configuration::new();
    let api = ServiceAccountsAPI::with_config(configuration);
    let resp = api
        .create_service_account_access_token(service_account_user_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
