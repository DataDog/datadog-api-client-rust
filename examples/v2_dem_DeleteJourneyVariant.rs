// Delete a DEM journey variant returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dem::DEMAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = DEMAPI::with_config(configuration);
    let resp = api.delete_journey_variant("variant_id".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
