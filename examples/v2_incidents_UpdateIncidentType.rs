// Update an incident type returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentTypePatchData;
use datadog_api_client::datadogV2::model::IncidentTypePatchRequest;
use datadog_api_client::datadogV2::model::IncidentTypeType;
use datadog_api_client::datadogV2::model::IncidentTypeUpdateAttributes;

#[tokio::main]
async fn main() {
    let body =
        IncidentTypePatchRequest::new(
            IncidentTypePatchData::new(
                IncidentTypeUpdateAttributes::new()
                    .description(
                        "Any incidents that harm (or have the potential to) the confidentiality, integrity, or availability of our data. Note: This will notify the security team.".to_string(),
                    )
                    .is_default(true)
                    .name("Security Incident".to_string()),
                IncidentTypeType::INCIDENT_TYPES,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateIncidentType", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .update_incident_type("incident_type_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
