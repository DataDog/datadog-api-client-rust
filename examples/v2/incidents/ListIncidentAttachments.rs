// Get a list of attachments returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListIncidentAttachments", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api.list_incident_attachments().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
