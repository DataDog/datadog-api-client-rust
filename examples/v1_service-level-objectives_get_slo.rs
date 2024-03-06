// Get an SLO's details returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_service_level_objectives::*;

#[tokio::main]
async fn main() {
    // there is a valid "slo" in the system
    let slo_data_0_id = std::env::var("SLO_DATA_0_ID").unwrap();
    let configuration = Configuration::new();
    let api = ServiceLevelObjectivesAPI::with_config(configuration);
    let resp = api
        .get_slo(slo_data_0_id.clone(), GetSLOOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}