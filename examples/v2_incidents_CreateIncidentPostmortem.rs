// Create postmortem for an incident returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::CreateIncidentPostmortemOptionalParams;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentPostmortemCreateAttributes;
use datadog_api_client::datadogV2::model::IncidentPostmortemCreateData;
use datadog_api_client::datadogV2::model::IncidentPostmortemCreateRequest;
use datadog_api_client::datadogV2::model::IncidentPostmortemType;

#[tokio::main]
async fn main() {
    // there is a valid "incident" in the system
    let incident_data_id = std::env::var("INCIDENT_DATA_ID").unwrap();
    let body = IncidentPostmortemCreateRequest::new(IncidentPostmortemCreateData::new(
        IncidentPostmortemCreateAttributes::new(
            "https://app.datadoghq.com/notebook/123".to_string(),
            "Postmortem for IR-123".to_string(),
        ),
        IncidentPostmortemType::INCIDENT_POSTMORTEMS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentPostmortem", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .create_incident_postmortem(
            incident_data_id.clone(),
            body,
            CreateIncidentPostmortemOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
