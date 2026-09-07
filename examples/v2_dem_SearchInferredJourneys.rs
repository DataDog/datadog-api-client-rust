// Search inferred DEM journeys returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dem::SearchInferredJourneysOptionalParams;
use datadog_api_client::datadogV2::api_dem::DEMAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = DEMAPI::with_config(configuration);
    let resp = api
        .search_inferred_journeys(SearchInferredJourneysOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
