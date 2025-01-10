// Update an incident type returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentTypePatchData;
use datadog_api_client::datadogV2::model::IncidentTypePatchRequest;
use datadog_api_client::datadogV2::model::IncidentTypeType;
use datadog_api_client::datadogV2::model::IncidentTypeUpdateAttributes;

#[tokio::main]
async fn main() {
    // there is a valid "incident_type" in the system
    let incident_type_data_id = std::env::var("INCIDENT_TYPE_DATA_ID").unwrap();
    let body = IncidentTypePatchRequest::new(IncidentTypePatchData::new(
        IncidentTypeUpdateAttributes::new().name("Security Incident-updated".to_string()),
        incident_type_data_id.clone(),
        IncidentTypeType::INCIDENT_TYPES,
    ));

    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateIncidentType", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .update_incident_type(incident_type_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
