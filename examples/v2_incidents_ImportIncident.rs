// Import an incident returns "CREATED" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::ImportIncidentOptionalParams;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentImportFieldAttributes;
use datadog_api_client::datadogV2::model::IncidentImportFieldAttributesSingleValue;
use datadog_api_client::datadogV2::model::IncidentImportRequest;
use datadog_api_client::datadogV2::model::IncidentImportRequestAttributes;
use datadog_api_client::datadogV2::model::IncidentImportRequestData;
use datadog_api_client::datadogV2::model::IncidentImportVisibility;
use datadog_api_client::datadogV2::model::IncidentType;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = IncidentImportRequest::new(IncidentImportRequestData::new(
        IncidentImportRequestAttributes::new("Example-Incident".to_string())
            .declared(
                DateTime::parse_from_rfc3339("2025-01-01T00:00:00+00:00")
                    .expect("Failed to parse datetime")
                    .with_timezone(&Utc),
            )
            .detected(
                DateTime::parse_from_rfc3339("2025-01-01T00:00:00+00:00")
                    .expect("Failed to parse datetime")
                    .with_timezone(&Utc),
            )
            .fields(BTreeMap::from([
                (
                    "severity".to_string(),
                    IncidentImportFieldAttributes::IncidentImportFieldAttributesSingleValue(
                        Box::new(
                            IncidentImportFieldAttributesSingleValue::new()
                                .value(Some("SEV-5".to_string())),
                        ),
                    ),
                ),
                (
                    "state".to_string(),
                    IncidentImportFieldAttributes::IncidentImportFieldAttributesSingleValue(
                        Box::new(
                            IncidentImportFieldAttributesSingleValue::new()
                                .value(Some("active".to_string())),
                        ),
                    ),
                ),
            ]))
            .visibility(IncidentImportVisibility::ORGANIZATION),
        IncidentType::INCIDENTS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ImportIncident", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .import_incident(body, ImportIncidentOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
