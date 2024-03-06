// Get a list of incidents returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_incidents::*;

#[tokio::main]
async fn main() {
    let mut configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListIncidents", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .list_incidents(ListIncidentsOptionalParams::default().page_size(2))
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
