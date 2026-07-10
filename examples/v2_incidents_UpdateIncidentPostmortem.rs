// Update postmortem for an incident returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::api_incidents::UpdateIncidentPostmortemOptionalParams;
use datadog_api_client::datadogV2::model::IncidentPostmortemType;
use datadog_api_client::datadogV2::model::IncidentPostmortemUpdateAttributes;
use datadog_api_client::datadogV2::model::IncidentPostmortemUpdateData;
use datadog_api_client::datadogV2::model::IncidentPostmortemUpdateRequest;
use datadog_api_client::datadogV2::model::PostmortemStatus;

#[tokio::main]
async fn main() {
    // there is a valid "postmortem" in the system
    let postmortem_data_id = std::env::var("POSTMORTEM_DATA_ID").unwrap();
    let postmortem_data_relationships_incident_data_id =
        std::env::var("POSTMORTEM_DATA_RELATIONSHIPS_INCIDENT_DATA_ID").unwrap();
    let body = IncidentPostmortemUpdateRequest::new(IncidentPostmortemUpdateData::new(
        IncidentPostmortemUpdateAttributes::new().status(PostmortemStatus::IN_REVIEW),
        postmortem_data_id.clone(),
        IncidentPostmortemType::INCIDENT_POSTMORTEMS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateIncidentPostmortem", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .update_incident_postmortem(
            postmortem_data_relationships_incident_data_id.clone(),
            body,
            UpdateIncidentPostmortemOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
