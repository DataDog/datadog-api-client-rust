// Export incidents returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentSearchIncidentsExportRequest;

#[tokio::main]
async fn main() {
    let body = IncidentSearchIncidentsExportRequest::new(
        vec![
            "title".to_string(),
            "severity".to_string(),
            "state".to_string(),
        ],
        "state:active".to_string(),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ExportIncidents", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api.export_incidents(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
