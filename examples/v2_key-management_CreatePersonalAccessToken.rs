// Create a personal access token returns "Created" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_key_management::KeyManagementAPI;
use datadog_api_client::datadogV2::model::PersonalAccessTokenCreateAttributes;
use datadog_api_client::datadogV2::model::PersonalAccessTokenCreateData;
use datadog_api_client::datadogV2::model::PersonalAccessTokenCreateRequest;
use datadog_api_client::datadogV2::model::PersonalAccessTokensType;

#[tokio::main]
async fn main() {
    let body = PersonalAccessTokenCreateRequest::new(PersonalAccessTokenCreateData::new(
        PersonalAccessTokenCreateAttributes::new(
            DateTime::parse_from_rfc3339("2022-11-11T11:11:11+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
            "Example-Key-Management".to_string(),
            vec!["dashboards_read".to_string()],
        ),
        PersonalAccessTokensType::PERSONAL_ACCESS_TOKENS,
    ));
    let configuration = datadog::Configuration::new();
    let api = KeyManagementAPI::with_config(configuration);
    let resp = api.create_personal_access_token(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
