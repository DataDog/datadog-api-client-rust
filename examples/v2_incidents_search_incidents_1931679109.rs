// Search for incidents returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_incidents::*;

#[tokio::main]
async fn main() {
    let mut configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.SearchIncidents", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .search_incidents(
            "state:(active OR stable OR resolved)".to_string(),
            SearchIncidentsOptionalParams::default().page_size(2),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
