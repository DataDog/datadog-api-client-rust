// Get all SLO corrections returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_service_level_objective_corrections::ListSLOCorrectionOptionalParams;
use datadog_api_client::datadogV1::api::api_service_level_objective_corrections::ServiceLevelObjectiveCorrectionsAPI;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = ServiceLevelObjectiveCorrectionsAPI::with_config(configuration);
    let response = api
        .list_slo_correction_with_pagination(ListSLOCorrectionOptionalParams::default().limit(2));
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
