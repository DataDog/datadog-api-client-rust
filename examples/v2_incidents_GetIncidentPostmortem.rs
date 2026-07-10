// Get postmortem for an incident returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::GetIncidentPostmortemOptionalParams;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "postmortem" in the system
    let postmortem_data_relationships_incident_data_id =
        std::env::var("POSTMORTEM_DATA_RELATIONSHIPS_INCIDENT_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetIncidentPostmortem", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .get_incident_postmortem(
            postmortem_data_relationships_incident_data_id.clone(),
            GetIncidentPostmortemOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
