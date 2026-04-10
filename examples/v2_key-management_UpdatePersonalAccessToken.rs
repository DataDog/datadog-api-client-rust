// Update a personal access token returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_key_management::KeyManagementAPI;
use datadog_api_client::datadogV2::model::PersonalAccessTokenUpdateAttributes;
use datadog_api_client::datadogV2::model::PersonalAccessTokenUpdateData;
use datadog_api_client::datadogV2::model::PersonalAccessTokenUpdateRequest;
use datadog_api_client::datadogV2::model::PersonalAccessTokensType;

#[tokio::main]
async fn main() {
    // there is a valid "personal_access_token" in the system
    let personal_access_token_data_id = std::env::var("PERSONAL_ACCESS_TOKEN_DATA_ID").unwrap();
    let body = PersonalAccessTokenUpdateRequest::new(PersonalAccessTokenUpdateData::new(
        PersonalAccessTokenUpdateAttributes::new()
            .name("Example-Key-Management-updated".to_string()),
        personal_access_token_data_id.clone(),
        PersonalAccessTokensType::PERSONAL_ACCESS_TOKENS,
    ));
    let configuration = datadog::Configuration::new();
    let api = KeyManagementAPI::with_config(configuration);
    let resp = api
        .update_personal_access_token(personal_access_token_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
