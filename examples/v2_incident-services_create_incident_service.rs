// Create a new incident service returns "CREATED" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_incident_services::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = IncidentServiceCreateRequest::new(
        IncidentServiceCreateData::new(IncidentServiceType::SERVICES).attributes(
            IncidentServiceCreateAttributes::new("Example-Incident-Service".to_string()),
        ),
    );
    let mut configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentService", true);
    let api = IncidentServicesAPI::with_config(configuration);
    let resp = api.create_incident_service(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
