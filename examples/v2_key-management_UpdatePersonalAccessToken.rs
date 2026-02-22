// Update personal access token returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_key_management::KeyManagementAPI;
use datadog_api_client::datadogV2::model::PersonalAccessTokenType;
use datadog_api_client::datadogV2::model::PersonalAccessTokenUpdateAttributes;
use datadog_api_client::datadogV2::model::PersonalAccessTokenUpdateData;
use datadog_api_client::datadogV2::model::PersonalAccessTokenUpdateRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = PersonalAccessTokenUpdateRequest::new(PersonalAccessTokenUpdateData::new(
        PersonalAccessTokenUpdateAttributes::new()
            .name("Updated Personal Access Token Name".to_string())
            .scopes(vec![
                "dashboards_read".to_string(),
                "dashboards_write".to_string(),
            ]),
        Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
        PersonalAccessTokenType::PERSONAL_ACCESS_TOKENS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdatePersonalAccessToken", true);
    let api = KeyManagementAPI::with_config(configuration);
    let resp = api
        .update_personal_access_token(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
