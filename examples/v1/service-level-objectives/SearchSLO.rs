// Search for SLOs returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_service_level_objectives::ServiceLevelObjectivesAPI;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    // there is a valid "slo" in the system
    let slo_data_0_name = std::env::var("SLO_DATA_0_NAME").unwrap();
    let configuration = Configuration::new();
    let api = ServiceLevelObjectivesAPI::with_config(configuration);
    let resp = api.search_slo().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
