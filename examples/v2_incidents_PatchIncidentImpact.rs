// Update an incident impact returns "OK" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::api_incidents::PatchIncidentImpactOptionalParams;
use datadog_api_client::datadogV2::model::IncidentImpactPatchAttributes;
use datadog_api_client::datadogV2::model::IncidentImpactPatchData;
use datadog_api_client::datadogV2::model::IncidentImpactPatchRequest;
use datadog_api_client::datadogV2::model::IncidentImpactType;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = IncidentImpactPatchRequest::new(
        IncidentImpactPatchData::new(IncidentImpactType::INCIDENT_IMPACTS).attributes(
            IncidentImpactPatchAttributes::new()
                .description("Service was unavailable for external users".to_string())
                .end_at(Some(
                    DateTime::parse_from_rfc3339("2025-08-29T13:17:00+00:00")
                        .expect("Failed to parse datetime")
                        .with_timezone(&Utc),
                ))
                .fields(BTreeMap::from([(
                    "customers_impacted".to_string(),
                    Value::from("all"),
                )]))
                .start_at(
                    DateTime::parse_from_rfc3339("2025-08-28T13:17:00+00:00")
                        .expect("Failed to parse datetime")
                        .with_timezone(&Utc),
                ),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.PatchIncidentImpact", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .patch_incident_impact(
            "incident_id".to_string(),
            "impact_id".to_string(),
            body,
            PatchIncidentImpactOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
