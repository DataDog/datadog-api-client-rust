// Create personal access token returns "OK" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_key_management::KeyManagementAPI;
use datadog_api_client::datadogV2::model::PersonalAccessTokenCreateAttributes;
use datadog_api_client::datadogV2::model::PersonalAccessTokenCreateData;
use datadog_api_client::datadogV2::model::PersonalAccessTokenCreateRequest;
use datadog_api_client::datadogV2::model::PersonalAccessTokenType;

#[tokio::main]
async fn main() {
    let body = PersonalAccessTokenCreateRequest::new(PersonalAccessTokenCreateData::new(
        PersonalAccessTokenCreateAttributes::new(
            DateTime::parse_from_rfc3339("2025-03-15T10:30:00+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
            "Example Personal Access Token".to_string(),
            vec!["dashboards_read".to_string(), "monitors_read".to_string()],
        ),
        PersonalAccessTokenType::PERSONAL_ACCESS_TOKENS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreatePersonalAccessToken", true);
    let api = KeyManagementAPI::with_config(configuration);
    let resp = api.create_personal_access_token(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
