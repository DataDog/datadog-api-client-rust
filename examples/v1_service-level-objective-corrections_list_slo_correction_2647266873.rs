// Get all SLO corrections returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_service_level_objective_corrections::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = ServiceLevelObjectiveCorrectionsAPI::with_config(configuration);
    let resp = api
        .list_slo_correction(ListSLOCorrectionOptionalParams::default().limit(2))
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}