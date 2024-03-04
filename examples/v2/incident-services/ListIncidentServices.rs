// Get a list of all incident services returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_incident_services::IncidentServicesAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    // there is a valid "service" in the system
    let service_data_attributes_name = std::env::var("SERVICE_DATA_ATTRIBUTES_NAME").unwrap();
    let configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListIncidentServices", true);
    let api = IncidentServicesAPI::with_config(configuration);
    let resp = api.list_incident_services().await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
