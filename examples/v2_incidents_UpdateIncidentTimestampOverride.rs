// Update a timestamp override for an incident returns "OK" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::api_incidents::UpdateIncidentTimestampOverrideOptionalParams;
use datadog_api_client::datadogV2::model::IncidentTimestampOverridePatchAttributes;
use datadog_api_client::datadogV2::model::IncidentTimestampOverridePatchData;
use datadog_api_client::datadogV2::model::IncidentTimestampOverridePatchRequest;
use datadog_api_client::datadogV2::model::IncidentsTimestampOverridesType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = IncidentTimestampOverridePatchRequest::new(IncidentTimestampOverridePatchData::new(
        IncidentTimestampOverridePatchAttributes::new(
            DateTime::parse_from_rfc3339("2024-12-29T11:00:00+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
        ),
        IncidentsTimestampOverridesType::INCIDENTS_TIMESTAMP_OVERRIDES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateIncidentTimestampOverride", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .update_incident_timestamp_override(
            Uuid::parse_str("9cecfde8-e35d-4387-8985-9b30dcb9cb1c").expect("invalid UUID"),
            Uuid::parse_str("6f48a86f-9a39-4bcf-a76b-9a1b20188995").expect("invalid UUID"),
            body,
            UpdateIncidentTimestampOverrideOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
