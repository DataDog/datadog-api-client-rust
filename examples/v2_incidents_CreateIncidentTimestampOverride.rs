// Create a timestamp override for an incident returns "Created" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::CreateIncidentTimestampOverrideOptionalParams;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentTimestampOverrideCreateAttributes;
use datadog_api_client::datadogV2::model::IncidentTimestampOverrideCreateData;
use datadog_api_client::datadogV2::model::IncidentTimestampOverrideCreateRequest;
use datadog_api_client::datadogV2::model::IncidentsTimestampOverridesType;
use datadog_api_client::datadogV2::model::TimestampType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body =
        IncidentTimestampOverrideCreateRequest::new(IncidentTimestampOverrideCreateData::new(
            IncidentTimestampOverrideCreateAttributes::new(
                TimestampType::CREATED,
                DateTime::parse_from_rfc3339("2024-12-29T10:00:00+00:00")
                    .expect("Failed to parse datetime")
                    .with_timezone(&Utc),
            ),
            IncidentsTimestampOverridesType::INCIDENTS_TIMESTAMP_OVERRIDES,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentTimestampOverride", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .create_incident_timestamp_override(
            Uuid::parse_str("9cecfde8-e35d-4387-8985-9b30dcb9cb1c").expect("invalid UUID"),
            body,
            CreateIncidentTimestampOverrideOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
