// Create an override returns "Created" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;
use datadog_api_client::datadogV2::model::OnCallUserRelationship;
use datadog_api_client::datadogV2::model::OnCallUserRelationshipData;
use datadog_api_client::datadogV2::model::OnCallUserRelationshipType;
use datadog_api_client::datadogV2::model::OverrideCreateData;
use datadog_api_client::datadogV2::model::OverrideCreateDataAttributes;
use datadog_api_client::datadogV2::model::OverrideCreateDataRelationships;
use datadog_api_client::datadogV2::model::OverrideCreateDataType;
use datadog_api_client::datadogV2::model::OverrideRequest;

#[tokio::main]
async fn main() {
    // there is a valid "schedule" in the system
    let schedule_data_id = std::env::var("SCHEDULE_DATA_ID").unwrap();

    // there is a valid "user" in the system
    let user_data_id = std::env::var("USER_DATA_ID").unwrap();
    let body = OverrideRequest::new(vec![OverrideCreateData::new(
        OverrideCreateDataAttributes::new(
            DateTime::parse_from_rfc3339("2021-11-11T12:11:11+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
            DateTime::parse_from_rfc3339("2021-11-11T11:11:11+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
        ),
        OverrideCreateDataType::OVERRIDES,
    )
    .relationships(
        OverrideCreateDataRelationships::new().user(
            OnCallUserRelationship::new().data(
                OnCallUserRelationshipData::new()
                    .id(user_data_id.clone())
                    .type_(OnCallUserRelationshipType::USERS),
            ),
        ),
    )]);
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .create_on_call_schedule_override(schedule_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
