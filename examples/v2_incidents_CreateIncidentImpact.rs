// Create an incident impact returns "CREATED" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::CreateIncidentImpactOptionalParams;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentImpactCreateAttributes;
use datadog_api_client::datadogV2::model::IncidentImpactCreateData;
use datadog_api_client::datadogV2::model::IncidentImpactCreateRequest;
use datadog_api_client::datadogV2::model::IncidentImpactType;

#[tokio::main]
async fn main() {
    // there is a valid "incident" in the system
    let incident_data_id = std::env::var("INCIDENT_DATA_ID").unwrap();
    let body = IncidentImpactCreateRequest::new(IncidentImpactCreateData::new(
        IncidentImpactCreateAttributes::new(
            "Outage in the us-east-1 region".to_string(),
            DateTime::parse_from_rfc3339("2025-09-12T13:50:00+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
        )
        .end_at(Some(
            DateTime::parse_from_rfc3339("2025-09-12T14:50:00+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
        )),
        IncidentImpactType::INCIDENT_IMPACTS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentImpact", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .create_incident_impact(
            incident_data_id.clone(),
            body,
            CreateIncidentImpactOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
