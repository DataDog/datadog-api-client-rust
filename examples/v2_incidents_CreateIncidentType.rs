// Create an incident type returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentTypeAttributes;
use datadog_api_client::datadogV2::model::IncidentTypeCreateData;
use datadog_api_client::datadogV2::model::IncidentTypeCreateRequest;
use datadog_api_client::datadogV2::model::IncidentTypeType;

#[tokio::main]
async fn main() {
    let body = IncidentTypeCreateRequest::new(IncidentTypeCreateData::new(IncidentTypeAttributes::new("Security Incident".to_string(), ).description("Any incidents that harm (or have the potential to) the confidentiality, integrity, or availability of our data.".to_string()).is_default(false), IncidentTypeType::INCIDENT_TYPES, ), );

    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentType", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api.create_incident_type(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
