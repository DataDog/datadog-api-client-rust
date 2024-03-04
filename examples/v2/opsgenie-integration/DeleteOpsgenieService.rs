// Delete a single service object returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_opsgenie_integration::OpsgenieIntegrationAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    // there is a valid "opsgenie_service" in the system
    let opsgenie_service_data_id = std::env::var("OPSGENIE_SERVICE_DATA_ID").unwrap();
    let configuration = Configuration::new();
    let api = OpsgenieIntegrationAPI::with_config(configuration);
    let resp = api.delete_opsgenie_service().await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
