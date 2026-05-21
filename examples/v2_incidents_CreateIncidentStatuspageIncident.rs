// Create a Statuspage incident for an incident returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentStatuspageIncidentDataAttributesRequest;
use datadog_api_client::datadogV2::model::IncidentStatuspageIncidentDataRequest;
use datadog_api_client::datadogV2::model::IncidentStatuspageIncidentRequest;
use datadog_api_client::datadogV2::model::IncidentStatuspageIncidentType;

#[tokio::main]
async fn main() {
    let body = IncidentStatuspageIncidentRequest::new(IncidentStatuspageIncidentDataRequest::new(
        IncidentStatuspageIncidentDataAttributesRequest::new()
            .body(Some("We are investigating the issue.".to_string()))
            .deliver_notifications(Some(true))
            .impact(Some("major".to_string()))
            .name(Some("Service Outage".to_string()))
            .page_id("abc123".to_string())
            .status(Some("investigating".to_string())),
        IncidentStatuspageIncidentType::INCIDENT_INTEGRATIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentStatuspageIncident", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .create_incident_statuspage_incident("incident_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
