// Create or update incident automation data returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::api_incidents::UpsertIncidentAutomationDataOptionalParams;
use datadog_api_client::datadogV2::model::IncidentAutomationDataAttributesRequest;
use datadog_api_client::datadogV2::model::IncidentAutomationDataDataRequest;
use datadog_api_client::datadogV2::model::IncidentAutomationDataRequest;
use datadog_api_client::datadogV2::model::IncidentAutomationDataType;

#[tokio::main]
async fn main() {
    let body = IncidentAutomationDataRequest::new(IncidentAutomationDataDataRequest::new(
        IncidentAutomationDataAttributesRequest::new("completed".to_string()),
        IncidentAutomationDataType::INCIDENTS_AUTOMATION_DATA,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpsertIncidentAutomationData", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .upsert_incident_automation_data(
            "incident_id".to_string(),
            "key".to_string(),
            body,
            UpsertIncidentAutomationDataOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
