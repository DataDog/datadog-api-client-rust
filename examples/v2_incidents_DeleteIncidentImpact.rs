// Delete an incident impact returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;

#[tokio::main]
async fn main() {
    // the "incident" has an "incident_impact"
    let incident_impact_data_id = std::env::var("INCIDENT_IMPACT_DATA_ID").unwrap();
    let incident_impact_data_relationships_incident_data_id =
        std::env::var("INCIDENT_IMPACT_DATA_RELATIONSHIPS_INCIDENT_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteIncidentImpact", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .delete_incident_impact(
            incident_impact_data_relationships_incident_data_id.clone(),
            incident_impact_data_id.clone(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
