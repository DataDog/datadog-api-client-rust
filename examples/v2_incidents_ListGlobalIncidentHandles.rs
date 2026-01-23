// List global incident handles returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::api_incidents::ListGlobalIncidentHandlesOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListGlobalIncidentHandles", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .list_global_incident_handles(ListGlobalIncidentHandlesOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
