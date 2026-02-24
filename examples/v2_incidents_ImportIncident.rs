// Import an incident returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::ImportIncidentOptionalParams;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentImportRequest;
use datadog_api_client::datadogV2::model::IncidentImportRequestAttributes;
use datadog_api_client::datadogV2::model::IncidentImportRequestData;
use datadog_api_client::datadogV2::model::IncidentImportVisibility;
use datadog_api_client::datadogV2::model::IncidentType;

#[tokio::main]
async fn main() {
    let body = IncidentImportRequest::new(IncidentImportRequestData::new(
        IncidentImportRequestAttributes::new("Example-Incident".to_string())
            .visibility(IncidentImportVisibility::ORGANIZATION),
        IncidentType::INCIDENTS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ImportIncident", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .import_incident(body, ImportIncidentOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
