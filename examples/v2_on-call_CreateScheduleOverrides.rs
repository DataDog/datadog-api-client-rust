// Create On-Call schedule overrides returns "Created" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::CreateScheduleOverridesOptionalParams;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;
use datadog_api_client::datadogV2::model::CreateOverrideRequestAttributes;
use datadog_api_client::datadogV2::model::CreateOverrideRequestData;
use datadog_api_client::datadogV2::model::CreateOverrideRequestRelationships;
use datadog_api_client::datadogV2::model::CreateOverridesRequest;
use datadog_api_client::datadogV2::model::OverrideDataType;
use datadog_api_client::datadogV2::model::OverrideRelationshipsUser;
use datadog_api_client::datadogV2::model::OverrideRelationshipsUserData;
use datadog_api_client::datadogV2::model::OverrideRelationshipsUserDataType;

#[tokio::main]
async fn main() {
    // there is a valid "schedule" in the system
    let schedule_data_id = std::env::var("SCHEDULE_DATA_ID").unwrap();

    // there is a valid "user" in the system
    let user_data_id = std::env::var("USER_DATA_ID").unwrap();
    let body = CreateOverridesRequest::new(vec![CreateOverrideRequestData::new(
        CreateOverrideRequestAttributes::new(
            DateTime::parse_from_rfc3339("2021-11-12T11:11:11+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
            DateTime::parse_from_rfc3339("2021-11-11T11:11:11+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
        ),
        OverrideDataType::OVERRIDES,
    )
    .relationships(CreateOverrideRequestRelationships::new().user(
        OverrideRelationshipsUser::new(OverrideRelationshipsUserData::new(
            user_data_id.clone(),
            OverrideRelationshipsUserDataType::USERS,
        )),
    ))]);
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .create_schedule_overrides(
            schedule_data_id.clone(),
            body,
            CreateScheduleOverridesOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
