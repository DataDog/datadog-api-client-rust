// Update an incident timeline entry returns "OK" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentTimelineCellType;
use datadog_api_client::datadogV2::model::IncidentTimelineEntryContent;
use datadog_api_client::datadogV2::model::IncidentTimelineEntryDataAttributesRequest;
use datadog_api_client::datadogV2::model::IncidentTimelineEntryDataRequest;
use datadog_api_client::datadogV2::model::IncidentTimelineEntryRequest;
use datadog_api_client::datadogV2::model::IncidentTimelineEntryType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = IncidentTimelineEntryRequest::new(IncidentTimelineEntryDataRequest::new(
        IncidentTimelineEntryDataAttributesRequest::new(
            IncidentTimelineCellType::MARKDOWN,
            IncidentTimelineEntryContent::new().message("Investigating the issue.".to_string()),
        )
        .display_time(
            DateTime::parse_from_rfc3339("2024-01-01T00:00:00+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
        )
        .important(false),
        IncidentTimelineEntryType::INCIDENT_TIMELINE_CELLS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateIncidentTimelineEntry", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .update_incident_timeline_entry(
            "incident_id".to_string(),
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
